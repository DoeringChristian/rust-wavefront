use crevice::std140::{AsStd140, Std140, WriteStd140};
use screen_13::prelude::*;
use std::any::type_name;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Deref;
use std::sync::Arc;

/// Gives the number of bytes needed to make `offset` be aligned to `alignment`.
pub const fn align_offset(offset: usize, alignment: usize) -> usize {
    if alignment == 0 || offset % alignment == 0 {
        0
    } else {
        alignment - offset % alignment
    }
}

pub struct Array<T> {
    pub buf: Arc<Buffer>,
    count: usize,
    stride: usize,
    _ty: PhantomData<T>,
}
impl<T> Array<T> {
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline]
    pub fn stride(&self) -> usize {
        self.stride
    }
}

impl Array<u8> {
    pub fn from_slice_u8(device: &Arc<Device>, usage: vk::BufferUsageFlags, data: &[u8]) -> Self {
        let buf = Arc::new(Buffer::create_from_slice(device, usage, &data).unwrap());
        Self {
            buf,
            stride: size_of::<u8>(),
            count: data.len(),
            _ty: PhantomData,
        }
    }
}

impl<T: Sized + Copy> Array<T> {
    pub fn storage(device: &Arc<Device>, data: &[T]) -> Self {
        Self::from_slice(device, vk::BufferUsageFlags::STORAGE_BUFFER, data)
    }
    pub fn uninitialized(device: &Arc<Device>, usage: vk::BufferUsageFlags, count: usize) -> Self {
        let stride = std::mem::size_of::<T>();

        let buf = Arc::new(
            Buffer::create(device, BufferInfo::new((stride * count) as _, usage)).unwrap(),
        );
        Self {
            buf,
            stride,
            count,
            _ty: PhantomData,
        }
    }
    pub fn copy_from_slice(
        &self,
        device: &Arc<Device>,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
        data: &[T],
    ) {
        assert!(data.len() == self.count());
        let size = self.stride() * data.len();
        let mut staging_buf = cache
            .lease(BufferInfo::new_mappable(
                size as _,
                vk::BufferUsageFlags::TRANSFER_SRC,
            ))
            .unwrap();
        let slice = Buffer::mapped_slice_mut(staging_buf.as_mut());
        unsafe {
            slice.copy_from_slice(std::slice::from_raw_parts(data.as_ptr() as *const _, size));
        }

        let buf_node = rgraph.bind_node(&self.buf);
        let staging_node = rgraph.bind_node(staging_buf);
        rgraph.copy_buffer(staging_node, buf_node);
    }
    pub fn from_slice_staging(
        device: &Arc<Device>,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> Self {
        let stride = size_of::<T>();
        let size = stride * data.len();
        let mut staging_buf = cache
            .lease(BufferInfo::new_mappable(
                size as _,
                vk::BufferUsageFlags::TRANSFER_SRC,
            ))
            .unwrap();

        let slice = Buffer::mapped_slice_mut(staging_buf.as_mut());
        unsafe {
            slice.copy_from_slice(std::slice::from_raw_parts(data.as_ptr() as *const _, size));
        }

        let buf = Arc::new(
            Buffer::create(
                device,
                BufferInfo::new(size as _, vk::BufferUsageFlags::TRANSFER_DST | usage),
            )
            .unwrap(),
        );

        let buf_node = rgraph.bind_node(&buf);
        let staging_node = rgraph.bind_node(staging_buf);
        rgraph.copy_buffer(staging_node, buf_node);

        Self {
            buf,
            stride,
            count: data.len(),
            _ty: PhantomData,
        }
    }
    pub fn from_slice(device: &Arc<Device>, usage: vk::BufferUsageFlags, data: &[T]) -> Self {
        let stride = std::mem::size_of::<T>();
        let size = stride * data.len();
        // println!("{}", type_name::<T>());
        // println!("{stride}");

        let buf = Arc::new({
            let buf = Buffer::create_from_slice(device, usage, unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const _, size)
            })
            .unwrap();
            buf
        });

        Self {
            buf,
            stride,
            count: data.len(),
            _ty: PhantomData,
        }
    }
    pub fn buf(&self) -> &Arc<Buffer> {
        &self.buf
    }
}
impl<T> Deref for Array<T> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}
