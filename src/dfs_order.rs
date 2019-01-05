use super::*;
use std::marker::PhantomData;



///Specified which type of dfs order we want. In order/pre order/post order.
trait DfsOrder {
    fn split_mut<T>(nodes: &mut [T]) -> (&mut T, &mut [T], &mut [T]);
    fn split<T>(nodes: &[T]) -> (&T, &[T], &[T]);
}

///Pass this to the tree for In order layout
#[derive(Copy,Clone,Debug)]
pub struct InOrder;
impl DfsOrder for InOrder {
    fn split_mut<T>(nodes: &mut [T]) -> (&mut T, &mut [T], &mut [T]) {
        let mid = nodes.len() / 2;
        let (left, rest) = nodes.split_at_mut(mid);
        let (middle, right) = rest.split_first_mut().unwrap();
        (middle, left, right)
    }
    fn split<T>(nodes: &[T]) -> (&T, &[T], &[T]) {
        let mid = nodes.len() / 2;
        let (left, rest) = nodes.split_at(mid);
        let (middle, right) = rest.split_first().unwrap();
        (middle, left, right)
    }
}

///Pass this to the tree for pre order layout
#[derive(Copy,Clone,Debug)]
pub struct PreOrder;
impl DfsOrder for PreOrder {
    fn split_mut<T>(nodes: &mut [T]) -> (&mut T, &mut [T], &mut [T]) {
        let (middle, rest) = nodes.split_first_mut().unwrap();
        let mm = rest.len() / 2;
        let (left, right) = rest.split_at_mut(mm);
        (middle, left, right)
    }
    fn split<T>(nodes: &[T]) -> (&T, &[T], &[T]) {
        let (middle, rest) = nodes.split_first().unwrap();
        let mm = rest.len() / 2;
        let (left, right) = rest.split_at(mm);
        (middle, left, right)
    }
}

///Pass this to the tree for post order layout
#[derive(Copy,Clone,Debug)]
pub struct PostOrder;
impl DfsOrder for PostOrder {
    fn split_mut<T>(nodes: &mut [T]) -> (&mut T, &mut [T], &mut [T]) {
        let (middle, rest) = nodes.split_last_mut().unwrap();
        let mm = rest.len() / 2;
        let (left, right) = rest.split_at_mut(mm);
        (middle, left, right)
    }
    fn split<T>(nodes: &[T]) -> (&T, &[T], &[T]) {
        let (middle, rest) = nodes.split_last().unwrap();
        let mm = rest.len() / 2;
        let (left, right) = rest.split_at(mm);
        (middle, left, right)
    }
}

///Error indicating the vec that was passed is not a size that you would expect for the given height.
#[derive(Copy, Clone, Debug)]
pub struct NotCompleteTreeSizeErr;

///Container for a dfs order tree. Internally uses a Vec. Derefs to a CompleteTree.
#[repr(transparent)]
pub struct CompleteTreeContainer<T, D> {
    _p:PhantomData<D>,
    nodes: Vec<T>,
}


impl<T> CompleteTreeContainer<T,PreOrder>{
    #[inline]
    pub fn from_vec(vec:Vec<T>)->Result<CompleteTreeContainer<T, PreOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeContainer::from_vec_inner(vec,PreOrder)
    }
}

impl<T> CompleteTreeContainer<T,InOrder>{
    #[inline]
    pub fn from_vec(vec:Vec<T>)->Result<CompleteTreeContainer<T, InOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeContainer::from_vec_inner(vec,InOrder)
    }
}

impl<T> CompleteTreeContainer<T,PostOrder>{
    #[inline]
    pub fn from_vec(vec:Vec<T>)->Result<CompleteTreeContainer<T, PostOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeContainer::from_vec_inner(vec,PostOrder)
    }
}

impl<T,D> CompleteTreeContainer<T,D>{
    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self) -> Vec<T> {
        self.nodes
    }
}

