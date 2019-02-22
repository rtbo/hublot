use gfx_hal as hal;
use hal::buffer;
use hal::device;
use hal::image;
use hal::memory;

use std::fmt::Debug;
use std::ops::{Deref, Range};
use std::sync::{Arc, Mutex};

use hal::Device;

/// Define how many bytes can be used from a specific size
#[derive(Debug, Copy, Clone)]
pub enum HeapUsage {
    /// Do not use this heap
    Forbid,
    /// Can use the entire heap
    Whole,
    /// Use the heap up to the specified size
    Upto(u64),
}

/// Option to define allocation behavior for each heap of the device
#[derive(Debug, Copy, Clone)]
pub struct HeapOptions {
    /// How many bytes may be use on the heap.
    /// set to 0 to forbid use of a specific heap and to size_t.max to allow entire use
    pub usage: HeapUsage,
    /// Size of a single DeviceMemory on this heap.
    /// set to 0 to use default behavior for this heap
    pub block_size: u64,
}

impl Default for HeapOptions {
    fn default() -> Self {
        Self {
            usage: HeapUsage::Whole,
            block_size: 0,
        }
    }
}

/// Options for the creation of an allocator
#[derive(Debug, Clone)]
pub struct AllocatorOptions {
    /// if set, the allocator will alloc a new DeviceMemory for each allocation
    pub dedicated: bool,
    /// one entry per heap in the device
    pub heap_options: Vec<HeapOptions>,
}

impl Default for AllocatorOptions {
    fn default() -> Self {
        Self {
            dedicated: false,
            heap_options: Vec::new(),
        }
    }
}

/// Control of an allocation of memory
#[derive(Copy, Clone, Debug)]
pub enum AllocControl {
    /// Allocation is done from a memory pool
    /// `no_alloc` can be set to force the use of an existing `Memory`.
    /// In that case, if none is found, allocation fails with `OutOfMem`.
    Pool { no_alloc: bool },
    /// Set to force the creation of a new DeviceMemory,
    /// that will be dedicated for the allocation.
    Dedicated,
}

impl Default for AllocControl {
    fn default() -> AllocControl {
        AllocControl::Pool { no_alloc: false }
    }
}

/// Describes the usage of a memory allocation
#[derive(Debug, Copy, Clone)]
pub enum MemoryUsage {
    /// Memory will be used on device only (MemProps.deviceLocal) and having it mappable
    /// on host is not requested (although it is possible on some devices).
    /// Usage:
    ///     - Resources written and read by device, e.g. images used as attachments.
    ///     - Resources transferred from host once or infrequently and read by device multiple times,
    ///        e.g. textures, vertex bufers, uniforms etc.
    GpuOnly,
    /// Memory will be mappable on host. It usually means CPU (system) memory.
    /// Resources created for this usage may still be accessible to the device,
    /// but access to them can be slower. Guarantees to be MemProps.hostVisible and MemProps.hostCoherent.
    /// Usage:
    ///     - Staging copy of resources used as transfer source.
    CpuOnly,
    /// Memory that is both mappable on host (guarantees to be MemProps.hostVisible)
    /// and preferably fast to access by GPU. CPU reads may be uncached and very slow.
    /// Usage:
    ///     - Resources written frequently by host (dynamic), read by device.
    ///         E.g. textures, vertex buffers, uniform buffers updated every frame or every draw call.
    CpuToGpu,
    /// Memory mappable on host (guarantees to be MemProps.hostVisible) and cached.
    /// Usage:
    ///     - Resources written by device, read by host - results of some computations,
    ///          e.g. screen capture, average scene luminance for HDR tone mapping.
    ///     - Any resources read or accessed randomly on host, e.g. CPU-side copy of
    ///          vertex buffer used as source of transfer, but also used for collision detection.
    GpuToCpu,
}

