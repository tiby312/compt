use super::*;
use alloc::boxed::Box;
use core::marker::PhantomData;

///Specified which type of dfs order we want. In order/pre order/post order.
trait DfsOrder: Clone {
    fn split_mut<T>(nodes: &mut [T]) -> (&mut T, &mut [T], &mut [T]);
    fn split<T>(nodes: &[T]) -> (&T, &[T], &[T]);
}

///Pass this to the tree for In order layout
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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

///Container for a dfs order tree. Internally uses a Vec. Derefs to a CompleteTree.
#[repr(transparent)]
#[derive(Clone)]
pub struct CompleteTreeContainer<T, D> {
    _p: PhantomData<D>,
    nodes: Box<[T]>,
}

impl<T> CompleteTreeContainer<T, PreOrder> {
    #[inline]
    pub fn from_preorder(
        vec: Vec<T>,
    ) -> Result<CompleteTreeContainer<T, PreOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeContainer::from_vec_inner(vec, PreOrder)
    }
}

impl<T> CompleteTreeContainer<T, InOrder> {
    #[inline]
    pub fn from_inorder(
        vec: Vec<T>,
    ) -> Result<CompleteTreeContainer<T, InOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeContainer::from_vec_inner(vec, InOrder)
    }
}

impl<T> CompleteTreeContainer<T, PostOrder> {
    #[inline]
    pub fn from_postorder(
        vec: Vec<T>,
    ) -> Result<CompleteTreeContainer<T, PostOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeContainer::from_vec_inner(vec, PostOrder)
    }
}

impl<T, D> CompleteTreeContainer<T, D> {
    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self) -> Box<[T]> {
        self.nodes
    }

    pub fn as_tree(&self) -> CompleteTree<T, D> {
        CompleteTree {
            _p: PhantomData,
            nodes: &self.nodes,
        }
    }

    pub fn as_tree_mut(&mut self) -> CompleteTreeMut<T, D> {
        CompleteTreeMut {
            _p: PhantomData,
            nodes: &mut self.nodes,
        }
    }

    #[inline]
    fn from_vec_inner(
        vec: Vec<T>,
        _order: D,
    ) -> Result<CompleteTreeContainer<T, D>, NotCompleteTreeSizeErr> {
        valid_node_num(vec.len())?;

        Ok(CompleteTreeContainer {
            _p: PhantomData,
            nodes: vec.into_boxed_slice(),
        })
    }
}

///Complete binary tree stored in DFS inorder order.
///Height is atleast 1.
#[repr(transparent)]
pub struct CompleteTree<'a, T, D> {
    _p: PhantomData<D>,
    nodes: &'a [T],
}

impl<'a, T> CompleteTree<'a, T, PreOrder> {
    #[inline]
    pub fn from_preorder(
        arr: &'a [T],
    ) -> Result<CompleteTree<'a, T, PreOrder>, NotCompleteTreeSizeErr> {
        CompleteTree::from_slice_inner(arr, PreOrder)
    }
}
impl<'a, T> CompleteTree<'a, T, InOrder> {
    #[inline]
    pub fn from_inorder(
        arr: &'a [T],
    ) -> Result<CompleteTree<'a, T, InOrder>, NotCompleteTreeSizeErr> {
        CompleteTree::from_slice_inner(arr, InOrder)
    }
}
impl<'a, T> CompleteTree<'a, T, PostOrder> {
    #[inline]
    pub fn from_postorder(
        arr: &'a [T],
    ) -> Result<CompleteTree<'a, T, PostOrder>, NotCompleteTreeSizeErr> {
        CompleteTree::from_slice_inner(arr, PostOrder)
    }
}

pub struct CompleteTreeMut<'a, T, D> {
    _p: PhantomData<D>,
    nodes: &'a mut [T],
}