impl<T, D> CompleteTreeContainer<T, D> {
    #[inline]
    fn from_vec_inner(vec: Vec<T>,_order:D) -> Result<CompleteTreeContainer<T, D>, NotCompleteTreeSizeErr> {
        if (vec.len() + 1).is_power_of_two() && !vec.is_empty() {
            Ok(CompleteTreeContainer {
                _p: PhantomData,
                nodes: vec,
            })
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    

}



impl<T, D> std::ops::Deref for CompleteTreeContainer<T, D> {
    type Target = CompleteTree<T, D>;
    #[inline]
    fn deref(&self) -> &CompleteTree<T, D> {
        unsafe { &*(self.nodes.as_slice() as *const [T] as *const dfs_order::CompleteTree<T, D>) }
    }
}
impl<T, D> std::ops::DerefMut for CompleteTreeContainer<T, D> {
    #[inline]
    fn deref_mut(&mut self) -> &mut CompleteTree<T, D> {
        unsafe {
            &mut *(self.nodes.as_mut_slice() as *mut [T] as *mut dfs_order::CompleteTree<T, D>)
        }
    }
}

///Complete binary tree stored in DFS inorder order.
///Height is atleast 1.
#[repr(transparent)]
pub struct CompleteTree<T, D> {
    _p: PhantomData<D>,
    nodes: [T],
}


impl<T> CompleteTree<T,PreOrder>{
    #[inline]
    pub fn from_slice(arr:&[T])->Result<&CompleteTree<T,PreOrder>,NotCompleteTreeSizeErr>{
        CompleteTree::from_slice_inner(arr,PreOrder)
    }
}
impl<T> CompleteTree<T,InOrder>{
    #[inline]
    pub fn from_slice(arr:&[T])->Result<&CompleteTree<T,InOrder>,NotCompleteTreeSizeErr>{
        CompleteTree::from_slice_inner(arr,InOrder)
    }
}
impl<T> CompleteTree<T,PostOrder>{
    #[inline]
    pub fn from_slice(arr:&[T])->Result<&CompleteTree<T,PostOrder>,NotCompleteTreeSizeErr>{
        CompleteTree::from_slice_inner(arr,PostOrder)
    }
}

impl<T> CompleteTree<T,PreOrder>{
    #[inline]
    pub fn from_slice_mut(arr:&mut [T])->Result<&mut CompleteTree<T,PreOrder>,NotCompleteTreeSizeErr>{
        CompleteTree::from_slice_inner_mut(arr,PreOrder)
    }
}
impl<T> CompleteTree<T,InOrder>{
    #[inline]
    pub fn from_slice_mut(arr:&mut [T])->Result<&mut CompleteTree<T,InOrder>,NotCompleteTreeSizeErr>{
        CompleteTree::from_slice_inner_mut(arr,InOrder)
    }
}
impl<T> CompleteTree<T,PostOrder>{
    #[inline]
    pub fn from_slice_mut(arr:&mut [T])->Result<&mut CompleteTree<T,PostOrder>,NotCompleteTreeSizeErr>{
        CompleteTree::from_slice_inner_mut(arr,PostOrder)
    }
}


impl<T, D> CompleteTree<T, D> {
    #[inline]
    fn from_slice_inner(arr: &[T],_order:D) -> Result<&CompleteTree<T, D>, NotCompleteTreeSizeErr> {
        if valid_node_num(arr.len()) {
            let tree = unsafe { &*(arr as *const [T] as *const dfs_order::CompleteTree<T, D>) };
            Ok(tree)
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline]
    fn from_slice_inner_mut(
        arr: &mut [T],_order:D
    ) -> Result<&mut CompleteTree<T, D>, NotCompleteTreeSizeErr> {
        if valid_node_num(arr.len()) {
            let tree = unsafe { &mut *(arr as *mut [T] as *mut dfs_order::CompleteTree<T, D>) };
            Ok(tree)
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        compute_height(self.nodes.len())
    }

    #[inline]
    pub fn get_nodes(&self) -> &[T] {
        &self.nodes
    }

    #[inline]
    pub fn get_nodes_mut(&mut self) -> &mut [T] {
        &mut self.nodes
    }

    #[inline]
    pub fn vistr(&self) -> Vistr<T, D> {
        Vistr {
            _p: PhantomData,
            remaining: &self.nodes,
        }
    }

    #[inline]
    pub fn vistr_mut(&mut self) -> VistrMut<T, D> {
        VistrMut {
            _p: PhantomData,
            remaining: &mut self.nodes,
        }
    }
}

///Tree visitor that returns a reference to each element in the tree.
#[repr(transparent)]
pub struct Vistr<'a, T: 'a, D> {
    _p: PhantomData<D>,
    remaining: &'a [T],
}

impl<'a, T: 'a, D> Vistr<'a, T, D> {
    #[inline]
    pub fn create_wrap(&self) -> Vistr<T, D> {
        Vistr {
            _p: PhantomData,
            remaining: self.remaining,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.remaining
    }

    #[inline]
    pub fn into_slice(self) -> &'a [T] {
        self.remaining
    }
}

impl<'a,T:'a> Visitor for Vistr<'a,T,PreOrder>{
    type Item = &'a T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_next::<_,PreOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_dfs_level_remaining_hint(self)
    }

    
    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_preorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter(){
            func(a);
        }
    }
}
impl<'a,T:'a> Visitor for Vistr<'a,T,InOrder>{
    type Item = &'a T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_next::<_,InOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_dfs_level_remaining_hint(self)
    }

    
    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_inorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter(){
            func(a);
        }
    }
}
impl<'a,T:'a> Visitor for Vistr<'a,T,PostOrder>{
    type Item = &'a T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_next::<_,PostOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_dfs_level_remaining_hint(self)
    }

    
    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_postorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter(){
            func(a);
        }
    }
}


