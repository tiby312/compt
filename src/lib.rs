//!## Summary
//! A library that provides a complete binary tree visitor trait with default implemenations for visiting strategies such as dfs_inorder or dfs_preorder, etc.
//! Some adaptors are also provided that let you map, zip, or optionally also produce the depth on every call to next().
//! It also provides two flavors of a complete binary tree data structure with mutable and immutable visitors that implement the visitor trait.
//! One laid out in bfs, and one laid out in dfs in order in memory. Both of these flavors assume that every node in the tree is the same type.
//!
//! This is the trait that this crate revoles around:
//!```
//!pub trait Visitor:Sized{
//!    type Item;
//!    fn next(self)->(Self::Item,Option<[Self;2]>);
//!}
//!```
//! If you have a visitor, you can call next() on it to consume it, and produce a reference to the node it is visiting, plus
//! the children nodes.
//!
//! The fact that the iterator is consumed when calling next(), allows us to return mutable references without fear of the users
//! being able to create the same mutable reference some other way.
//! So this property provides a way to get mutable references to children nodes simultaneously safely. Useful for parallelizing divide and conquer style problems.
//!
//!## Goals
//!
//! To provide a useful complete binary tree visitor trait that has some similar features to the Iterator trait,
//! such as zip(), and map(), and that can be used in parallel divide and conquer style problems.
//!
//!## Unsafety in the provided two tree implementations
//!
//! With a regular Vec, getting one mutable reference to an element will borrow the
//! entire Vec. However a tree has properties that let us make guarentees about
//! which elements can be mutably borrowed at the same time. With the bfs tree, the children
//! for an element at index k can be found at 2k+1 and 2k+2. This means that we are guarenteed that the parent,
//! and the two children are all distinct elements and so mutable references two all of them can exist at the same time.
//! With the dfs implementation, on every call to next() we use split_at_mut() to split the current slice we have into three parts:
//! the current node, the elements ot the left, and the elements to the right.
//!
//!## Memory Locality
//!
//! Ordering the elements in dfs in order is likely better for divide and conquer style problems.
//! The main memory access pattern that we want to be fast is the following: If I have a parent, I hope to be able
//! to access the children fast. So we want the children to be close to the parent.
//! While in bfs order, the root's children are literally right next to it, the children of nodes in the the second
//! to last level of the tree could be extremly far apart (possibly n/2 elements away!).
//! With dfs order, as you go down the tree, you gain better and better locality.
//!
//! A downside with dfs ordering is that if not all space is used by the leaf nodes,
//! Then that wasted space is interspered throughout the entire data structure. In a bfs ordering,
//! All the leaves are at the end of the data structure, so the memory locality penalty may not be as high
//! When traversing tree.
//!
//! For parallel divide and conquer, dfs ordering is likely better than bfs ordering.
//! With dfs ordering, once you divide the problem, the memory sections that each task deals with
//! do not intersect. With bfs ordering the tasks would still be operating on memory sections that interleave
//!

#![no_std]
extern crate alloc;
use alloc::vec::Vec;

///A complete binary tree stored in a Vec<T> laid out in bfs order.
pub mod bfs_order;
///A complete binary tree stored in a Vec<T> laid out in dfs in order.
///One advantage of using the dfs order over the bfs order, is that at any point during traversal of the tree,
///you can turn the visitor into a slice representing the rest of the nodes underneath that visitor.
pub mod dfs_order;

//use core::collections::vec_deque::VecDeque;

///Compute the number of nodes in a complete binary tree based on a height.
#[inline]
pub fn compute_num_nodes(height: usize) -> usize {
    (1 << height) - 1
}

#[must_use]
fn valid_node_num(num: usize) -> bool {
    (num + 1).is_power_of_two() && num != 0
}

///Computes the height for the number of nodes given.
///Returns the number of trailing zeroes after the last bit in the binary representation.
///For complete binary trees this would be the height.
#[inline]
pub fn compute_height(num_nodes: usize) -> usize {
    (num_nodes + 1).trailing_zeros() as usize
}

///Dfs in order iterator. Each call to next() will return the next element
///in dfs in order.
///Internally uses a Vec for the stack.
pub struct DfsInOrderIter<C: Visitor> {
    a: Vec<(C::Item, Option<C>)>,
    length: Option<usize>,
    min_length: usize,
    num: usize,
}