/// Structure controlling an allocation of memory
#[derive(Debug, Copy, Clone)]
pub struct AllocOptions {
    /// Allocation control
    pub control: AllocControl,
    /// Intended usage. Will affect preferred_props and required_props
    pub usage: Option<MemoryUsage>,
    /// MemProps bits that are optional but are preferred to be present.
    /// Allocation will favor memory types with these bits if available, but may
    /// fallback to other memory types.
    pub preferred_props: memory::Properties,
    /// MemProps bits that must be set.
    /// Allocation will fail if it can't allocate a memory type satisfies all bits.
    pub required_props: memory::Properties,
    /// mask of memory type indices (0b0101 means indices 0 and 2) that, if not
    /// zero, will constrain `hal::memory::Requirements.type_mask`.
    pub type_index_mask: u64,
}

impl AllocOptions {
    /// Initializes an AllocOptions with usage
    pub fn for_usage(usage: MemoryUsage) -> AllocOptions {
        AllocOptions {
            control: Default::default(),
            usage: Some(usage),
            preferred_props: memory::Properties::empty(),
            required_props: memory::Properties::empty(),
            type_index_mask: 0,
        }
    }
    /// set flags to options
    pub fn with_control(mut self, control: AllocControl) -> AllocOptions {
        self.control = control;
        self
    }
    /// set preferredProps to options
    pub fn with_preferred_props(mut self, props: memory::Properties) -> AllocOptions {
        self.preferred_props = props;
        self
    }
    /// set requiredProps to options
    pub fn with_required_props(mut self, props: memory::Properties) -> AllocOptions {
        self.required_props = props;
        self
    }
    /// set type index mask to options
    pub fn with_type_index_mask(mut self, mask: u64) -> AllocOptions {
        self.type_index_mask = mask;
        self
    }
}

impl Default for AllocOptions {
    fn default() -> AllocOptions {
        AllocOptions {
            control: Default::default(),
            usage: None,
            preferred_props: memory::Properties::empty(),
            required_props: memory::Properties::empty(),
            type_index_mask: 0,
        }
    }
}

/// A memory allocation
pub trait MemAlloc<B: hal::Backend>: Debug {
    /// Access underlying device memory
    fn mem(&self) -> &B::Memory;
    /// the range of this allocation within `self.mem()`
    fn range(&self) -> Range<u64>;
    /// The size of this allocation
    fn size(&self) -> u64 {
        let r = self.range();
        r.end - r.start
    }
}

/// A Buffer memory allocation
#[derive(Debug)]
pub struct BufferAlloc<B: hal::Backend> {
    buffer: B::Buffer,
    mem: Arc<B::Memory>,
    range: Range<u64>,
    heap_idx: usize,
}

impl<B: hal::Backend> Deref for BufferAlloc<B> {
    type Target = B::Buffer;
    /// Access underlying buffer
    fn deref(&self) -> &B::Buffer {
        &self.buffer
    }
}

impl<B: hal::Backend> MemAlloc<B> for BufferAlloc<B> {
    fn mem(&self) -> &B::Memory {
        &self.mem
    }
    fn range(&self) -> Range<u64> {
        self.range.clone()
    }
}

/// An Image memory allocation
#[derive(Debug)]
pub struct ImageAlloc<B: hal::Backend> {
    image: B::Image,
    mem: Arc<B::Memory>,
    range: Range<u64>,
    heap_idx: usize,
}

impl<B: hal::Backend> Deref for ImageAlloc<B> {
    type Target = B::Image;
    /// Access underlying image
    fn deref(&self) -> &B::Image {
        &self.image
    }
}

impl<B: hal::Backend> MemAlloc<B> for ImageAlloc<B> {
    fn mem(&self) -> &B::Memory {
        &self.mem
    }
    fn range(&self) -> Range<u64> {
        self.range.clone()
    }
}

#[derive(Debug)]
pub enum Error {
    /// All compatible heaps are full.
    HeapExhausted,
    /// Error reported with `AllocControl::Pool { no_alloc: true }` if no existing block with
    /// sufficient size exists.
    NoFreeBlock,
}

#[derive(Debug)]
pub enum BufferError {
    Creation(buffer::CreationError),
    Bind(device::BindError),
    Alloc(device::AllocationError),
    Error(Error),
}

impl From<buffer::CreationError> for BufferError {
    fn from(err: buffer::CreationError) -> Self {
        BufferError::Creation(err)
    }
}

