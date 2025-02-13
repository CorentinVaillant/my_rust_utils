use std::collections::VecDeque;

use super::{KdTree, KdTreePoint, Node};

struct KdTreePrefixIterator<'a,const DIM :usize,POINT:KdTreePoint<DIM>>{
    tree : &'a KdTree<DIM,POINT>,
    iterator_stack : VecDeque<&'a Node<DIM>>,
}

impl<'a,const DIM :usize,POINT:KdTreePoint<DIM>> Iterator for KdTreePrefixIterator<'a,DIM,POINT> {
    type Item = &'a POINT;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_node = self.iterator_stack.pop_front()?;
        

        todo!()
    }
}