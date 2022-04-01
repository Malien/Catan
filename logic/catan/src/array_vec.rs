use enum_map::MaybeUninit;

#[derive(Debug)]
pub struct ArrayVec<T, const N: usize>{
    storage: [MaybeUninit<T>; N],
    size: usize
}