fn vistr_dfs_level_remaining_hint<T,D:DfsOrder>(vistr:&Vistr<T,D>)->(usize,Option<usize>){
    let left = ((vistr.remaining.len() + 1) as f64).log2() as usize;
    (left, Some(left))
}
fn vistr_next<T,D:DfsOrder>(vistr:Vistr<T,D>)->(&T,Option<[Vistr<T,D>;2]>){
    let remaining = vistr.remaining;
    if remaining.len() == 1 {
        (&remaining[0], None)
    } else {
        let (middle, left, right) = D::split(remaining);

        (
            middle,
            Some([
                Vistr {
                    _p: PhantomData,
                    remaining: left,
                },
                Vistr {
                    _p: PhantomData,
                    remaining: right,
                },
            ]),
        )
    }
}

unsafe impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T, PreOrder> {}
unsafe impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T, InOrder> {}
unsafe impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T, PostOrder> {}




impl<'a, T, D> std::ops::Deref for VistrMut<'a, T, D> {
    type Target = Vistr<'a, T, D>;
    #[inline]
    fn deref(&self) -> &Vistr<'a, T, D> {
        unsafe { &*(self as *const VistrMut<T, D> as *const Vistr<T, D>) }
    }
}






///Tree visitor that returns a mutable reference to each element in the tree.
#[repr(transparent)]
pub struct VistrMut<'a, T: 'a, D> {
    _p: PhantomData<D>,
    remaining: &'a mut [T],
}

impl<'a, T: 'a, D> VistrMut<'a, T, D> {
    #[inline]
    pub fn create_wrap_mut(&mut self) -> VistrMut<T, D> {
        VistrMut {
            _p: PhantomData,
            remaining: self.remaining,
        }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        self.remaining
    }

    #[inline]
    pub fn into_slice(self) -> &'a mut [T] {
        self.remaining
    }
}


fn vistr_mut_dfs_level_remaining_hint<T,D:DfsOrder>(vistr:&VistrMut<T,D>)->(usize,Option<usize>){
    let left = ((vistr.remaining.len() + 1) as f64).log2() as usize;
    (left, Some(left))
}
fn vistr_mut_next<T,D:DfsOrder>(vistr:VistrMut<T,D>)->(&mut T,Option<[VistrMut<T,D>;2]>){
    let remaining = vistr.remaining;
    if remaining.len() == 1 {
        (&mut remaining[0], None)
    } else {
        let (middle, left, right) = D::split_mut(remaining);

        (
            middle,
            Some([
                VistrMut {
                    _p: PhantomData,
                    remaining: left,
                },
                VistrMut {
                    _p: PhantomData,
                    remaining: right,
                },
            ]),
        )
    }
}



impl<'a,T:'a> Visitor for VistrMut<'a,T,PreOrder>{
    type Item = &'a mut T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_mut_next::<_,PreOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_mut_dfs_level_remaining_hint(self)
    }

    
    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_preorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter_mut(){
            func(a);
        }
    }
}

impl<'a,T:'a> Visitor for VistrMut<'a,T,InOrder>{
    type Item = &'a mut T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_mut_next::<_,InOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_mut_dfs_level_remaining_hint(self)
    }

    
    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_inorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter_mut(){
            func(a);
        }
    }
}
impl<'a,T:'a> Visitor for VistrMut<'a,T,PostOrder>{
    type Item = &'a mut T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_mut_next::<_,PostOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_mut_dfs_level_remaining_hint(self)
    }

    
    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_postorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter_mut(){
            func(a);
        }
    }
}

unsafe impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T, PreOrder> {}
unsafe impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T, InOrder> {}
unsafe impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T, PostOrder> {}
