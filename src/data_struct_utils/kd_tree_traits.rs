use std::borrow::Cow;

pub trait KdTreePoint<const DIM:usize>{
    fn as_kdtree_point(&self)->Cow<[f32;DIM]>;
}

impl<const DIM:usize> KdTreePoint<DIM> for [f32;DIM]{
    fn as_kdtree_point(&self)->Cow<[f32;DIM]> {
        Cow::Borrowed(self)
    }
}