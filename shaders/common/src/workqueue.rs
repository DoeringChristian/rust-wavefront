use core::ops::{Index, IndexMut};

use spirv_std::arch::atomic_i_increment;
use spirv_std::RuntimeArray;

#[repr(C, align(16))]
pub struct WorkQueue<T: Copy> {
    pub cap: u32,
    pub len: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub items: RuntimeArray<T>,
}
impl<T: Copy> WorkQueue<T> {
    pub fn push(&mut self, item: T) {
        unsafe {
            let i = atomic_i_increment::<u32, 1, 0>(&mut self.len);
            assert!(i < self.cap);
            *self.items.index_mut(i as usize) = item;
        }
    }
    pub fn item(&self, idx: u32) -> &T {
        unsafe {
            assert!(idx < self.len);
            self.items.index(idx as usize)
        }
    }
    pub fn item_mut(&mut self, idx: u32) -> &mut T {
        unsafe {
            assert!(idx < self.len);
            self.items.index_mut(idx as usize)
        }
    }
}
macro_rules! index {
    ($idx:ident) => {
        impl<T: Copy> Index<$idx> for WorkQueue<T> {
            type Output = T;

            fn index(&self, index: $idx) -> &Self::Output {
                self.item(index as _)
            }
        }
        impl<T: Copy> IndexMut<$idx> for WorkQueue<T> {
            fn index_mut(&mut self, index: $idx) -> &mut Self::Output {
                self.item_mut(index as _)
            }
        }
    };
}

index!(usize);
index!(u32);
index!(u64);