impl<C: Visitor> DfsInOrderIter<C> {
    fn add_all_lefts(stack: &mut Vec<(C::Item, Option<C>)>, node: C) {
        let mut target = Some(node);
        loop {
            let (i, next) = target.take().unwrap().next();
            match next {
                Some([left, right]) => {
                    let bleep = (i, Some(right));
                    stack.push(bleep);
                    target = Some(left);
                }
                None => {
                    let bleep = (i, None);
                    stack.push(bleep);
                    break;
                }
            }
        }
    }
}

impl<C: Visitor> Iterator for DfsInOrderIter<C> {
    type Item = C::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.a.pop() {
            Some((i, nl)) => match nl {
                Some(nl) => {
                    let res = i;
                    DfsInOrderIter::add_all_lefts(&mut self.a, nl);
                    self.num += 1;
                    Some(res)
                }
                None => Some(i),
            },
            None => None,
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.min_length - self.num,
            self.length.map(|a| a - self.num),
        )
    }
}

impl<C: Visitor> core::iter::FusedIterator for DfsInOrderIter<C> {}
//unsafe impl<C: FixedDepthVisitor> core::iter::TrustedLen for DfsInOrderIter<C> {}
impl<C: FixedDepthVisitor> core::iter::ExactSizeIterator for DfsInOrderIter<C> {}

///Dfs preorder iterator. Each call to next() will return the next element
///in dfs order.
///Internally uses a Vec for the stack.
pub struct DfsPreOrderIter<C: Visitor> {
    a: Vec<C>,
    length: Option<usize>,
    min_length: usize,
    num: usize,
}

impl<C: Visitor> core::iter::FusedIterator for DfsPreOrderIter<C> {}
//unsafe impl<C: FixedDepthVisitor> core::iter::TrustedLen for DfsPreOrderIter<C> {}
impl<C: FixedDepthVisitor> core::iter::ExactSizeIterator for DfsPreOrderIter<C> {}

impl<C: Visitor> Iterator for DfsPreOrderIter<C> {
    type Item = C::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.a.pop() {
            Some(x) => {
                let (i, next) = x.next();
                if let Some([left, right]) = next {
                    self.a.push(right);
                    self.a.push(left);
                }
                self.num += 1;
                Some(i)
            }
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.min_length - self.num,
            self.length.map(|a| a - self.num),
        )
    }
}

/*
///Bfs Iterator. Each call to next() returns the next
///element in bfs order.
///Internally uses a VecDeque for the queue.
pub struct BfsIter<C: Visitor> {
    a: VecDeque<C>,
    a:PhantomData<C>,
    num: usize,
    min_length: usize,
    length: Option<usize>,
}

impl<C: Visitor> core::iter::FusedIterator for BfsIter<C> {}
impl<C: FixedDepthVisitor> core::iter::ExactSizeIterator for BfsIter<C> {}

impl<C: Visitor> Iterator for BfsIter<C> {
    type Item = C::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        
        let queue = &mut self.a;
        match queue.pop_front() {
            Some(e) => {
                let (nn, rest) = e.next();
                if let Some([left, right]) = rest {
                    queue.push_back(left);
                    queue.push_back(right);
                }
                Some(nn)
            }
            None => None,
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.min_length - self.num,
            self.length.map(|a| a - self.num),
        )
    }
}
*/

///Map iterator adapter
pub struct Map<C, F> {
    func: F,
    inner: C,
}
impl<B, C: Visitor, F: Fn(C::Item) -> B + Clone> Visitor for Map<C, F> {
    type Item = B;

    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let (a, rest) = self.inner.next();

        let k = (self.func)(a);
        match rest {
            Some([left, right]) => {
                let ll = Map {
                    func: self.func.clone(),
                    inner: left,
                };
                let rr = Map {
                    func: self.func,
                    inner: right,
                };
                (k, Some([ll, rr]))
            }
            None => (k, None),
        }
    }
}

unsafe impl<B, C: FixedDepthVisitor, F: Fn(C::Item) -> B + Clone> FixedDepthVisitor for Map<C, F> {}

///If implemented, then the level_remaining_hint must return the exact height of the tree.
///If this is implemented, then the exact number of nodes that will be returned by a dfs or bfs traversal is known
///so those iterators can implement TrustedLen in this case.
pub unsafe trait FixedDepthVisitor: Visitor {}


