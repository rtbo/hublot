use crate::geom;
use crate::render::frame::RectNode;

pub mod col {

    use super::RectNode;
    use super::{build_vertices, round_indices, sharp_indices};

    use crate::color;
    use crate::render::memalloc as m;
    use crate::render::services::{self, Services};
    use crate::render::Context;
    use crate::render::NodeRenderer;
    use crate::transform;
    use crate::Paint;
    use crate::Transform;

    use m::MemAlloc;

    use gfx_hal::buffer;
    use gfx_hal::command::CommandBuffer;
    use gfx_hal::format as f;
    use gfx_hal::pass;
    use gfx_hal::pso;
    use gfx_hal::Device as DevTrait;
    use gfx_hal::Primitive;

    use std::mem;
    use std::sync::Arc;

    pub struct RectRenderer<B: hal::Backend> {
        services: Arc<Services<B>>,
        ds_layout: B::DescriptorSetLayout,
        ds_pool: services::CircularDescriptorPool<B>,
        ds: services::CircularDescriptorSet<B>,
        pipeline_layout: B::PipelineLayout,
        pipeline: B::GraphicsPipeline,

        index_buffer: m::BufferAlloc<B>,
        sharp_ind_len: usize,
        round_ind_len: usize,

        vertex_buffer: Option<m::BufferAlloc<B>>,
        vertex_cursor: u64,
        vertex_map: Option<m::Map>,

        locals_buffer: Option<m::BufferAlloc<B>>,
        vs_locals_cursor: u64,
        fs_locals_cursor: u64,
        locals_map: Option<m::Map>,
    }