impl From<device::BindError> for BufferError {
    fn from(err: device::BindError) -> Self {
        BufferError::Bind(err)
    }
}

impl From<device::AllocationError> for BufferError {
    fn from(err: device::AllocationError) -> Self {
        BufferError::Alloc(err)
    }
}

impl From<Error> for BufferError {
    fn from(err: Error) -> Self {
        BufferError::Error(err)
    }
}

pub type BufferResult<B> = Result<BufferAlloc<B>, BufferError>;

#[derive(Debug)]
pub enum ImageError {
    Creation(image::CreationError),
    Bind(device::BindError),
    Alloc(device::AllocationError),
    Error(Error),
}

impl From<image::CreationError> for ImageError {
    fn from(err: image::CreationError) -> Self {
        ImageError::Creation(err)
    }
}

impl From<device::BindError> for ImageError {
    fn from(err: device::BindError) -> Self {
        ImageError::Bind(err)
    }
}

impl From<device::AllocationError> for ImageError {
    fn from(err: device::AllocationError) -> Self {
        ImageError::Alloc(err)
    }
}

impl From<Error> for ImageError {
    fn from(err: Error) -> Self {
        ImageError::Error(err)
    }
}

pub type ImageResult<B> = Result<ImageAlloc<B>, ImageError>;

/// A memory allocator
pub struct Allocator<B: hal::Backend> {
    device: Arc<B::Device>,
    options: AllocatorOptions,
    mem_props: hal::MemoryProperties,
    pools: Vec<Mutex<Pool<B>>>,
}

impl<B: hal::Backend> Allocator<B> {
    /// Build a new memory allocator
    pub fn new(
        device: Arc<B::Device>,
        mem_props: hal::MemoryProperties,
        options: AllocatorOptions,
    ) -> Self {
        let mut pools = Vec::with_capacity(mem_props.memory_heaps.len());

        for (i, h) in mem_props.memory_heaps.iter().enumerate() {
            let opts = if options.heap_options.len() > i {
                options.heap_options[i]
            } else {
                Default::default()
            };
            let max_bytes = match opts.usage {
                HeapUsage::Forbid => 0,
                HeapUsage::Whole => *h,
                HeapUsage::Upto(sz) => sz,
            };
            let (max_bytes, block_size) = Self::heap_block_size(*h, max_bytes, opts.block_size);
            pools.push(Mutex::new(Pool::new(
                device.clone(),
                i,
                max_bytes,
                block_size,
            )));
        }

        Self {
            device,
            options,
            mem_props,
            pools,
        }
    }

    /// Create a new buffer and bind it to a new suballocation fitting with its requirements
    pub unsafe fn allocate_buffer(
        &self,
        usage: buffer::Usage,
        size: u64,
        options: &AllocOptions,
    ) -> BufferResult<B> {
        let mut buffer = self.device.create_buffer(size, usage)?;
        let reqs = self.device.get_buffer_requirements(&buffer);
        let res = self.allocate_raw(&reqs, options)?;
        self.device
            .bind_buffer_memory(&res.mem, res.span.start, &mut buffer)?;

        Ok(BufferAlloc {
            buffer,
            mem: res.mem,
            range: res.span,
            heap_idx: res.heap_idx,
        })
    }

    /// Free a buffer allocated with this allocator
    pub unsafe fn free_buffer(&self, buffer: BufferAlloc<B>) {
        let BufferAlloc {
            buffer,
            mem,
            range,
            heap_idx,
        } = buffer;

        self.device.destroy_buffer(buffer);
        let mut pool = self.pools[heap_idx].lock().unwrap();
        pool.free(mem, range.start);
    }

    /// Create a new image and bind it to a new suballocation fittting with its requirements
    pub unsafe fn allocate_image(
        &self,
        kind: image::Kind,
        mip_levels: image::Level,
        format: hal::format::Format,
        tiling: image::Tiling,
        usage: image::Usage,
        view_caps: image::ViewCapabilities,
        options: &AllocOptions,
    ) -> ImageResult<B> {
        let mut image = self
            .device
            .create_image(kind, mip_levels, format, tiling, usage, view_caps)?;
        let reqs = self.device.get_image_requirements(&image);
        let res = self.allocate_raw(&reqs, options)?;
        self.device
            .bind_image_memory(&res.mem, res.span.start, &mut image)?;

        Ok(ImageAlloc {
            image,
            mem: res.mem,
            range: res.span,
            heap_idx: res.heap_idx,
        })
    }

