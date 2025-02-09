use core::f32;

struct Node<const DIM: usize> {
    point: Point<DIM>,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

#[derive(Debug, Clone,Copy)]
pub(crate)struct Point<const DIM: usize> {
    pub(crate) position: [f32; DIM],
}

pub struct KdTree<const DIM: usize> {
    base_node: Option<Box<Node<DIM>>>,
}

impl<const DIM: usize> From<Vec<Point<DIM>>> for KdTree<DIM> {
    fn from(value: Vec<Point<DIM>>) -> Self {
        let mut value = value;
        let points = value.as_mut_slice();

        Self {
            base_node: Node::<DIM>::construct_kdtree(points, 0),
        }
    }
}

impl<const DIM: usize> Point<DIM> {
    fn squared_distance(&self, other: &Self) -> f32 {
        self.position
            .iter()
            .zip(other.position.iter())
            .fold(0., |acc, (x, y)| acc + (x - y) * (x - y))
    }
}

impl<const DIM: usize> Node<DIM> {
    fn nearest<'a>(
        &'a self,
        target: Point<DIM>,
        depth: usize,
        best: Option<&'a Node<DIM>>,
    ) -> Option<&'a Self> {
        let point = &self.point;

        let best = if let Some(best) = best {
            if point.squared_distance(&target) < best.point.squared_distance(&target) {
                self
            } else {
                best
            }
        } else {
            self
        };

        let axis = depth % DIM;

        let (next, opposite_branch) = if target.position[axis] < point.position[axis] {
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

        if f32::abs(target.position[axis] - self.point.position[axis])
            < best
                .point.squared_distance(&target)
        {
            return opposite_branch
                .and_then(|n| n.nearest(target, depth + 1, Some(best)));

        }
        Some(best)
    }

    fn construct_kdtree(points: &mut [Point<DIM>], depth: usize) -> Option<Box<Self>> {
        if points.is_empty() {
            return None;
        }
        let axis = depth % DIM;

        points.sort_by(|p1, p2| p1.position[axis].total_cmp(&p2.position[axis]));
        let median = points.len() / 2;
        let (left, right) = points.split_at_mut(median);
        let point = right[0];
        let right = &mut right[1..];


        let left = Self::construct_kdtree(left, depth + 1);
        let right = Self::construct_kdtree(right, depth + 1);

        Some(Box::new(Self { point, left, right }))
    }
}

impl<const DIM:usize> KdTree<DIM>{
    pub(crate) fn nearest(&self,target:Point<DIM>)->Option<Point<DIM>>{
        self.base_node.as_ref().and_then(|n|
            n.nearest(target, 0, None)
            .map(|b|b.point))
    }
}