    #[derive(Debug, Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        edge: [f32; 3],
    }

    impl From<([f32; 2], [f32; 3])> for Vertex {
        fn from(pos_edge: ([f32; 2], [f32; 3])) -> Vertex {
            Vertex {
                position: [pos_edge.0[0], pos_edge.0[1], 0f32],
                edge: pos_edge.1,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct VsLocals {
        model: [f32; 16],
        view_proj: [f32; 16],
    }

    #[derive(Debug, Copy, Clone)]
    struct ColStop {
        color: [f32; 4],
        position: f32,
        padding: [f32; 3],
    }

    impl ColStop {
        fn new(color: [f32; 4], position: f32) -> ColStop {
            ColStop {
                color,
                position,
                padding: [0f32; 3],
            }
        }
    }

    impl Default for ColStop {
        fn default() -> ColStop {
            ColStop {
                color: [0f32; 4],
                position: 0f32,
                padding: [0f32; 3],
            }
        }
    }

    const MAX_STOPS: usize = 4;

    #[derive(Debug, Copy, Clone)]
    struct FsLocals {
        stroke_col: [f32; 4],
        stroke_width: f32,
        num_stops: u32,
        stops: [ColStop; MAX_STOPS],
    }

    impl<B: hal::Backend> RectRenderer<B> {
        pub fn new(services: Arc<Services<B>>, render_pass: &B::RenderPass) -> RectRenderer<B> {
            let ds_layout = unsafe {
                services.device().create_descriptor_set_layout(
                    &[
                        pso::DescriptorSetLayoutBinding {
                            binding: 0,
                            ty: pso::DescriptorType::UniformBufferDynamic,
                            count: 1,
                            stage_flags: pso::ShaderStageFlags::VERTEX,
                            immutable_samplers: false,
                        },
                        pso::DescriptorSetLayoutBinding {
                            binding: 1,
                            ty: pso::DescriptorType::UniformBufferDynamic,
                            count: 1,
                            stage_flags: pso::ShaderStageFlags::FRAGMENT,
                            immutable_samplers: false,
                        },
                    ],
                    &[],
                )
            }
            .expect("Can't create descriptor set layout");

            let pipeline_layout = unsafe {
                services.device().create_pipeline_layout(
                    std::iter::once(&ds_layout),
                    &[(pso::ShaderStageFlags::VERTEX, 0..8)],
                )
            }
            .expect("Can't create pipeline layout");

            let pipeline = {
                let shader_set = unsafe {
                    services.create_shader_set(
                        include_bytes!("shaders/rectcol.vert.spv"),
                        include_bytes!("shaders/rectcol.frag.spv"),
                    )
                };

                let pipeline = {
                    let shader_entries = shader_set.entries();

                    let subpass = pass::Subpass {
                        index: 0,
                        main_pass: render_pass,
                    };

                    let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                        shader_entries,
                        Primitive::TriangleList,
                        pso::Rasterizer::FILL,
                        &pipeline_layout,
                        subpass,
                    );
                    pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                        pso::ColorMask::ALL,
                        pso::BlendState::ALPHA,
                    ));
                    pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                        binding: 0,
                        stride: std::mem::size_of::<Vertex>() as u32,
                        rate: 0,
                    });

                    pipeline_desc.attributes.push(pso::AttributeDesc {
                        location: 0,
                        binding: 0,
                        element: pso::Element {
                            format: <[f32; 3] as f::AsFormat>::SELF,
                            offset: 0,
                        },
                    });
                    pipeline_desc.attributes.push(pso::AttributeDesc {
                        location: 1,
                        binding: 0,
                        element: pso::Element {
                            format: <[f32; 3] as f::AsFormat>::SELF,
                            offset: 12,
                        },
                    });

                    unsafe {
                        services
                            .device()
                            .create_graphics_pipeline(&pipeline_desc, None)
                    }
                };

                services.destroy_shader_set(shader_set);
                pipeline.expect("Can't create graphics pipeline")
            };

            let (indices, sharp_ind_len, round_ind_len) = {
                let mut sharp_indices = sharp_indices();
                let sharp_len = sharp_indices.len();
                let mut round_indices = round_indices();
                let round_len = round_indices.len();

                sharp_indices.append(&mut round_indices);

                (sharp_indices, sharp_len, round_len)
            };

            let index_buffer = unsafe {
                services
                    .allocator()
                    .allocate_buffer(
                        buffer::Usage::INDEX,
                        (indices.len() * mem::size_of::<u16>()) as _,
                        &Default::default(),
                    )
                    .unwrap()
            };

            let mut ds_pool = unsafe {
                services.create_circular_desc_pool(
                    1,
                    &[pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::UniformBufferDynamic,
                        count: 2,
                    }],
                )
            };

            let ds = ds_pool.allocate(Some(&ds_layout)).remove(0);

            RectRenderer {
                services,
                ds_layout,
                ds_pool,
                ds,
                pipeline_layout,
                pipeline,

                index_buffer,
                sharp_ind_len,
                round_ind_len,

                vertex_buffer: None,
                vertex_cursor: 0,
                vertex_map: None,

                locals_buffer: None,
                vs_locals_cursor: 0,
                fs_locals_cursor: 0,
                locals_map: None,
            }
        }
    }

    impl<B: hal::Backend> NodeRenderer<B> for RectRenderer<B> {
        type Node = RectNode;

        fn prerender(&mut self, node: &mut Self::Node) {
            self.vs_locals_cursor += mem::size_of::<VsLocals>() as u64;
            self.fs_locals_cursor += mem::size_of::<FsLocals>() as u64;
            self.vertex_cursor += if node.radius > 0f32 {
                40 * mem::size_of::<Vertex>() as u64
            } else {
                16 * mem::size_of::<Vertex>() as u64
            };
        }

        fn prerender_end(&mut self) {
            let locals_size = self.vs_locals_cursor + self.fs_locals_cursor;

            let (locals_buffer, locals_change) = unsafe {
                self.services.alloc_buffer(
                    buffer::Usage::UNIFORM,
                    locals_size as _,
                    m::AllocOptions::for_usage(m::MemoryUsage::CpuToGpu),
                    self.locals_buffer.take(),
                )
            };
            self.locals_buffer = Some(locals_buffer);

            let (vertex_buffer, _) = unsafe {
                self.services.alloc_buffer(
                    buffer::Usage::VERTEX,
                    self.vertex_cursor as _,
                    m::AllocOptions::for_usage(m::MemoryUsage::CpuToGpu),
                    self.vertex_buffer.take(),
                )
            };
            self.locals_buffer = Some(vertex_buffer);

            self.fs_locals_cursor = self.vs_locals_cursor;
            self.vs_locals_cursor = 0;
            self.vertex_cursor = 0;

            if locals_change {
                self.ds.rotate();
                unsafe {
                    self.services.device().write_descriptor_sets(vec![
                        pso::DescriptorSetWrite {
                            set: &*self.ds,
                            binding: 0,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Buffer(
                                &**self.locals_buffer.as_ref().unwrap(),
                                Some(0)..Some(mem::size_of::<VsLocals>() as u64),
                            )),
                        },
                        pso::DescriptorSetWrite {
                            set: &*self.ds,
                            binding: 1,
                            array_offset: 0,
                            descriptors: Some(pso::Descriptor::Buffer(
                                &**self.locals_buffer.as_ref().unwrap(),
                                Some(0)..Some(mem::size_of::<VsLocals>() as u64),
                            )),
                        },
                    ]);
                }
            }

            unsafe {
                self.vertex_map = Some(
                    self.vertex_buffer
                        .as_ref()
                        .unwrap()
                        .map(..)
                        .expect("could not map buffer"),
                );
                self.locals_map = Some(
                    self.locals_buffer
                        .as_ref()
                        .unwrap()
                        .map(..)
                        .expect("could not map buffer"),
                );
            }
        }

        fn render<'a>(
            &mut self,
            node: &mut Self::Node,
            context: &Context,
            model: &Transform,
            encoder: &mut hal::command::RenderPassInlineEncoder<'a, B>,
        ) {
            let vs_locals = VsLocals {
                model: model.to_4x4_col_major(),
                view_proj: context.view_proj.to_4x4_col_major(),
            };
            let fs_locals = {
                let stroke = match &node.border {
                    Some(stroke) => stroke,
                    None => &(color::BLACK, 0f32),
                };
                let mut locals = FsLocals {
                    stroke_col: From::from(stroke.0),
                    stroke_width: stroke.1,
                    num_stops: 0,
                    stops: [Default::default(); MAX_STOPS],
                };
                match &node.paint {
                    Paint::Solid(col) => {
                        locals.num_stops = 1;
                        locals.stops[0] = ColStop::new(From::from(*col), 0f32);
                    }
                    Paint::LinearGradient(stops, _) => {
                        let num_stops = std::cmp::min(stops.len(), MAX_STOPS);
                        for i in 0..num_stops {
                            locals.stops[i] = ColStop::new(From::from(stops[i].1), stops[i].0);
                        }
                        locals.num_stops = num_stops as _;
                    }
                }
                locals
            };
            if let Some(map) = self.locals_map.as_mut() {
                {
                    let view = map.view_mut::<VsLocals>(self.vs_locals_cursor, 1);
                    view[0] = vs_locals;
                }
                {
                    let offset = self.fs_locals_cursor;
                    let view = map.view_mut::<FsLocals>(offset, 1);
                    view[0] = fs_locals;
                }
            } else {
                panic!();
            }

            let num_verts = match self.vertex_map.as_mut() {
                Some(map) => {
                    let len = if node.radius >= 0f32 { 40 } else { 16 };
                    let view = map.view_mut::<Vertex>(self.vertex_cursor, len);
                    build_vertices(&node, view);
                    len
                }
                _ => panic!(),
            };

            let ds_offsets = [self.vs_locals_cursor as u32, self.fs_locals_cursor as u32];
            let (ind_offset, ind_len) = if node.radius > 0f32 {
                (self.sharp_ind_len * mem::size_of::<u16>(), self.round_ind_len as u32)
            } else {
                (0, self.sharp_ind_len as u32)
            };

            unsafe {
                encoder.bind_graphics_pipeline(&self.pipeline);
                encoder.bind_graphics_descriptor_sets(
                    &self.pipeline_layout,
                    0,
                    Some(&*self.ds),
                    &ds_offsets,
                );
                encoder.bind_vertex_buffers(
                    0,
                    Some((
                        &**self.vertex_buffer.as_ref().unwrap(),
                        self.vertex_cursor as _,
                    )),
                );
                encoder.bind_index_buffer(buffer::IndexBufferView {
                    buffer: &*self.index_buffer,
                    offset: ind_offset as _,
                    index_type: hal::IndexType::U16,
                });
                encoder.draw_indexed(0..ind_len, 0, 0..1);
            }

            self.vs_locals_cursor += mem::size_of::<VsLocals>() as u64;
            self.fs_locals_cursor += mem::size_of::<FsLocals>() as u64;
            self.vertex_cursor += num_verts as u64 * mem::size_of::<Vertex>() as u64;
        }

        fn post_render(&mut self) {
            unsafe {
                self.vertex_buffer
                    .as_ref()
                    .unwrap()
                    .unmap(self.vertex_map.take().unwrap());
                self.locals_buffer
                    .as_ref()
                    .unwrap()
                    .unmap(self.locals_map.take().unwrap());
            }
        }
    }

    fn set_gpos(node: &RectNode, verts: &mut [Vertex]) {
        if let Paint::LinearGradient(_, dir) = node.paint {
            // angle zero is defined to top (-Y)
            // angle 90deg is defined to right (+X)
            let r = node.rect;
            let angle = dir.compute_angle(r.size());
            let c = r.center();
            let ca = angle.cos();
            let sa = angle.sin();

            // unit vec along gradient line
            let u = [sa, -ca];

            // signed distance from center along the gradient line
            let ortho_proj_dist = |p| {
                transform::dot(From::from(p-c), u)
            };

            let tl = ortho_proj_dist(r.top_left());
            let tr = ortho_proj_dist(r.top_right());
            let br = ortho_proj_dist(r.bottom_right());
            let bl = ortho_proj_dist(r.bottom_left());

            let fact = 0.5f32 / tl.max(tr.max(br.max(bl)));

            foreach (ref v; verts) {
                v.gpos = fact * orthoProjDist(v.vpos) + 0.5f;
            }
        }
    }
}