    /// Free a buffer allocated with this allocator
    pub unsafe fn free_image(&self, image: ImageAlloc<B>) {
        let ImageAlloc {
            image,
            mem,
            range,
            heap_idx,
        } = image;

        self.device.destroy_image(image);
        let mut pool = self.pools[heap_idx].lock().unwrap();
        pool.free(mem, range.start);
    }
}

impl<B: hal::Backend> Allocator<B> {
    unsafe fn allocate_raw(
        &self,
        reqs: &memory::Requirements,
        options: &AllocOptions,
    ) -> CommonResult<B> {
        let control = if self.options.dedicated {
            AllocControl::Dedicated
        } else {
            options.control
        };
        let mut allowed_mask = reqs.type_mask;
        let mut common_err: Option<CommonError> = None;
        while allowed_mask != 0 {
            match self.find_mem_type_index(allowed_mask, options) {
                None => break,
                Some(idx) => {
                    match self.try_allocate(reqs, idx, control) {
                        Ok(res) => {
                            return Ok(res);
                        }
                        Err(err) => {
                            common_err = Some(err);
                        }
                    }
                    allowed_mask &= !(1 << idx);
                }
            }
        }
        if let Some(err) = common_err {
            Err(err)
        } else {
            Err(From::from(Error::HeapExhausted))
        }
    }

    unsafe fn try_allocate(
        &self,
        reqs: &memory::Requirements,
        mem_type_index: usize,
        control: AllocControl,
    ) -> CommonResult<B> {
        let mem_type = self.mem_props.memory_types[mem_type_index];
        let mut pool = self.pools[mem_type.heap_index].lock().unwrap();
        pool.allocate(mem_type_index, reqs, control)
    }

    fn find_mem_type_index(
        &self,
        allowed_index_mask: u64,
        options: &AllocOptions,
    ) -> Option<usize> {
        let allowed_index_mask = {
            let mut mask = allowed_index_mask;
            if options.type_index_mask != 0 {
                mask &= options.type_index_mask;
            }
            mask
        };
        let mut preferred = options.preferred_props;
        let mut required = options.required_props;

        match options.usage {
            None => {}
            Some(MemoryUsage::GpuOnly) => {
                preferred.insert(memory::Properties::DEVICE_LOCAL);
            }
            Some(MemoryUsage::CpuOnly) => {
                required.insert(memory::Properties::CPU_VISIBLE | memory::Properties::COHERENT);
            }
            Some(MemoryUsage::CpuToGpu) => {
                preferred.insert(memory::Properties::DEVICE_LOCAL);
                required.insert(memory::Properties::CPU_VISIBLE);
            }
            Some(MemoryUsage::GpuToCpu) => {
                preferred.insert(memory::Properties::COHERENT | memory::Properties::CPU_CACHED);
                required.insert(memory::Properties::CPU_VISIBLE);
            }
        }

        let mut index: Option<(usize, u32)> = None;

        for (i, ty) in self.mem_props.memory_types.iter().enumerate() {
            let mask = 1 << i;
            if allowed_index_mask & mask == 0 {
                continue;
            }
            let props = ty.properties;
            if !required.contains(props) {
                continue;
            }
            let score = (preferred & props).bits().count_ones();
            index = match index {
                None => Some((i, score)),
                Some((idx, sc)) => {
                    if score > sc {
                        Some((i, score))
                    } else {
                        Some((idx, sc))
                    }
                }
            };
        }

        index.map(|(idx, _)| idx)
    }

