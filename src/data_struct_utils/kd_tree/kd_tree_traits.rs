use std::borrow::Cow;

pub trait KdTreePoint<const DIM:usize>{
    fn as_kdtree_point(&self)->Cow<[f64;DIM]>;
}

impl<const DIM:usize> KdTreePoint<DIM> for [f64;DIM]{
    fn as_kdtree_point(&self)->Cow<[f64;DIM]> {
        Cow::Borrowed(self)
    }
}

impl KdTreePoint<1> for f64{
    fn as_kdtree_point(&self)->Cow<[f64;1]> {
        Cow::Owned([*self])
    }
}

impl KdTreePoint<2> for (f64,f64){
    fn as_kdtree_point(&self)->Cow<[f64;2]> {
        Cow::Owned([self.0,self.1])
    }
}

impl KdTreePoint<3> for (f64,f64,f64){
    fn as_kdtree_point(&self)->Cow<[f64;3]> {
        Cow::Owned([self.0,self.1,self.2])
    }
}

impl KdTreePoint<4> for (f64,f64,f64,f64){
    fn as_kdtree_point(&self)->Cow<[f64;4]> {
        Cow::Owned([self.0,self.1,self.2,self.3])
    }
}