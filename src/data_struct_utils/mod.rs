pub mod kd_tree;
pub mod kd_tree_traits;

// #[cfg(test)]
// mod tests {
//     use crate::data_struct_utils::kd_tree::{KdTree, Point};

//     #[test]
//     fn test_nearest_neighbor() {
//         let points = vec![
//             Point { position: [0.0, 0.0] },
//             Point { position: [1.0, 1.0] },
//             Point { position: [2.0, 2.0] },
//         ];
//         let tree = KdTree::from(points);
//         let target = Point { position: [1.1, 1.1] };
//         let nearest = tree.nearest(target).unwrap();
//         assert_eq!(nearest.position, [1.0, 1.0]);
//     }
// }