#[rustfmt::skip]
fn sharp_indices() -> Vec<u16> {
    vec![
        0, 1, 2, 2, 1, 3,
        4, 5, 6, 6, 5, 7,
        8, 9, 10, 10, 9, 11,
        12, 13, 14, 14, 13, 15,
    ]
}

#[rustfmt::skip]
fn round_indices() -> Vec<u16> {
    let mut inds = Vec::with_capacity(6 * 4 + 12 * 4);
    inds.append(&mut vec![0, 1, 2, 0, 2, 3]);
    inds.append(&mut vec![4, 5, 6, 4, 6, 7]);
    inds.append(&mut vec![8, 9, 10, 8, 10, 11]);
    inds.append(&mut vec![12, 13, 14, 12, 14, 15]);
    let mut add_indices = |start: u16| {
        inds.append(&mut vec![
            start + 0, start + 1, start + 4,
            start + 0, start + 4, start + 2,
            start + 4, start + 1, start + 5,
            start + 5, start + 1, start + 3,
        ]);
    };
    add_indices(16);
    add_indices(22);
    add_indices(28);
    add_indices(34);
    return inds;
}

fn build_vertices<V: From<([f32; 2], [f32; 3])>>(node: &RectNode, verts: &mut [V]) -> u64 {
    let r = node.rect;
    let hm = r.width.min(r.height) / 2f32;
    let hw = match node.border {
        Some((_, w)) => w / 2f32,
        _ => 0f32,
    };

    // inner rect
    let ir = r - geom::Margins::from(hm);
    // extent rect
    let er = r + geom::Margins::from(hw);

    if node.radius > 0f32 {
        let rd = node.radius.min(hm);
        if rd != node.radius {
            warn!(target: "hublot", "Specified radius is too big for the rect");
        }
        // top left corner
        let tl_edge = [r.left() + rd, r.top() + rd, rd];
        verts[0] = From::from(([er.left(), er.top()], tl_edge));
        verts[1] = From::from(([r.left() + rd, er.top()], tl_edge));
        verts[2] = From::from(([r.left() + rd, r.top() + rd], tl_edge));
        verts[3] = From::from(([er.left(), r.top() + rd], tl_edge));
        // top right corner
        let tr_edge = [r.right() - rd, r.top() + rd, rd];
        verts[4] = From::from(([r.right() - rd, er.top()], tr_edge));
        verts[5] = From::from(([er.right(), er.top()], tr_edge));
        verts[6] = From::from(([er.right(), r.top() + rd], tr_edge));
        verts[7] = From::from(([r.right() - rd, r.top() + rd], tr_edge));
        // bottom right corner
        let br_edge = [r.right() - rd, r.bottom() - rd, rd];
        verts[8] = From::from(([r.right() - rd, r.bottom() - rd], br_edge));
        verts[9] = From::from(([er.right(), r.bottom() - rd], br_edge));
        verts[10] = From::from(([er.right(), er.bottom()], br_edge));
        verts[11] = From::from(([r.right() - rd, er.bottom()], br_edge));
        // bottom left corner
        let bl_edge = [r.left() + rd, r.bottom() - rd, rd];
        verts[12] = From::from(([er.left(), r.bottom() - rd], bl_edge));
        verts[13] = From::from(([r.left() + rd, r.bottom() - rd], bl_edge));
        verts[14] = From::from(([r.left() + rd, er.bottom()], bl_edge));
        verts[15] = From::from(([er.left(), er.bottom()], bl_edge));

        // top side
        verts[16] = From::from(([r.left() + rd, er.top()], [r.left() + rd, ir.top(), hm]));
        verts[17] = From::from(([r.right() - rd, er.top()], [r.right() - rd, ir.top(), hm]));
        verts[18] = From::from(([r.left() + rd, r.top() + rd], [r.left() + rd, ir.top(), hm]));
        verts[19] = From::from((
            [r.right() - rd, r.top() + rd],
            [r.right() - rd, ir.top(), hm],
        ));
        verts[20] = From::from(([ir.left(), ir.top()], [ir.left(), ir.top(), hm]));
        verts[21] = From::from(([ir.right(), ir.top()], [ir.right(), ir.top(), hm]));
        // right side
        verts[22] = From::from(([er.right(), r.top() + rd], [ir.right(), r.top() + rd, hm]));
        verts[23] = From::from((
            [er.right(), r.bottom() - rd],
            [ir.right(), r.bottom() - rd, hm],
        ));
        verts[24] = From::from((
            [r.right() - rd, r.top() + rd],
            [ir.right(), r.top() + rd, hm],
        ));
        verts[25] = From::from((
            [r.right() - rd, r.bottom() - rd],
            [ir.right(), r.bottom() - rd, hm],
        ));
        verts[26] = From::from(([ir.right(), ir.top()], [ir.right(), ir.top(), hm]));
        verts[27] = From::from(([ir.right(), ir.bottom()], [ir.right(), ir.bottom(), hm]));
        // bottom side
        verts[28] = From::from((
            [r.right() - rd, er.bottom()],
            [r.right() - rd, ir.bottom(), hm],
        ));
        verts[29] = From::from((
            [r.left() + rd, er.bottom()],
            [r.left() + rd, ir.bottom(), hm],
        ));
        verts[30] = From::from((
            [r.right() - rd, r.bottom() - rd],
            [r.right() - rd, ir.bottom(), hm],
        ));
        verts[31] = From::from((
            [r.left() + rd, r.bottom() - rd],
            [r.left() + rd, ir.bottom(), hm],
        ));
        verts[32] = From::from(([ir.right(), ir.bottom()], [ir.right(), ir.bottom(), hm]));
        verts[33] = From::from(([ir.left(), ir.bottom()], [ir.left(), ir.bottom(), hm]));
        // left side
        verts[34] = From::from((
            [er.left(), r.bottom() - rd],
            [ir.left(), r.bottom() - rd, hm],
        ));
        verts[35] = From::from(([er.left(), r.top() + rd], [ir.left(), r.top() + rd, hm]));
        verts[36] = From::from((
            [r.left() + rd, r.bottom() - rd],
            [ir.left(), r.bottom() - rd, hm],
        ));
        verts[37] = From::from(([r.left() + rd, r.top() + rd], [ir.left(), r.top() + rd, hm]));
        verts[38] = From::from(([ir.left(), ir.bottom()], [ir.left(), ir.bottom(), hm]));
        verts[39] = From::from(([ir.left(), ir.top()], [ir.left(), ir.top(), hm]));

        40
    } else {
        // top side
        verts[0] = From::from(([er.left(), er.top()], [er.left(), ir.top(), hm]));
        verts[1] = From::from(([er.right(), er.top()], [er.right(), ir.top(), hm]));
        verts[2] = From::from(([ir.left(), ir.top()], [ir.left(), ir.top(), hm]));
        verts[3] = From::from(([ir.right(), ir.top()], [ir.right(), ir.top(), hm]));
        // right side
        verts[4] = From::from(([er.right(), er.top()], [ir.right(), er.top(), hm]));
        verts[5] = From::from(([er.right(), er.bottom()], [ir.right(), er.bottom(), hm]));
        verts[6] = From::from(([ir.right(), ir.top()], [ir.right(), ir.top(), hm]));
        verts[7] = From::from(([ir.right(), ir.bottom()], [ir.right(), ir.bottom(), hm]));
        // bottom side
        verts[8] = From::from(([er.right(), er.bottom()], [er.right(), ir.bottom(), hm]));
        verts[9] = From::from(([er.left(), er.bottom()], [er.left(), ir.bottom(), hm]));
        verts[10] = From::from(([ir.right(), ir.bottom()], [ir.right(), ir.bottom(), hm]));
        verts[11] = From::from(([ir.left(), ir.bottom()], [ir.left(), ir.bottom(), hm]));
        // left side
        verts[12] = From::from(([er.left(), er.bottom()], [ir.left(), er.bottom(), hm]));
        verts[13] = From::from(([er.left(), er.top()], [ir.left(), er.top(), hm]));
        verts[14] = From::from(([ir.left(), ir.bottom()], [ir.left(), ir.bottom(), hm]));
        verts[15] = From::from(([ir.left(), ir.top()], [ir.left(), ir.top(), hm]));

        16
    }
}
