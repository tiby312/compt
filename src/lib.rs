//!## Summary
//! A library that provides a complete binary tree visitor trait with default implemenations for visiting strategies such as dfs_inorder or bfs, etc.
//! Some adaptors are also provided that let you map, zip, or optionally also produce the depth on every call to next().
//! It also provides two flavors of a complete binary tree data structure with mutable and immutable visitors that implement the visitor trait.
//! One laid out in bfs, and one laid out in dfs in order in memory. Both of these flavors assume that every node in the tree is the same type.
//! The visitor trait is more flexible than this, however. With the Extra associated type, users can implement a visitor for
//! tree data structures that have different types for the nonleafs and leafs.
//! 
//! This is the trait that this crate revoles around:
//!```
//!pub trait CTreeIterator:Sized{
//!    type Item;
//!    type Extra;
//!    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>);
//!}
//!```
//! If you have a visitor, you can call next() on it to consume it, and produce the node it is visiting, plus
//! the children nodes. Sometimes, non leaf nodes contain additional data that does not apply to leaf nodes. This is 
//! the purpose of the Extra associated type. Users can choose to define it to be some data that only non leaf nodes provide.
//! For the two provided implementations, both leafs and nonleafs have the same time, so in those cases we just use the empty type.
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
//!
//!
//!## Unsafety in the provided two tree implementations
//!
//! With a regular Vec, getting one mutable reference to an element will borrow the
//! entire Vec. However the two provided trees have invariants that let us make guarentees about
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

#![feature(ptr_offset_from)]
#![feature(trusted_len)]

///A complete binary tree stored in a Vec<T> laid out in bfs order.
pub mod bfs_order;
///A complete binary tree stored in a Vec<T> laid out in dfs in order.
pub mod dfs_order;

///Provides functionality to measure the computation time of a tree on a level by level basis.
pub mod timer;

use std::collections::vec_deque::VecDeque;


///Compute the number of nodes in a complete binary tree based on a height.
#[inline(always)]
pub fn compute_num_nodes(height:usize)->usize{
    return (1 << height) - 1;
}


///Dfs iterator. Each call to next() will return the next element
///in dfs order.
///Internally uses a Vec for the stack.
pub struct DfsPreorderIter<C:CTreeIterator>{
    a:Vec<C>,
    level_hint:(usize,Option<usize>)
}


impl<C:CTreeIterator> std::iter::FusedIterator for DfsPreorderIter<C>{}

unsafe impl<C:FixedDepthCTreeIterator> std::iter::TrustedLen for DfsPreorderIter<C>{}
impl<C:FixedDepthCTreeIterator> std::iter::ExactSizeIterator for DfsPreorderIter<C>{}

impl<C:CTreeIterator> Iterator for DfsPreorderIter<C>{
    type Item=(C::Item,Option<C::Extra>);

    fn next(&mut self)->Option<Self::Item>{
        match self.a.pop(){
            Some(x)=>{
                let (i,next)=x.next();
                let extra=match next{
                    Some((extra,left,right))=>{
                        self.a.push(right);
                        self.a.push(left);
                        Some(extra)
                    },
                    _=>{None}
                };

                Some((i,extra))
            },
            None=>{
                None
            }
        }
    }

    fn size_hint(&self)->(usize,Option<usize>){
        let height=self.level_hint.0;
        let len=2usize.pow(height as u32)-1;
        (len,Some(len))
    }
}


///Bfs Iterator. Each call to next() returns the next
///element in bfs order.
///Internally uses a VecDeque for the queue.
pub struct BfsIter<C:CTreeIterator>{
    a:VecDeque<C>,
    level_hint:(usize,Option<usize>)
}


impl<C:CTreeIterator> std::iter::FusedIterator for BfsIter<C>{}
unsafe impl<C:FixedDepthCTreeIterator> std::iter::TrustedLen for BfsIter<C>{}
impl<C:FixedDepthCTreeIterator> std::iter::ExactSizeIterator for BfsIter<C>{}


impl<C:CTreeIterator> Iterator for BfsIter<C>{
    type Item=(C::Item,Option<C::Extra>);
    fn next(&mut self)->Option<Self::Item>{
        let queue=&mut self.a;
        match queue.pop_front(){
            Some(e)=>{
                let (nn,rest)=e.next();
                let extra=match rest{
                    Some((extra,left,right))=>{
                        queue.push_back(left);
                        queue.push_back(right);
                        Some(extra)
                    },
                    None=>{
                        None
                    }
                };
                Some((nn,extra))
            },
            None=>{
                None
            }
        }
    }
    fn size_hint(&self)->(usize,Option<usize>){
        let height=self.level_hint.0;
        let len=2usize.pow(height as u32)-1;
        (len,Some(len))
    }
}

///Map iterator adapter
pub struct Map<C,F>{
    func:F,
    inner:C
}
impl<E,B,C:CTreeIterator,F:Fn(C::Item,Option<C::Extra>)->(B,Option<E>)+Clone> CTreeIterator for Map<C,F>{
    type Item=B;
    type Extra=E;

    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>){
        let (a,rest)=self.inner.next();
        
        match rest{
            Some((extra,left,right))=>{

                let (res,extra)=(self.func)(a,Some(extra));

                let extra=extra.unwrap();

                let ll=Map{func:self.func.clone(),inner:left};
                let rr=Map{func:self.func,inner:right};
                (res,Some((extra,ll,rr)))
            },
            None=>{
                let (res,extra)=(self.func)(a,None);
                assert!(extra.is_none());
                (res,None)
            }
        }
    }
}


