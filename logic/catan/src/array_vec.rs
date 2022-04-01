use std::mem::MaybeUninit;

/// Vec with a backing array (with const size `N`) as a storage.
/// It is useful when you want vec-like semantics of pushing values to the end of the
/// vector, and be able to express arbitrary-sized sequences of data, without allocating
/// memory on the heap, but rather storing the values inline with the object. The maximum
/// size of the vec is to be known at build-time, since pushing more values than it is
/// allowed by the size (`N`) of the underlying array will panic.
pub struct ArrayVec<T, const N: usize> {
    storage: [MaybeUninit<T>; N],
    size: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        Self {
            storage: [(); N].map(|_| MaybeUninit::uninit()),
            size: 0,
        }
    }

    /// Will panic if size exceeds capacity
    pub fn push(&mut self, value: T) {
        assert!(self.size < N);
        // SAFETY: 
        //  - self.size is less than N, which means accessing the self.storage[self.size]
        //    is safe. There is no possibility for self.size to be modified concurrently
        //    since we are holding an exclusive reference to self.
        //  - .write(value) is safe since there is no way for this memory location to be
        //    initialized in other way. And since self.size is never going to be the same
        //    there is no way to initialize cell again. The initialized value is dropped
        //    correctly, since there is a correct impl of Drop
        unsafe {
            self.storage.get_unchecked_mut(self.size).write(value);
        }
        self.size += 1;
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for ArrayVec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        for value in self {
            // SAFETY: 
            //  - Value is initialized, since iterator of ArrayVec only iterates
            //    upon initialized values. No one except for us has the reference
            //    to the value, since we are requiring exclusive reference to self
            //    meaning value is safe to both read and write.
            //  - Value is properly aligned since underlying storage (array) guarantees
            //    to store properly aligned values.
            //  - Who the fuck may actually know if I violate any additional Drop invariants.
            unsafe { std::ptr::drop_in_place(value) };
        }
    }
}

impl<T: PartialEq, const N: usize> PartialEq for ArrayVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq, const N: usize> Eq for ArrayVec<T, N> {}

impl<T, const N: usize> AsRef<[T]> for ArrayVec<T, N> {
    fn as_ref(&self) -> &[T] {
        // SAFETY: 
        //  - Values in the range 0..self.size are guaranteed to be initialized.
        //  - MaybeUninit<T> is guaranteed to have the same layout as a T.
        //  - &[MaybeUninit<T>] has the same layout as &[T]
        // So it is safe to transmute values in range ..self.size to &[T]
        unsafe { std::mem::transmute(&self.storage[..self.size]) }
    }
}

impl<T, const N: usize> AsMut<[T]> for ArrayVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        // SAFETY: 
        //  - Values in the range 0..self.size are guaranteed to be initialized.
        //  - MaybeUninit<T> is guaranteed to have the same layout as a T.
        //  - &mut [MaybeUninit<T>] has the same layout as &mut [T]
        // So it is safe to transmute values in range ..self.size to &mut [T]
        unsafe { std::mem::transmute(&mut self.storage[..self.size]) }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a ArrayVec<T, N> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut ArrayVec<T, N> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().into_iter()
    }
}

impl<A, const N: usize> FromIterator<A> for ArrayVec<A, N> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut vec = ArrayVec::new();
        for value in iter {
            vec.push(value);
        }
        vec
    }
}

macro_rules! array_vec {
    ($($items: expr),*) => {{
        let mut vec = $crate::array_vec::ArrayVec::new();
        $( vec.push($items); )*
        vec
    }};
}

pub(crate) use array_vec;

// pub struct IterMut<'a, T> {
//     current: NonNull<T>,
//     end: *const T,
//     _phantom: PhantomData<&'a T>,
// }

// impl<'a, T, const N: usize> IntoIterator for &'a mut ArrayVec<T, N> {
//     type Item = &'a mut T;

//     type IntoIter = IterMut<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         let start_mut = unsafe { std::mem::transmute(self.storage.as_mut_ptr()) };
//         let start = start_mut as *const T;
//         IterMut {
//             current: unsafe { NonNull::new_unchecked(start_mut) },
//             end: unsafe { start.offset(self.size as isize) },
//             _phantom: PhantomData,
//         }
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current.as_ptr() as *const T == self.end {
//             None
//         } else {
//             let item = unsafe { self.current.as_mut() };
//             self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().offset(1)) };
//             Some(item)
//         }
//     }
// }

// pub struct Iter<'a, T> {
//     current: *const T,
//     end: *const T,
//     _phantom: PhantomData<&'a T>,
// }

// impl<'a, T, const N: usize> IntoIterator for &'a ArrayVec<T, N> {
//     type Item = &'a T;

//     type IntoIter = Iter<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         let start = unsafe { std::mem::transmute(self.storage.as_ptr()) };
//         Iter {
//             current: start,
//             end: unsafe { start.offset(self.size as isize) },
//             _phantom: PhantomData,
//         }
//     }
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current == self.end {
//             None
//         } else {
//             let item = unsafe { &*self.current };
//             self.current = unsafe { self.current.offset(1) };
//             Some(item)
//         }
//     }
// }
