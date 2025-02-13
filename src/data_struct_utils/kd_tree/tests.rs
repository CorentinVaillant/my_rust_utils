#[cfg(test)]
pub(crate) mod kdtree_tests {
    use crate::data_struct_utils::kd_tree::*;

    #[test]
    fn test_kdtree_construction() {
        let points = vec![
            [1.0, 2.0],
            [3.0, 4.0],
            [5.0, 6.0],
            [7.0, 8.0],
        ];

        let kdtree = KdTree::<2, [f64; 2]>::from(points.clone());

        // Ensure the tree is constructed correctly
        assert!(!kdtree.is_empty());
    }

    #[test]
    fn test_nearest_by_coord() {
        let points = vec![
            [1.0, 2.0],
            [4.0, 4.0],
            [5.0, 6.0],
            [7.0, 8.0],
        ];

        let kdtree = KdTree::<2, [f64; 2]>::from(points.clone());

        // Test nearest neighbor search by coordinates
        let nearest = kdtree.nearest_by_coord(&[2.0, 3.0]).unwrap();
        assert_eq!(nearest, &[1.0, 2.0]);

        let nearest = kdtree.nearest_by_coord(&[6.0, 7.0]).unwrap();
        assert_eq!(nearest, &[5.0, 6.0]);
    }

    #[test]
    fn test_nearest() {
        let points = vec![
            [1.0, 2.0],
            [4.0, 4.0],
            [5.0, 6.0],
            [7.0, 8.0],
        ];

        let kdtree = KdTree::<2, [f64; 2]>::from(points.clone());
        assert!(!kdtree.is_empty());
        println!("KD-Tree Root: {:#?}", kdtree.root);

        // Test nearest neighbor search by another point
        let target = [2.0, 3.0];
        let nearest = kdtree.nearest(&target).unwrap();
        assert_eq!(nearest, &[1.0, 2.0]);

        let target = [6.0, 7.0];
        let nearest = kdtree.nearest(&target).unwrap();
        assert_eq!(nearest, &[5.0, 6.0]);
    }

    #[test]
    fn test_empty_tree() {
        let points: Vec<[f64; 2]> = vec![];

        let kdtree = KdTree::<2, [f64; 2]>::from(points);

        // Test nearest neighbor search on an empty tree
        assert!(kdtree.nearest_by_coord(&[1.0, 2.0]).is_none());
    }

    #[test]
    fn test_single_point_tree() {
        let points = vec![[1.0, 2.0]];

        let kdtree = KdTree::<2, [f64; 2]>::from(points.clone());

        // Test nearest neighbor search on a tree with a single point
        let nearest = kdtree.nearest_by_coord(&[3.0, 4.0]).unwrap();
        assert_eq!(nearest, &[1.0, 2.0]);
    }

    #[test]
    fn test_large_tree() {
        let points = (0..100).map(|i| [i as f64, i as f64]).collect::<Vec<_>>();

        let kdtree = KdTree::from(points.clone());

        // Test nearest neighbor search on a larger tree
        let nearest = kdtree.nearest_by_coord(&[50.5, 50.5]).unwrap();
        assert_eq!(nearest, &[50.0, 50.0]);

        let nearest = kdtree.nearest_by_coord(&[99.9, 99.9]).unwrap();
        assert_eq!(nearest, &[99.0, 99.0]);
    }

    #[test]
    fn test_zero_dim_tree(){
        let points: Vec<[f64; 0]> = (0..100).map(|_i|[]).collect::<Vec<_>>();

        let kdtree = KdTree::from(points);

        let nearest = kdtree.nearest_by_coord(&[]);
        assert!(nearest.is_none());
    }


    #[test]
    fn test_add(){
        let points = vec![
            [1.,1.],
            [0.,0.],
            [5.,5.]
        ];

        let mut kd_tree = KdTree::from(points);

        let nearest = kd_tree.nearest(&[10.,10.]);
        assert_eq!(nearest, Some(&[5.,5.]));

        kd_tree.add_point([9.,5.]);

        let nearest = kd_tree.nearest(&[10.,10.]);
        assert_eq!(nearest, Some(&[9.,5.]));
    }

    #[test]
    fn test_add_to_empty_tree(){
        let points = vec![];

        let mut kd_tree = KdTree::from(points);

        let nearest = kd_tree.nearest(&[10.,10.]);
        assert_eq!(nearest, None);

        kd_tree.add_point([9.,5.]);

        let nearest = kd_tree.nearest(&[10.,10.]);
        assert_eq!(nearest, Some(&[9.,5.]));
    }
}