///The trait this crate revoles around.
///A complete binary tree visitor.
pub trait Visitor: Sized {
    ///The common item produced for both leafs and non leafs.
    type Item;

    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self) -> (Self::Item, Option<[Self; 2]>);

    ///Return the levels remaining including the one that will be produced by consuming this iterator.
    ///So if you first made this object from the root for a tree of size 5, it should return 5.
    ///Think of is as height-depth.
    ///This is used to make good allocations when doing dfs and bfs.
    ///Defaults to (0,None)
    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    ///Iterator Adapter to also produce the depth each iteration.
    #[inline]
    fn with_depth(self, start_depth: Depth) -> LevelIter<Self> {
        LevelIter {
            inner: self,
            depth: start_depth,
        }
    }

    ///Combine two tree visitors.
    #[inline]
    fn zip<F: Visitor>(self, f: F) -> Zip<Self, F> {
        Zip { a: self, b: f }
    }

    ///Map iterator adapter
    #[inline]
    fn map<B, F: Fn(Self::Item) -> B>(self, func: F) -> Map<Self, F> {
        Map { func, inner: self }
    }

    ///Only produce children up to num.
    #[inline]
    fn take(self, num: usize) -> Take<Self> {
        Take { a: self, num }
    }

    ///Flips left and right children.
    #[inline]
    fn flip(self) -> Flip<Self> {
        Flip(self)
    }

    /*
    ///Provides an iterator that returns each element in bfs order.
    #[inline]
    fn bfs_iter(self) -> BfsIter<Self> {
        
        
        let (levels, max_levels) = self.level_remaining_hint();

        //Need enough room to fit all the leafs in the queue at once, of which there are n/2.
        let cap = (2u32.pow(levels as u32)) / 2;
        let mut a = VecDeque::with_capacity(cap as usize);

        let min_length = 2usize.pow(levels as u32) - 1;

        let length = max_levels.map(|max_levels| 2usize.pow(max_levels as u32) - 1);

        a.push_back(self);
        BfsIter {
            a,
            min_length,
            length,
            num: 0,
        }
        
    }
    */

    ///Provides a dfs preorder iterator. Unlike the callback version,
    ///This one relies on dynamic allocation for its stack.
    #[inline]
    fn dfs_preorder_iter(self) -> DfsPreOrderIter<Self> {
        let (levels, max_levels) = self.level_remaining_hint();
        let mut a = Vec::with_capacity(levels);

        a.push(self);

        let min_length = 2usize.pow(levels as u32) - 1;
        let length = max_levels.map(|levels_max| 2usize.pow(levels_max as u32) - 1);
        DfsPreOrderIter {
            a,
            length,
            min_length,
            num: 0,
        }
    }
    #[inline]
    fn dfs_inorder_iter(self) -> DfsInOrderIter<Self> {
        let (levels, max_levels) = self.level_remaining_hint();
        let mut a = Vec::with_capacity(levels);

        let length = max_levels.map(|levels_max| 2usize.pow(levels_max as u32) - 1);

        let min_length = 2usize.pow(levels as u32) - 1;

        DfsInOrderIter::add_all_lefts(&mut a, self);

        DfsInOrderIter {
            a,
            min_length,
            length,
            num: 0,
        }
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_preorder(self, mut func: impl FnMut(Self::Item)) {
        rec_pre(self, &mut func);
    }

    ///Calls the closure in dfs preorder (left,right,root).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_inorder(self, mut func: impl FnMut(Self::Item)) {
        rec_inorder(self, &mut func);
    }

    ///Calls the closure in dfs preorder (left,right,root).
    ///Takes advantage of the callstack to do dfs.
    #[inline]
    fn dfs_postorder(self, mut func: impl FnMut(Self::Item)) {
        rec_post(self, &mut func);
    }
}

fn rec_pre<C: Visitor>(a: C, func: &mut impl FnMut(C::Item)) {
    let (nn, rest) = a.next();

    match rest {
        Some([left, right]) => {
            func(nn);
            rec_pre(left, func);
            rec_pre(right, func);
        }
        None => func(nn),
    }
}
fn rec_inorder<C: Visitor>(a: C, func: &mut impl FnMut(C::Item)) {
    let (nn, rest) = a.next();

    match rest {
        Some([left, right]) => {
            rec_inorder(left, func);
            func(nn);
            rec_inorder(right, func);
        }
        None => {
            func(nn);
        }
    }
}
fn rec_post<C: Visitor>(a: C, func: &mut impl FnMut(C::Item)) {
    let (nn, rest) = a.next();

    match rest {
        Some([left, right]) => {
            rec_post(left, func);
            rec_post(right, func);
            func(nn);
        }
        None => {
            func(nn);
        }
    }
}

