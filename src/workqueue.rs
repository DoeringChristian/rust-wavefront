use common::*;
use screen_13::prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::array::Array;

#[derive(Debug)]
pub struct WorkQueue<T> {
    buf: Arc<Buffer>,
    cap: usize,
    _ty: PhantomData<T>,
}

impl<T: Copy> WorkQueue<T> {
    pub fn new(device: &Arc<Device>, cap: usize) -> Self {
        let cap = cap as u32;
        let len = 0u32;

        let size = std::mem::size_of::<u64>() * 4 + std::mem::size_of::<T>() * cap as usize;

        let mut buf = Arc::new({
            let mut buf = Buffer::create(
                device,
                BufferInfo::new_mappable(
                    size as u64,
                    vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                ),
            )
            .unwrap();
            let slice = Buffer::mapped_slice_mut(&mut buf);
            slice[0..16].copy_from_slice(bytemuck::cast_slice(&[cap, len, 0, 0]));
            buf
        });
        Self {
            buf,
            cap: cap as usize,
            _ty: PhantomData,
        }
    }

    pub fn buf(&self) -> &Arc<Buffer> {
        &self.buf
    }
    pub fn cap(&self) -> usize {
        self.cap
    }
    pub fn len(&self) -> usize {
        let slice = &Buffer::mapped_slice(&self.buf)[0..16];

        let slice = bytemuck::cast_slice(slice);

        let len: u32 = slice[1];
        len as usize
    }
    pub fn items(&self) -> &[T] {
        let slice = &Buffer::mapped_slice(&self.buf);
        let len = self.len();

        unsafe { std::slice::from_raw_parts(slice[16..].as_ptr() as *const _, len as usize) }
    }
    pub fn clear(&mut self) {
        let slice = Buffer::mapped_slice_mut(Arc::get_mut(&mut self.buf).unwrap());
        let slice: &mut [u32] = bytemuck::cast_slice_mut(&mut slice[0..16]);
        slice[1] = 0;
    }
}

pub type ItemWorkQueue<T> = WorkQueue<WorkItem<T>>;
