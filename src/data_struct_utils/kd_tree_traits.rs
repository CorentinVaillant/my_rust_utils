use std::borrow::Cow;

pub trait KdTreePoint<const DIM:usize>{
    fn as_kdtree_point(&self)->Cow<[f64;DIM]>;
}

impl<const DIM:usize> KdTreePoint<DIM> for [f64;DIM]{
    fn as_kdtree_point(&self)->Cow<[f64;DIM]> {
        Cow::Borrowed(self)
    }
}