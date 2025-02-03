pub struct KdTree<const K: usize,POINT>{
    base_node : Option<KdTreeNode<K,POINT>>,
    depth: usize
}

impl<const K: usize, POINT> KdTree<K,POINT> {
    pub fn new(point_list:&[POINT], depth:usize)->Self{
        if point_list.is_empty(){
            return Self::empty(depth);
        }

        //Selection of the axe of comparaison
        let axe = depth % K;

        //Selection of the midle point
        //todo

        

    }

    pub fn empty(depth:usize)->Self{
        Self{
            base_node:None,
            depth
        }
    }

    pub fn is_empty(&self)->bool{
        self.base_node.is_some()
    }
}

impl<const K: usize, POINT> KdTreeNode<K,POINT> {
    todo!()
}

pub trait KdTreePoint<const K: usize>{

}

enum KdTreeNodeData<POINT> {
    Child,
    Pointset(Vec<POINT>)
}

struct KdTreeNode<const K:usize,POINT>{
    is_leaf :bool,
    min : POINT,
    max : POINT,
    data: KdTreeNodeData<POINT>,
    parent : Option<std::ptr::NonNull<Self>>


}