///Flips left and right children.
pub struct Flip<T: Visitor>(T);
impl<T: Visitor> Visitor for Flip<T> {
    type Item = T::Item;
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let (a, rest) = self.0.next();
        (a, rest.map(|[l, r]| [Flip(r), Flip(l)]))
    }
}
unsafe impl<T: FixedDepthVisitor> FixedDepthVisitor for Flip<T> {}

///Only returns children up untill level num.
pub struct Take<T: Visitor> {
    a: T,
    num: usize,
}

impl<T: Visitor> Visitor for Take<T> {
    type Item = T::Item;

    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let (a, rest) = self.a.next();

        let rest = match rest {
            Some([left, right]) => {
                if self.num == 0 {
                    None
                } else {
                    Some([
                        Take {
                            a: left,
                            num: self.num - 1,
                        },
                        Take {
                            a: right,
                            num: self.num - 1,
                        },
                    ])
                }
            }
            None => None,
        };
        (a, rest)
    }
}

///Tree visitor that zips up two seperate visitors.
///If one of the iterators returns None for its children, this iterator will return None.
pub struct Zip<T1: Visitor, T2: Visitor> {
    a: T1,
    b: T2,
}

impl<T1: Visitor, T2: Visitor> Zip<T1, T2> {
    #[inline]
    pub fn into_inner(self) -> (T1, T2) {
        let Zip { a, b } = self;
        (a, b)
    }
    #[inline]
    pub fn as_inner(&self) -> (&T1, &T2) {
        (&self.a, &self.b)
    }
    #[inline]
    pub fn as_inner_mut(&mut self) -> (&mut T1, &mut T2) {
        (&mut self.a, &mut self.b)
    }
}
impl<T1: Visitor, T2: Visitor> Visitor for Zip<T1, T2> {
    type Item = (T1::Item, T2::Item);

    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let (a_item, a_rest) = self.a.next();
        let (b_item, b_rest) = self.b.next();

        let item = (a_item, b_item);
        match (a_rest, b_rest) {
            (Some(a_rest), Some(b_rest)) => {
                let [aleft, aright] = a_rest;
                let [bleft, bright] = b_rest;

                //let b_rest=b_rest.unwrap();
                let f1 = Zip { a: aleft, b: bleft };
                let f2 = Zip {
                    a: aright,
                    b: bright,
                };
                (item, Some([f1, f2]))
            }
            _ => (item, None),
        }
    }

    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        let a = self.a.level_remaining_hint();
        let b = self.b.level_remaining_hint();
        let min = a.0.min(b.0);

        let min2 = match (a.1, b.1) {
            (Some(a), Some(b)) => Some(a.min(b)),
            _ => None,
        };

        (min, min2)
    }
}
unsafe impl<T1: FixedDepthVisitor, T2: FixedDepthVisitor> FixedDepthVisitor for Zip<T1, T2> {}

#[derive(Copy, Clone)]
///A level descriptor.
pub struct Depth(pub usize);

///A wrapper iterator that will additionally return the depth of each element.
pub struct LevelIter<T> {
    inner: T,
    depth: Depth,
}
impl<T> LevelIter<T> {
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth.0
    }
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
    #[inline]
    pub fn as_inner(&self) -> &T {
        &self.inner
    }
    #[inline]
    pub fn as_inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
impl<T: Visitor> Visitor for LevelIter<T> {
    type Item = (Depth, T::Item);

    #[inline(always)]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let LevelIter { inner, depth } = self;
        let (nn, rest) = inner.next();

        let r = (depth, nn);
        match rest {
            Some([left, right]) => {
                let ln = Depth(depth.0 + 1);
                let ll = LevelIter {
                    inner: left,
                    depth: ln,
                };
                let rr = LevelIter {
                    inner: right,
                    depth: ln,
                };
                (r, Some([ll, rr]))
            }
            None => (r, None),
        }
    }
    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        self.inner.level_remaining_hint()
    }
}
unsafe impl<T: FixedDepthVisitor> FixedDepthVisitor for LevelIter<T> {}
