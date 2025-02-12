pub mod kd_tree_traits;

#[cfg(test)]
pub mod tests;

use core::f64;
use std::rc::Rc;

pub use kd_tree_traits::KdTreePoint;

///Node for the KdTree
#[derive(Debug,Clone)]
struct Node<const DIM: usize> {
    point: Point<DIM>,
    left: Option<Rc<Self>>,
    right: Option<Rc<Self>>,
}

///Structure that represent a point for a KdTree
#[derive(Debug, Clone,Copy)]
pub(crate)struct Point<const DIM: usize> {
    pub(crate) position: [f64; DIM],
    index:usize
}

///A Kdtree
/// A tree that partionate a K dimensional space
#[derive(Debug,Clone)]
pub struct KdTree<const DIM: usize,POINT: KdTreePoint<DIM>> {
    base_node: Option<Rc<Node<DIM>>>,

    points : Vec<POINT>
}

impl<const DIM: usize, POINT:KdTreePoint<DIM>> From<Vec<POINT>> for KdTree<DIM,POINT> {
    fn from(value: Vec<POINT>) -> Self {
        let mut indices = (0..value.len()).collect::<Vec<_>>();       

        Self {
            base_node: Node::<DIM>::construct_kdtree(&value,&mut indices.as_mut_slice(), 0),
            points : value
        }
    }
}

impl<const DIM: usize> Point<DIM> {
    fn squared_distance(&self, other: &[f64;DIM]) -> f64 {
        self.position
            .iter()
            .zip(other.iter())
            .fold(0., |acc, (x, y)| acc + (x - y) * (x - y))
    }
}

impl<const DIM: usize> Node<DIM> {
    fn nearest<'a>(
        &'a self,
        target: &[f64;DIM],
        depth: usize,
        best: Option<&'a Node<DIM>>,
    ) -> Option<&'a Self> {
        let point = &self.point;

        let best = match best {
            Some(best) if best.point.squared_distance(target) <= point.squared_distance(target) => best,
            _ => self,
        };
    

        let axis = depth % DIM;

        let (next, opposite_branch) = if target[axis] < point.position[axis] {
            (
                (self.left).as_ref().map(|n| n.as_ref()),
                (self.right).as_ref().map(|n| n.as_ref()),
            )
        } else {
            (
                (self.right).as_ref().map(|n| n.as_ref()),
                (self.left).as_ref().map(|n| n.as_ref()),
            )
        };

        let candidate = next.and_then(|n| n.nearest(target, depth + 1, Some(best)));
        let best = candidate.unwrap_or(best);

        if (target[axis] - self.point.position[axis]).powi(2) < best.point.squared_distance(&target)

        {
            return opposite_branch
              .and_then(|n| n.nearest(target, depth + 1, Some(best)))
              .or(Some(best));

        }
        Some(best)
    }

    fn construct_kdtree<POINT:KdTreePoint<DIM>>(values:&Vec<POINT>,indices: &mut [usize], depth: usize) -> Option<Rc<Self>> {
        if indices.is_empty() {
            return None;
        }
        let axis = depth % DIM;

        let median = indices.len() / 2;
        let (left, index, right) = indices.select_nth_unstable_by(median, |p1, p2| 
            values[*p1].as_kdtree_point()[axis].partial_cmp(&values[*p2].as_kdtree_point()[axis]).unwrap_or(std::cmp::Ordering::Equal));


        let left = Self::construct_kdtree(values,left, depth + 1);
        let right = Self::construct_kdtree(values,right, depth + 1);
        let point = Point{
            position : *values[*index].as_kdtree_point(),
            index:*index
        };

        Some(Rc::new(Self { point, left, right }))
    }
}

impl<const DIM:usize,POINT:KdTreePoint<DIM>> KdTree<DIM,POINT>{

    ///Return a ref to the nearest POINT by using some coordinates
    pub fn nearest_by_coord(&self, coord :&[f64;DIM]) ->Option<&POINT>{
        let index = self.base_node.as_ref().and_then(|n|
            n.nearest(coord, 0, None)
            .map(|b|b.point.index))?;

        Some(&self.points[index])
        
    }

    ///Return a ref to the nearest POINT by using another POINT
    pub fn nearest(&self,target:&POINT)->Option<&POINT>{
        let target = &target.as_kdtree_point();


        let index = self.base_node.as_ref().and_then(|n|
            n.nearest(target, 0, None)
            .map(|b|b.point.index))?;

        Some(&self.points[index])
        
    }

    pub fn is_empty(&self)->bool{
        self.base_node.is_none()
    }
}