///If implemented, then the level_remaining_hint must return the exact height of the tree.
///If this is implemented, then the exact number of nodes that will be returned by a dfs or bfs traversal is known
///so those iterators can implement TrustedLen in this case.
pub unsafe trait FixedDepthCTreeIterator:CTreeIterator{
}



///The trait this crate revoles around.
///A complete binary tree visitor.
pub trait CTreeIterator:Sized{
    ///The common item produced for both leafs and non leafs.
    type Item;
    ///A extra item can be returned for non leafs.
    type Extra;

    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>);

    ///Return the levels remaining including the one that will be produced by consuming this iterator.
    ///So if you first made this object from the root for a tree of size 5, it should return 5.
    ///Think of is as height-depth.
    ///This is used to make good allocations when doing dfs and bfs.
    ///Defaults to (0,None)
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        (0,None)
    }

    ///Iterator Adapter to also produce the depth each iteration. 
    fn with_depth(self,start_depth:Depth)->LevelIter<Self>{
        LevelIter{inner:self,depth:start_depth}
    }

    ///Combine two tree visitors.
    fn zip<F:CTreeIterator>(self,f:F)->Zip<Self,F>{
        Zip{a:self,b:f}
    }

    ///Map iterator adapter
    fn map<B,E,F:Fn(Self::Item,Option<Self::Extra>)->(B,Option<E>)>(self,func:F)->Map<Self,F>{
        Map{func,inner:self}
    }

    ///Provides an iterator that returns each element in bfs order.
    fn bfs_iter(self)->BfsIter<Self>{
        //Need enough room to fit all the leafs in the queue at once, of which there are n/2.
        let cap=(2u32.pow(self.level_remaining_hint().0 as u32))/2;
        let mut a=VecDeque::with_capacity(cap as usize);
        //println!("bfs order cap={:?}",a.capacity());
        let level_hint=self.level_remaining_hint();
        a.push_back(self);
        BfsIter{a,level_hint}
    }


    ///Provides a dfs preorder iterator. Unlike the callback version,
    ///This one relies on dynamic allocation for its stack.
    fn dfs_preorder_iter(self)->DfsPreorderIter<Self>{
        let mut v=Vec::with_capacity(self.level_remaining_hint().0);
        let level_hint=self.level_remaining_hint();
        v.push(self);
        DfsPreorderIter{a:v,level_hint}
    }

    ///Calls the closure in dfs preorder (root,left,right).
    ///Takes advantage of the callstack to do dfs.
    fn dfs_preorder(self,mut func:impl FnMut(Self::Item,Option<Self::Extra>)){
        fn rec<C:CTreeIterator>(a:C,func:&mut impl FnMut(C::Item,Option<C::Extra>)){
            
            let (nn,rest)=a.next();
            
            match rest{
                Some((extra,left,right))=>{
                    func(nn,Some(extra));
                    rec(left,func);
                    rec(right,func);
                },
                None=>{
                    func(nn,None)
                }
            }
        }
        rec(self,&mut func);
    }


    ///Calls the closure in dfs preorder (left,right,root).
    ///Takes advantage of the callstack to do dfs.
    fn dfs_inorder(self,mut func:impl FnMut(Self::Item,Option<Self::Extra>)){
        fn rec<C:CTreeIterator>(a:C,func:&mut impl FnMut(C::Item,Option<C::Extra>)){
            
            let (nn,rest)=a.next();
            
            match rest{
                Some((extra,left,right))=>{
                    rec(left,func);
                    func(nn,Some(extra));
                    rec(right,func);
                },
                None=>{
                    func(nn,None);
                }
            }
        }
        rec(self,&mut func);
    }
}

///Tree visitor that zips up two seperate visitors.
///If one of the iterators returns None for its children, this iterator will return None.
pub struct Zip<T1:CTreeIterator,T2:CTreeIterator>{
    a:T1,
    b:T2,
}


impl<T1:CTreeIterator,T2:CTreeIterator> CTreeIterator for Zip<T1,T2>{
    type Item=(T1::Item,T2::Item);
    type Extra=(T1::Extra,T2::Extra);

    #[inline(always)]
    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>){
        let (a_item,a_rest)=self.a.next();
        let (b_item,b_rest)=self.b.next();

        let item=(a_item,b_item);
        match (a_rest,b_rest){
            (Some(a_rest),Some(b_rest))=>{
                //let b_rest=b_rest.unwrap();
                let f1=Zip{a:a_rest.1,b:b_rest.1};
                let f2=Zip{a:a_rest.2,b:b_rest.2};
                (item,Some(((a_rest.0,b_rest.0),f1,f2)))
            },
            _ =>{
                (item,None)
            }
        }
    }
}


#[derive(Copy,Clone)]
///A level descriptor.
pub struct Depth(pub usize);


///A wrapper iterator that will additionally return the depth of each element.
pub struct LevelIter<T>{
    pub inner:T,
    pub depth:Depth
}

impl<T:CTreeIterator> CTreeIterator for LevelIter<T>{
    type Item=(Depth,T::Item);
    type Extra=T::Extra;
    #[inline(always)]
    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>){
        let LevelIter{inner,depth}=self;
        let (nn,rest)=inner.next();

        let r=(depth,nn);
        match rest{
            Some((extra,left,right))=>{
                let ln=Depth(depth.0+1);
                let ll=LevelIter{inner:left,depth:ln};
                let rr=LevelIter{inner:right,depth:ln};
                (r,Some((extra,ll,rr)))
            },
            None=>{
                (r,None)
            }
        }
    }

}