    // returns final (max_bytes, block_size)
    fn heap_block_size(heap: u64, max_bytes: u64, block_size: u64) -> (u64, u64) {
        use std::cmp::min;

        const LARGE_HEAP_THRESHOLD: u64 = 1024 * 1024 * 1024;
        const LARGE_HEAP_BLOCK_SIZE: u64 = 256 * 1024 * 1024;

        let max_bytes = min(max_bytes, heap);

        let mut block_size = block_size;
        if block_size == 0 || block_size > max_bytes {
            block_size = if heap >= LARGE_HEAP_THRESHOLD {
                LARGE_HEAP_BLOCK_SIZE
            } else {
                heap / 8
            };
        }
        if !block_size.is_power_of_two() {
            block_size = block_size.next_power_of_two() / 2;
        }
        while block_size > max_bytes {
            block_size /= 2;
        }

        (max_bytes, block_size)
    }
}

#[derive(Debug)]
struct AllocRes<B: hal::Backend> {
    mem: Arc<B::Memory>,
    span: Range<u64>,
    heap_idx: usize,
}

#[derive(Debug)]
enum CommonError {
    Alloc(device::AllocationError),
    Error(Error),
}

impl From<device::AllocationError> for CommonError {
    fn from(err: device::AllocationError) -> Self {
        CommonError::Alloc(err)
    }
}

impl From<Error> for CommonError {
    fn from(err: Error) -> Self {
        CommonError::Error(err)
    }
}

type CommonResult<B> = Result<AllocRes<B>, CommonError>;

impl From<CommonError> for BufferError {
    fn from(err: CommonError) -> Self {
        match err {
            CommonError::Alloc(err) => BufferError::Alloc(err),
            CommonError::Error(err) => BufferError::Error(err),
        }
    }
}

impl From<CommonError> for ImageError {
    fn from(err: CommonError) -> Self {
        match err {
            CommonError::Alloc(err) => ImageError::Alloc(err),
            CommonError::Error(err) => ImageError::Error(err),
        }
    }
}

/// Align the `addr` to `alignment`.
/// `alignment` must be a power of 2.
/// The following expression is true: `align_up(addr, alignment) >= addr`.
fn align_up(addr: u64, alignment: u64) -> u64 {
    debug_assert!(
        alignment.is_power_of_two(),
        "alignment must be a power of 2"
    );
    let align_mask = alignment - 1;
    if (addr & align_mask) == 0 {
        addr
    } else {
        (addr | align_mask) + 1
    }
}

/// Represent an entire heap of memory.
/// It is made of several Block, each of which is associated to a single
/// Memory object.
struct Pool<B: hal::Backend> {
    device: Arc<B::Device>,
    heap_idx: usize,
    max_bytes: u64,
    used_bytes: u64,
    block_size: u64,
    blocks: Vec<Block<B>>,
}

impl<B: hal::Backend> Pool<B> {
    fn new(device: Arc<B::Device>, heap_idx: usize, max_bytes: u64, block_size: u64) -> Self {
        Self {
            device,
            heap_idx,
            max_bytes,
            used_bytes: 0,
            block_size,
            blocks: Vec::new(),
        }
    }

    unsafe fn allocate(
        &mut self,
        mem_type_index: usize,
        reqs: &memory::Requirements,
        control: AllocControl,
    ) -> CommonResult<B> {
        match control {
            AllocControl::Pool { no_alloc } => self.allocate_pool(mem_type_index, reqs, no_alloc),
            AllocControl::Dedicated => self.new_block_alloc(mem_type_index, reqs, true),
        }
    }

    /// Free at the given block id and address within the block
    /// Panics if block or allocation within block is not found
    unsafe fn free(&mut self, mem: Arc<B::Memory>, addr: u64) {
        let idx = self
            .blocks
            .iter()
            .position(|b| Arc::ptr_eq(&mem, &b.mem))
            .unwrap();
        if self.blocks[idx].free(addr) {
            {
                let _ = self.blocks.remove(idx);
            }
            self.device.free_memory(Arc::try_unwrap(mem).unwrap());
        }
    }

    unsafe fn allocate_pool(
        &mut self,
        mem_type_index: usize,
        reqs: &memory::Requirements,
        no_alloc: bool,
    ) -> CommonResult<B> {
        for block in self
            .blocks
            .iter_mut()
            .filter(|b| b.mem_type_index == mem_type_index && !b.dedicated)
        {
            if let Some(res) = block.allocate(self.heap_idx, reqs) {
                return Ok(res);
            }
        }
        if no_alloc {
            Err(From::from(Error::NoFreeBlock))
        } else {
            self.new_block_alloc(mem_type_index, reqs, false)
        }
    }

