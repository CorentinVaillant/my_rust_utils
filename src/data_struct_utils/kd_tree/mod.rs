//! This module implements a Kd-Tree. 
//! The purpose of this structure is to organize K-dimensional points
//!
//! # Features 
//! - Construction of a Kd-Tree from a set of points
//! - `nearest` function to find the nearest point to a given one


pub mod kd_tree_traits;

#[cfg(test)]
pub mod tests;


pub use kd_tree_traits::KdTreePoint;

///Node for the KdTree
#[derive(Debug,Clone)]
struct Node<const DIM: usize> {
    point: Point<DIM>,      // the stored point in this node
    left: Option<Box<Self>>, // left child
    right: Option<Box<Self>>,// right child
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
    root: Option<Box<Node<DIM>>>, //Root node of the Kd-Tree

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

impl<'a,const DIM: usize> Node<DIM> {
    /// Recursively finds the nearest neighbor to the target point.
    ///
    /// # Parameters:
    /// - `target`: The coordinates of the target point.
    /// - `depth`: The current depth in the tree (used to determine the split axis).
    /// - `best`: The best candidate node found so far.
    ///
    /// # Returns:
    /// - An `Option` containing a reference to the nearest node.
    fn nearest(
        &'a self,
        target: &[f64;DIM],
        depth: usize,
        best: Option<&'a Node<DIM>>,
    ) -> Option<&'a Self> {
        let point = &self.point;


        let self_distance = point.squared_distance(target);
        let best_distance = best.map_or(f64::INFINITY, |b| b.point.squared_distance(target));

        // Update the best node if this node is closer
        let best = if self_distance < best_distance { self } else { best.unwrap_or(self) };
    

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
    fn construct_kdtree<POINT:KdTreePoint<DIM>>(values:&[POINT],indices: &mut [usize], depth: usize) -> Option<Box<Self>> {
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

        Some(Box::new(Self { point, left, right }))
    }

    fn add_node(&mut self,new_node:Self,depth: usize){
        let axis = depth % DIM;
        
        if self.point.position[axis] < new_node.point.position[axis]{
            if let Some(right) = &mut self.right{
                right.add_node(new_node, depth+1);
                
            }else {
                self.right = Some(Box::new(new_node));
                
            }
        }else{
            if let Some(left) = &mut self.left{
                left.add_node(new_node, depth+1);
                
            }else {
                self.left = Some(Box::new(new_node));
                
            }
        }
    }

    fn is_leaf(&self)->bool{
        self.left.is_none() && self.right.is_none()
    }

    fn height(&self,depth: usize)->usize{
        if self.is_leaf(){
            return depth+1;
        }else {
            return usize::max(
                (self.right.as_ref()).map(|r|r.height(depth+1)).unwrap_or(0), 
                (self.left.as_ref()).map(|r|r.height(depth+1)).unwrap_or(0));
        }
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

    pub fn add_point(&mut self, point: POINT) {
        let position = *point.as_kdtree_point();
        let index = self.points.len();
        self.points.push(point);
    
        let new_node = Node {
            point: Point { position, index },
            left: None,
            right: None,
        };
    
        if let Some(root) = &mut self.root {
            root.add_node(new_node, 0);
        } else {
            self.root = Some(Box::new(new_node));
        }
    }

    pub fn is_empty(&self)->bool{
        self.root.is_none()
    }

    pub fn size(&self)->usize{
        self.points.len()
    }

    pub fn height(&self)->usize{
        self.root.as_ref()
           .map(|r|r.height(0))
           .unwrap_or(0)
    }
}