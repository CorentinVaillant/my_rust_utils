//! This module implements a Kd-Tree. 
//! The purpose of this structure is to organize K-dimensional points
//!
//! # Features 
//! - Construction of a Kd-Tree from a set of points
//! - `nearest` function to find the nearest point to a given one


pub mod kd_tree_traits;

#[cfg(test)]
pub mod tests;

use std::rc::Rc;

pub use kd_tree_traits::KdTreePoint;

///Node for the KdTree
#[derive(Debug,Clone)]
struct Node<const DIM: usize> {
    point: Point<DIM>,      // the stored point in this node
    left: Option<Rc<Self>>, // left child
    right: Option<Rc<Self>>,// right child
}

///Structure that represent a k-dimensional point
#[derive(Debug, Clone,Copy)]
pub(crate)struct Point<const DIM: usize> {
    pub(crate) position: [f64; DIM], //Coordinates of the point
    index:usize //Index of the point in the original input list
}

/// A Kd-Tree data structure for partitioning a k-dimensional space.
/// 
/// This structure allows efficient nearest neighbor searches.
/// This structure stores a `Vec` of POINT
///
/// # Type Parameters:
/// - `DIM`: The number of dimensions.
/// - `POINT`: The type of point stored in the tree, which must implement `KdTreePoint`.
/// 

#[derive(Debug,Clone)]
pub struct KdTree<const DIM: usize,POINT: KdTreePoint<DIM>> {
    root: Option<Rc<Node<DIM>>>, //Root node of the Kd-Tree

    points : Vec<POINT>
}

impl<const DIM: usize, POINT:KdTreePoint<DIM>> From<Vec<POINT>> for KdTree<DIM,POINT> {
    /// Constructs a Kd-Tree from a vector of points.
    fn from(value: Vec<POINT>) -> Self {
        if DIM == 0{
            return Self{
                root : None,
                points : value,
            };
        }

        let mut indices = (0..value.len()).collect::<Vec<_>>();       

        Self {
            root: Node::<DIM>::construct_kdtree(&value,&mut indices.as_mut_slice(), 0),
            points : value
        }
    }
}

impl<const DIM: usize> Point<DIM> {
    /// Computes the squared Euclidean distance between this point and another point.
    fn squared_distance(&self, other: &[f64;DIM]) -> f64 {
        self.position
            .iter()
            .zip(other.iter())
            .fold(0., |acc, (x, y)| acc + (x - y) * (x - y))
    }
}

impl<const DIM: usize> Node<DIM> {
    /// Recursively finds the nearest neighbor to the target point.
    ///
    /// # Parameters:
    /// - `target`: The coordinates of the target point.
    /// - `depth`: The current depth in the tree (used to determine the split axis).
    /// - `best`: The best candidate node found so far.
    ///
    /// # Returns:
    /// - An `Option` containing a reference to the nearest node.
    fn nearest<'a>(
        &'a self,
        target: &[f64;DIM],
        depth: usize,
        best: Option<&'a Node<DIM>>,
    ) -> Option<&'a Self> {
        let point = &self.point;

        // Update the best node if this node is closer
        let best = match best {
            Some(best) if best.point.squared_distance(target) <= point.squared_distance(target) => best,
            _ => self,
        };
    

        let axis = depth % DIM;// Determine the splitting axis

        // Determine the next subtree to search
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

        // Search the next subtree
        let candidate = next.and_then(|n| n.nearest(target, depth + 1, Some(best)));
        let best = candidate.unwrap_or(best);

        // Check if we need to search the opposite subtree
        if (target[axis] - self.point.position[axis]).powi(2) < best.point.squared_distance(&target){
            return opposite_branch
              .and_then(|n| n.nearest(target, depth + 1, Some(best)))
              .or(Some(best));

        }
        Some(best)
    }

    /// Constructs a Kd-Tree recursively.
    ///
    /// # Parameters:
    /// - `values`: The input points.
    /// - `indices`: Mutable slice of point indices to sort and partition.
    /// - `depth`: The current depth in the tree.
    ///
    /// # Returns:
    /// - An `Option<Rc<Node<DIM>>>` representing the root of the constructed subtree.
    fn construct_kdtree<POINT:KdTreePoint<DIM>>(values:&Vec<POINT>,indices: &mut [usize], depth: usize) -> Option<Rc<Self>> {
        if indices.is_empty() {
            return None;
        }
        let axis = depth % DIM; //DIM != 0 because the condition is verify into the from function

        // Find the median index
        let median = indices.len() / 2;
        let (left, index, right) = indices.select_nth_unstable_by(median, |p1, p2| 
            values[*p1].as_kdtree_point()[axis].partial_cmp(&values[*p2].as_kdtree_point()[axis]).unwrap_or(std::cmp::Ordering::Equal));

        // Recursively construct left and right subtrees
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

    ///Returns a reference to the nearest POINT using given coordinates
    pub fn nearest_by_coord(&self, coord :&[f64;DIM]) ->Option<&POINT>{
        let index = self.root.as_ref().and_then(|n|
            n.nearest(coord, 0, None)
            .map(|b|b.point.index))?;

        Some(&self.points[index])
        
    }

    ///Returns a reference to the nearest POINT using another POINT
    pub fn nearest(&self,target:&POINT)->Option<&POINT>{
        let target = &target.as_kdtree_point();


        let index = self.root.as_ref().and_then(|n|
            n.nearest(target, 0, None)
            .map(|b|b.point.index))?;

        Some(&self.points[index])
        
    }

    pub fn is_empty(&self)->bool{
        self.root.is_none()
    }
}