    unsafe fn new_block_alloc(
        &mut self,
        mem_type_index: usize,
        reqs: &memory::Requirements,
        dedicated: bool,
    ) -> CommonResult<B> {
        let sz = if dedicated {
            reqs.size
        } else {
            std::cmp::max(reqs.size, self.block_size)
        };

        if self.used_bytes + sz < self.max_bytes {
            return Err(From::from(Error::HeapExhausted));
        }

        let mem = self
            .device
            .allocate_memory(From::from(mem_type_index), sz)?;

        self.used_bytes += sz;

        let blk = Block::new(Arc::new(mem), mem_type_index, sz, sz == reqs.size);
        self.blocks.push(blk);
        Ok(self
            .blocks
            .last_mut()
            .unwrap()
            .allocate(self.heap_idx, reqs)
            .unwrap())
    }
}

/// Data associated with a device memory.
/// Sub allocations are tracked with chunks
struct Block<B: hal::Backend> {
    mem: Arc<B::Memory>,
    mem_type_index: usize,
    dedicated: bool,
    // chunks are always spanning the entire block
    // when a chunk is freed, it is merged with its free neighbours
    chunks: Vec<Chunk>,
}

impl<B: hal::Backend> Block<B> {
    fn new(mem: Arc<B::Memory>, mem_type_index: usize, size: u64, dedicated: bool) -> Block<B> {
        Block {
            mem,
            mem_type_index,
            dedicated,
            chunks: vec![Chunk {
                span: 0..size,
                occupied: false,
            }],
        }
    }

    fn allocate(&mut self, heap_idx: usize, reqs: &memory::Requirements) -> Option<AllocRes<B>> {
        let mut spot: Option<(usize, Chunk)> = None;
        for (i, chunk) in self.chunks.iter_mut().filter(|c| !c.occupied).enumerate() {
            let start = align_up(chunk.span.start, reqs.alignment);
            let end = align_up(start + reqs.size, reqs.alignment);

            if end <= chunk.span.end {
                spot = Some((
                    i,
                    Chunk {
                        span: start..end,
                        occupied: true,
                    },
                ));
                break;
            }
        }

        spot.map(|(mut idx, new)| {
            let current = self.chunks[idx].clone();

            if new.span.start > current.span.start {
                let pad = Chunk {
                    span: current.span.start..new.span.start,
                    occupied: false,
                };
                self.chunks.insert(idx, pad);
                idx += 1;
            }
            if new.span.end < current.span.end {
                let pad = Chunk {
                    span: new.span.end..current.span.end,
                    occupied: false,
                };
                self.chunks.insert(idx + 1, pad);
            }

            AllocRes {
                mem: self.mem.clone(),
                span: new.span,
                heap_idx,
            }
        })
    }

    /// Returns true if there is no more occupied chunks after free.
    /// Panics if no chunk is found at addr.
    fn free(&mut self, addr: u64) -> bool {
        let mut idx = self
            .chunks
            .iter()
            .position(|c| c.span.start == addr)
            .expect("memalloc::Block::free could not find addr");

        debug_assert!(self.chunks[idx].occupied == true);
        self.chunks[idx].occupied = false;

        if idx > 0 && !self.chunks[idx - 1].occupied {
            // front merge
            let freed = self.chunks.remove(idx);
            self.chunks[idx - 1].span.end = freed.span.end;
            idx -= 1;
        }

        if idx < self.chunks.len() && !self.chunks[idx + 1].occupied {
            // back merge
            let freed = self.chunks.remove(idx);
            self.chunks[idx + 1].span.start = freed.span.start;
        }

        // chunk(s) remain
        debug_assert!(self.chunks.len() > 0);
        // if one remains, it must be free
        debug_assert!(self.chunks.len() > 1 || !self.chunks[0].occupied);

        self.chunks.len() == 1
    }
}

/// Tracks a suballocation in the device memory.
#[derive(Debug, Clone)]
struct Chunk {
    span: Range<u64>,
    occupied: bool,
}