impl<'a, T> CompleteTreeMut<'a, T, PreOrder> {
    #[inline]
    pub fn from_preorder_mut(
        arr: &'a mut [T],
    ) -> Result<CompleteTreeMut<'a, T, PreOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeMut::from_slice_inner_mut(arr, PreOrder)
    }
}
impl<'a, T> CompleteTree<'a, T, InOrder> {
    #[inline]
    pub fn from_inorder_mut(
        arr: &'a mut [T],
    ) -> Result<CompleteTreeMut<'a, T, InOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeMut::from_slice_inner_mut(arr, InOrder)
    }
}
impl<'a, T> CompleteTreeMut<'a, T, PostOrder> {
    #[inline]
    pub fn from_post_mut(
        arr: &'a mut [T],
    ) -> Result<CompleteTreeMut<'a, T, PostOrder>, NotCompleteTreeSizeErr> {
        CompleteTreeMut::from_slice_inner_mut(arr, PostOrder)
    }
}

impl<'a, T, D> From<CompleteTreeMut<'a, T, D>> for CompleteTree<'a, T, D> {
    fn from(a: CompleteTreeMut<'a, T, D>) -> CompleteTree<'a, T, D> {
        CompleteTree {
            _p: PhantomData,
            nodes: a.nodes,
        }
    }
}

impl<'a, T, D> CompleteTreeMut<'a, T, D> {
    pub fn as_tree(&self) -> CompleteTree<T, D> {
        CompleteTree {
            _p: PhantomData,
            nodes: self.nodes,
        }
    }

    pub fn borrow_mut(&mut self)->CompleteTreeMut<T,D>{
        CompleteTreeMut{
            _p:PhantomData,
            nodes:self.nodes
        }
    }

    #[inline]
    fn from_slice_inner_mut(
        arr: &'a mut [T],
        _order: D,
    ) -> Result<CompleteTreeMut<'a, T, D>, NotCompleteTreeSizeErr> {
        valid_node_num(arr.len())?;
        Ok(CompleteTreeMut {
            _p: PhantomData,
            nodes: arr,
        })
    }

    #[inline]
    pub fn get_nodes_mut( self) -> &'a mut [T] {
        self.nodes
    }

    #[inline]
    pub fn vistr_mut(self) -> VistrMut<'a,T, D> {
        VistrMut {
            _p: PhantomData,
            remaining:  self.nodes,
        }
    }
}
impl<'a, T, D> CompleteTree<'a, T, D> {
    #[inline]
    fn from_slice_inner(
        arr: &'a [T],
        _order: D,
    ) -> Result<CompleteTree<'a, T, D>, NotCompleteTreeSizeErr> {
        valid_node_num(arr.len())?;
        Ok(CompleteTree {
            _p: PhantomData,
            nodes: arr,
        })
    }

    pub fn borrow(&self)->CompleteTree<T,D>{
        CompleteTree{
            _p:PhantomData,
            nodes:self.nodes
        }
    }

    #[inline]
    pub fn get_height(self) -> usize {
        compute_height(self.nodes.len())
    }

    #[inline]
    pub fn get_nodes(self) -> &'a [T] {
        self.nodes
    }

    #[inline]
    pub fn vistr(self) -> Vistr<'a,T, D> {
        Vistr {
            _p: PhantomData,
            remaining: self.nodes,
        }
    }
}

///Tree visitor that returns a reference to each element in the tree.
#[repr(transparent)]

pub struct Vistr<'a, T: 'a, D> {
    _p: PhantomData<D>,
    remaining: &'a [T],
}

impl<'a, T: 'a, D> Clone for Vistr<'a, T, D> {
    fn clone(&self) -> Vistr<'a, T, D> {
        Vistr {
            _p: PhantomData,
            remaining: self.remaining,
        }
    }
}

impl<'a, T: 'a, D> Vistr<'a, T, D> {
    #[inline]
    pub fn borrow(&self) -> Vistr<T, D> {
        Vistr {
            _p: PhantomData,
            remaining: self.remaining,
        }
    }

    #[inline]
    pub fn into_slice(self) -> &'a [T] {
        self.remaining
    }
}

