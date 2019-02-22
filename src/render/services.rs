use crate::render::memalloc as m;

use gfx_hal as hal;
use gfx_hal::buffer;
use gfx_hal::memory;
use gfx_hal::pso;

use hal::DescriptorPool;
use hal::Device;
use m::MemAlloc;

use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;

/// Number of consecutive frames a command buffer is estimated to be running at the maximum.
/// This is used e.g. to determine when a resource must be released after it is submitted
/// to a command buffer but not needed after that.
pub const FRAME_OVERLAP: usize = 4;

pub struct Services<B: hal::Backend> {
    device: Arc<B::Device>,
    allocator: Arc<m::Allocator<B>>,
    mem_props: hal::MemoryProperties,
}

impl<B: hal::Backend> Services<B> {
    pub fn new(device: Arc<B::Device>, mem_props: hal::MemoryProperties) -> Self {
        let heap_options = mem_props
            .memory_heaps
            .iter()
            .map(|_| m::HeapOptions {
                usage: m::HeapUsage::Whole,
                // we're doing gui, so use rather small block_size
                block_size: 128 * 1024,
            })
            .collect();
        let options = m::AllocatorOptions {
            dedicated: false,
            heap_options,
        };
        let allocator = Arc::new(m::Allocator::new(
            device.clone(),
            mem_props.clone(),
            options,
        ));
        Services {
            device,
            allocator,
            mem_props,
        }
    }

    pub fn device(&self) -> &Arc<B::Device> {
        &self.device
    }

    pub fn allocator(&self) -> &Arc<m::Allocator<B>> {
        &self.allocator
    }

    pub unsafe fn alloc_buffer(
        &self,
        usage: buffer::Usage,
        size: u64,
        options: m::AllocOptions,
        prev: Option<m::BufferAlloc<B>>,
    ) -> (m::BufferAlloc<B>, bool) {
        if let Some(pb) = prev {
            if !must_realloc(pb.size(), size) {
                return (pb, false);
            } else {
                self.allocator.free_buffer(pb);
            }
        }

        (
            self.allocator
                .allocate_buffer(usage, size, &options)
                .expect("can't allocate buffer"),
            true,
        )
    }

    pub unsafe fn create_shader_set(&self, vertex: &[u8], fragment: &[u8]) -> ShaderSet<B> {
        ShaderSet {
            vertex: self.device.create_shader_module(vertex).unwrap(),
            fragment: self.device.create_shader_module(fragment).unwrap(),
        }
    }

    pub fn destroy_shader_set(&self, set: ShaderSet<B>) {
        unsafe {
            let ShaderSet { vertex, fragment } = set;
            self.device.destroy_shader_module(vertex);
            self.device.destroy_shader_module(fragment);
        }
    }

    pub unsafe fn create_circular_desc_pool<I>(
        &self,
        max_sets: usize,
        descriptor_ranges: I,
    ) -> CircularDescriptorPool<B>
    where
        I: IntoIterator,
        I::Item: Borrow<pso::DescriptorRangeDesc>,
    {
        let num_frames = FRAME_OVERLAP;
        let pool = self
            .device
            .create_descriptor_pool(
                num_frames * max_sets,
                descriptor_ranges.into_iter().map(|d| {
                    let d = d.borrow();
                    pso::DescriptorRangeDesc {
                        ty: d.ty,
                        count: d.count * num_frames,
                    }
                }),
            )
            .unwrap();
        CircularDescriptorPool { pool, num_frames }
    }

    pub unsafe fn destroy_circular_desc_pool(&self, pool: CircularDescriptorPool<B>) {
        self.device.destroy_descriptor_pool(pool.pool);
    }

    pub fn find_mem_type(
        &self,
        requirements: memory::Requirements,
        props: memory::Properties,
    ) -> Option<usize> {
        for (i, ty) in self.mem_props.memory_types.iter().enumerate() {
            if (requirements.type_mask & (1 << i)) != 0 && ty.properties.contains(props) {
                return Some(i as _);
            }
        }
        None
    }
}

/// Checks whether a buffer or memory area should be re-allocated, considering
/// the needed size. The rule is the following:
///     `cur_size < needed_size || cur_size > 2 * needed_size`
pub fn must_realloc(cur_size: u64, needed_size: u64) -> bool {
    cur_size < needed_size || cur_size > 2 * needed_size
}

#[derive(Debug)]
pub struct ShaderSet<B: hal::Backend> {
    vertex: B::ShaderModule,
    fragment: B::ShaderModule,
}

impl<B: hal::Backend> ShaderSet<B> {
    pub fn entries<'a>(&'a self) -> pso::GraphicsShaderSet<'a, B> {
        let (vs_entry, fs_entry) = (
            pso::EntryPoint {
                entry: "main",
                module: &self.vertex,
                specialization: pso::Specialization::default(),
            },
            pso::EntryPoint {
                entry: "main",
                module: &self.fragment,
                specialization: pso::Specialization::default(),
            },
        );
        pso::GraphicsShaderSet {
            vertex: vs_entry,
            hull: None,
            domain: None,
            geometry: None,
            fragment: Some(fs_entry),
        }
    }
}

/// A circular descriptor pool mimics a descriptor pool whose descriptors
/// wraps enough actual descriptor such as to use one different descriptor for each frame
#[derive(Debug)]
pub struct CircularDescriptorPool<B: hal::Backend> {
    pool: B::DescriptorPool,
    num_frames: usize,
}

impl<B: hal::Backend> CircularDescriptorPool<B> {
    pub fn allocate<L>(&mut self, layouts: L) -> Vec<CircularDescriptorSet<B>>
    where
        L: IntoIterator,
        L::Item: Borrow<B::DescriptorSetLayout> + Clone,
    {
        use std::iter::repeat;

        //  l1 l2 l3   ==>  l1f1 l1f2   l2f1 l2f2   l3f1 l3f2

        let num_frames = self.num_frames;
        let circular_layouts = layouts.into_iter().flat_map(|l| repeat(l).take(num_frames));

        let mut dss = Vec::new();
        unsafe {
            self.pool.allocate_sets(circular_layouts, &mut dss).unwrap();
        }

        let num_cds = dss.len() / num_frames;
        let mut cds = Vec::with_capacity(num_cds);
        for _ in 0..num_cds {
            cds.push(CircularDescriptorSet {
                sets: Vec::with_capacity(num_frames),
                frame: num_frames - 1,
            });
        }

        for (i, ds) in dss.into_iter().enumerate() {
            cds[i / num_frames].sets.push(ds);
        }

        cds
    }
}

#[derive(Debug)]
pub struct CircularDescriptorSet<B: hal::Backend> {
    sets: Vec<B::DescriptorSet>,
    frame: usize,
}

impl<B: hal::Backend> CircularDescriptorSet<B> {
    pub fn rotate(&mut self) {
        self.frame += 1;
        if self.frame == self.sets.len() {
            self.frame = 0;
        }
    }
}

impl<B: hal::Backend> Deref for CircularDescriptorSet<B> {
    type Target = B::DescriptorSet;
    fn deref(&self) -> &B::DescriptorSet {
        &self.sets[self.frame]
    }
}
