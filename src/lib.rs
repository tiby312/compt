//!## Summary
//! A library that provides a useful binary tree visitor trait with common default implemenations for visiting strategies such as dfs_inorder or bfs, etc.
//! It also provides two flavors of a complete binary tree data structure with mutable and immutable visitors that implement the visitor trait.
//! One laid out in bfs, and one laid out in dfs in order in memory.
//! However users can still implement their own tree data structures and take advantage of the utility of the binary tree visitor trait.
//! This is thr trait that this crate revoles around:
//!```
//!pub trait CTreeIterator:Sized{
//!    type Item;
//!    type Extra;
//!    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>);
//!}
//!```
//! If you have a visitor of a node, you can call next() on it to consume it, and produce the value of that node, plus
//! the children nodes. Sometimes, non leaf nodes contain additional data that does not apply to leaf nodes. This is 
//! the purpose of the Extra associated type. Users can choose to define it to be some data that only non leaf nodes provide.
//!
//! The fact that the iterator is consumed when calling next(), allows us to return mutable references without fear of the users
//! Being able to create the same mutable reference some other way.
//! So this property provides a way to get mutable references to children nodes simultaneously safely. Useful for parallelizing divide and conquer style problems.
//!
//!## Goals
//!
//! To provide a useful complete binary tree visitor trait. That has some similar features to the Iterator trait,
//! such as zip(), and map().
//!
//! To create a safe and compact complete binary tree data structure that provides an api
//! that parallel algorithms can exploit.
//!

#![feature(ptr_offset_from)]

///A complete binary tree stored in a Vec<T> laid out in bfs order.
pub mod bfs_order;
///A complete binary tree toredd in a Vec<T> laid out in dfs in order.
pub mod dfs_order;

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
    a:Vec<C>
}
impl<C:CTreeIterator> Drop for DfsPreorderIter<C>{
    fn drop(&mut self){
        println!("capacity={:?}",self.a.capacity());
    }
}

impl<C:CTreeIterator> std::iter::FusedIterator for DfsPreorderIter<C>{}
//TODO implement exact size.
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
}


///Bfs Iterator. Each call to next() returns the next
///element in bfs order.
///Internally uses a VecDeque for the queue.
pub struct BfsIter<C:CTreeIterator>{
    a:VecDeque<C>
}
impl<C:CTreeIterator> Drop for BfsIter<C>{
    fn drop(&mut self){
        println!("dequeue capacity={:?}",self.a.capacity());
    }
}

impl<C:CTreeIterator> std::iter::FusedIterator for BfsIter<C>{}

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




pub trait FixedDepthCTreeIterator:CTreeIterator{}


///The trait this crate revoles around.
pub trait CTreeIterator:Sized{
    type Item;
    type Extra;

    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>);

    ///Return the levels remaining including the one that will be produced by consuming this iterator.
    ///So if you first made this object from the root for a tree of size 5, it should return 5.
    ///Think of is as height-depth
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        (0,None)
    }
    ///Iterator Adapter to also produce the depth each iteration. 
    fn with_depth(self,start_depth:Depth)->LevelIter<Self>{
        LevelIter::new(self,start_depth)
    }

    ///Combine two tree visitors.
    fn zip<F:CTreeIterator>(self,f:F)->Zip<Self,F>{
        Zip::new(self,f)
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
        println!("bfs order cap={:?}",a.capacity());
        a.push_back(self);
        BfsIter{a}
    }


    ///Provides a dfs preorder iterator. Unlike the callback version,
    ///This one relies on dynamic allocation for its stack.
    fn dfs_preorder_iter(self)->DfsPreorderIter<Self>{
        let mut v=Vec::with_capacity(self.level_remaining_hint().0);
        v.push(self);
        DfsPreorderIter{a:v}
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

impl<T1:CTreeIterator,T2:CTreeIterator>  Zip<T1,T2>{
    #[inline(always)]
    fn new(a:T1,b:T2)->Zip<T1,T2>{
        Zip{a:a,b:b}
    }
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
///A level descriptor. This is passed to LevelIter.
pub struct Depth(pub usize);

impl Depth{
    #[inline(always)]
    pub fn next_down(&self)->Depth{
        Depth(self.0+1)
    }
}

///A wrapper iterator that will additionally return the depth of each element.
pub struct LevelIter<T>{
    pub inner:T,
    pub depth:Depth
}
impl <T> LevelIter<T>{
    #[inline(always)]
    fn new(a:T,depth:Depth)->LevelIter<T>{
        return LevelIter{inner:a,depth};
    }

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
                let ln=depth.next_down();
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