impl<'a, T: 'a> Visitor for Vistr<'a, T, PreOrder> {
    type Item = &'a T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_next::<_, PreOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_dfs_level_remaining_hint(self)
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_preorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter() {
            func(a);
        }
    }
}
impl<'a, T: 'a> Visitor for Vistr<'a, T, InOrder> {
    type Item = &'a T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_next::<_, InOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_dfs_level_remaining_hint(self)
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_inorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter() {
            func(a);
        }
    }
}
impl<'a, T: 'a> Visitor for Vistr<'a, T, PostOrder> {
    type Item = &'a T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_next::<_, PostOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_dfs_level_remaining_hint(self)
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_postorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter() {
            func(a);
        }
    }
}

//TODO put this somewhere else
fn log_2(x: usize) -> usize {
    const fn num_bits<T>() -> usize {
        core::mem::size_of::<T>() * 8
    }

    assert!(x > 0);
    (num_bits::<usize>() as u32 - x.leading_zeros() - 1) as usize
}

fn vistr_dfs_level_remaining_hint<T, D: DfsOrder>(vistr: &Vistr<T, D>) -> (usize, Option<usize>) {
    let left = log_2(vistr.remaining.len() + 1);
    //let left = ((vistr.remaining.len() + 1) as f64).log2() as usize;
    (left, Some(left))
}
fn vistr_next<T, D: DfsOrder>(vistr: Vistr<T, D>) -> (&T, Option<[Vistr<T, D>; 2]>) {
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

impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T, PreOrder> {}
impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T, InOrder> {}
impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T, PostOrder> {}

impl<'a, T: 'a, D> From<VistrMut<'a, T, D>> for Vistr<'a, T, D> {
    #[inline]
    fn from(a: VistrMut<'a, T, D>) -> Vistr<'a, T, D> {
        Vistr {
            _p: PhantomData,
            remaining: a.remaining,
        }
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
    pub fn borrow(&self) -> Vistr<T, D> {
        Vistr {
            _p: PhantomData,
            remaining: self.remaining,
        }
    }

    #[inline]
    pub fn borrow_mut(&mut self) -> VistrMut<T, D> {
        VistrMut {
            _p: PhantomData,
            remaining: self.remaining,
        }
    }

    #[inline]
    pub fn into_slice(self) -> &'a mut [T] {
        self.remaining
    }
}

fn vistr_mut_dfs_level_remaining_hint<T, D: DfsOrder>(
    vistr: &VistrMut<T, D>,
) -> (usize, Option<usize>) {
    let left = log_2(vistr.remaining.len() + 1);
    //let left = ((vistr.remaining.len() + 1) as f64).log2() as usize;
    (left, Some(left))
}
fn vistr_mut_next<T, D: DfsOrder>(vistr: VistrMut<T, D>) -> (&mut T, Option<[VistrMut<T, D>; 2]>) {
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

impl<'a, T: 'a> Visitor for VistrMut<'a, T, PreOrder> {
    type Item = &'a mut T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_mut_next::<_, PreOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_mut_dfs_level_remaining_hint(self)
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_preorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter_mut() {
            func(a);
        }
    }
}

impl<'a, T: 'a> Visitor for VistrMut<'a, T, InOrder> {
    type Item = &'a mut T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_mut_next::<_, InOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_mut_dfs_level_remaining_hint(self)
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_inorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter_mut() {
            func(a);
        }
    }
}
impl<'a, T: 'a> Visitor for VistrMut<'a, T, PostOrder> {
    type Item = &'a mut T;
    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        vistr_mut_next::<_, PostOrder>(self)
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        vistr_mut_dfs_level_remaining_hint(self)
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_postorder(self, mut func: impl FnMut(Self::Item)) {
        for a in self.remaining.iter_mut() {
            func(a);
        }
    }
}

impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T, PreOrder> {}
impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T, InOrder> {}
impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T, PostOrder> {}
