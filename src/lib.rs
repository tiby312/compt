//!## Summary
//! A Complete Binary Tree library.
//! It is internally represented as a 1D vec.
//! Provides a way to get mutable references to children nodes simultaneously. Useful for parallelizing divide and conquer style problems.
//! There is no api to add and remove nodes. The existence of the tree implies that 2k-1 elements already exist. It is a full tree.
//! Provides tree visitors that implement the below trait. They can be combined together using zip().
//!
//!```
//!pub trait CTreeIterator:Sized{
//!    type Item;
//!    ///Consume this visitor, and produce the element it was pointing to
//!    ///along with it's children visitors.
//!    fn next(self)->(Self::Item,Option<(Self,Self)>);
//!}
//!```
//!
//!## Goals
//!
//!To create a safe and compact complete binary tree data structure that provides an api
//!that parallel algorithms can exploit.
//!
//!## Unsafety
//!
//!With a regular slice, getting one mutable reference to an element will borrow the
//!entire slice. The slice that GenTree uses, however, internally has the invariant that it is laid out
//!in BFS order. Therefore one can safely assume that if (starting at the root),
//!one had a mutable reference to a parent k, and one were to get the children using 2k+1 and 2k+2
//!to get *two* mutable references to the children,
//!they would be guarenteed to be distinct (from each other and also the parent) despite the fact that they belong to the same slice.
//!
//!## Example
//!```
//!extern crate compt;
//!fn main()
//!{
//!        use compt::CTreeIterator;
//!        //Create a tree of height 2 with elemenets set to zero.
//!        let mut tree=compt::GenTree::from_bfs(||0,2);
//!        {
//!            //Create a mutable tree visitor.
//!            let mut down=tree.create_down_mut();
//!            //Call the iterator's next() function.
//!            let (e,maybe_children)=down.next();
//!            //Set the root to 1.
//!            *e=1;
//!            //Set the children to 2 and 3.
//!            let (mut left,mut right)=maybe_children.unwrap();
//!            *left.next().0=2;
//!            *right.next().0=3;
//!        }
//!        {
//!            //Create a immutable tree visitor.
//!            let down=tree.create_down();
//!            //Iterate dfs over our constructed tree.
//!            let mut v=Vec::new();
//!            down.dfs_postorder(|a|{
//!                 v.push(*a);
//!            });
//!            assert_eq!(v,vec!(3,2,1));
//!        }
//!}
//!```
//!


pub mod bfs_order;
pub mod dfs_order;


//extern crate smallvec;
use std::collections::vec_deque::VecDeque;
//use smallvec::SmallVec;


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


//TODO implement exact size???
impl<C:CTreeIterator> Iterator for DfsPreorderIter<C>{
    type Item=(C::Item,Option<C::Extra>);

    fn next(&mut self)->Option<Self::Item>{
        match self.a.pop(){
            Some(x)=>{
                let (i,next)=x.next();
                let extra=match next{
                    Some((extra,left,right))=>{
                        self.a.push(left);
                        self.a.push(right);
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

impl<C:CTreeIterator> Iterator for BfsIter<C>{
    type Item=(C::Item,Option<C::Extra>);
    fn next(&mut self)->Option<Self::Item>{
        let queue=&mut self.a;
        match queue.pop_front(){
            Some(e)=>{
                let (nn,rest)=e.next();
                //func(nn);
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

                //Fn Closures don't automatically implement clone.
                let ll=Map{func:self.func.clone(),inner:left};
                let rr=Map{func:self.func,inner:right};

                /*
                let (ll,rr)=unsafe{
                    let mut ll:Map<C,F>=std::mem::uninitialized();
                    let mut rr:Map<C,F>=std::mem::uninitialized();
                    ll.inner=left;
                    rr.inner=right;
                    std::ptr::copy(&self.func,&mut ll.func,1);
                    std::ptr::copy(&self.func,&mut rr.func,1);
                    (ll,rr)
                };
                */
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


/*

//TODO use this!!!!
pub mod par{
    use super::*;

    use super::*;
    pub trait AxisTrait:Copy+Send{
        type Next:AxisTrait;
        fn next(&self)->Self::Next;
        fn is_xaxis(&self)->bool;
    }


    #[derive(Copy,Clone,Debug)]
    pub struct XAXIS;

    #[derive(Copy,Clone,Debug)]
    pub struct YAXIS;
    impl AxisTrait for XAXIS{
        type Next=YAXIS;
        fn next(&self)->YAXIS{
            YAXIS
        }
        fn is_xaxis(&self)->bool{
            true
        }
    }
    impl AxisTrait for YAXIS{
        type Next=XAXIS;
        fn next(&self)->XAXIS{
            XAXIS
        }
        fn is_xaxis(&self)->bool{
            false
        }
    }

    //TODO put htis in the iterator trait
    pub fn in_preorder_parallel<Y:Send,X:Send,T:CTreeIterator<Item=X>+Send,F1:Fn(X,Y)->(Y,Y)+Sync,F2:Fn(Y,Y)->Y+Sync,F3:Fn(X,Y)->Y+Sync>(
            it:T,func:F1,f2:F2,f3:F3,depth_to_switch:Depth,val:Y)->Y{


        fn recc<Y:Send,X:Send,T:CTreeIterator<Item=(Depth,X)>+Send,F1:Fn(X,Y)->(Y,Y)+Sync,F2:Fn(Y,Y)->Y+Sync,F3:Fn(X,Y)->Y+Sync>(s:T,depth_to_switch:Depth,func:&F1,f2:&F2,f3:&F3,val:Y)->Y{
            let ((depth,nn),rest)=s.next();
            
            match rest{
                Some((left,right))=>{

                    let (yl,yr)=func(nn,val);
                    /*
                    let (a,b)=if depth_to_switch.0>=depth.0{
                        rayon::join(
                            move ||recc(left,depth,func,f2,f3,yl),
                            move ||recc(right,depth,func,f2,f3,yr)
                        )
                    }else{
                        */
                     let (a,b)=   (recc(left,depth,func,f2,f3,yl),
                        recc(right,depth,func,f2,f3,yr));
                    //};

                    f2(a,b)
                },
                None=>{
                    f3(nn,val)
                }
            }
        }

        let level=LevelIter::new(it,Depth(0));
        recc(level,depth_to_switch,&func,&f2,&f3,val)
    }
}*/

/*
pub trait CIter{
    type Visitor:CTreeIterator<Item=Self::Item>;
    type Item;

    ///Calls the closure in dfs preorder (left,right,root).
    ///Takes advantage of the callstack to do dfs.
    fn dfs_inorder(&self,a:Self::Visitor,mut func:impl FnMut(Self::Item)){
        fn rec<C:CIter+?Sized>(itt:&C,a:C::Visitor,func:&mut impl FnMut(C::Item)){
            
            let (nn,rest)=a.next();
            
            match rest{
                Some((left,right))=>{
                    rec(itt,left,func);
                    func(nn);
                    rec(itt,right,func);
                },
                None=>{
                    func(nn);
                }
            }
        }
        rec(self,a,&mut func);
    }
    /*
    fn dfs_inorder_iter<X:Iterator<Item=isize>>(&self,a:Self::Visitor,mut func:impl FnMut(Self::Item))->impl Iterator<Item=Self::Item>{
        unimplemented!()
    }
    */

    fn bfs(&self,aa:Self::Visitor,mut func:impl FnMut(Self::Item)){
        let mut a=VecDeque::new();
        a.push_back(aa);
        let b=BfsIter{a};
        for i in b{
            func(i);
        }
    }
}
*/


//TODO enhance to use this!!
/*
pub trait ExactSizeCTreeIterator:CTreeIterator{
    fn height(&self)->usize;
}
*/

///All binary tree visitors implement this.
pub trait CTreeIterator:Sized{
    type Item;
    type Extra;

    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>);

    fn with_depth(self,start_depth:Depth)->LevelIter<Self>{
        LevelIter::new(self,start_depth)
    }

    fn with_extra<F:Fn(&Self::Item,X)->(X,X)+Copy,X:Clone>(self,func:F,extra:X)->Extra<F,X,Self>{
        Extra{c:self,extra,func}
    }

    ///Combine two tree visitors.
    fn zip<F:CTreeIterator>(self,f:F)->Zip<Self,F>{
        Zip::new(self,f)
    }

    fn map<B,E,F:Fn(Self::Item,Option<Self::Extra>)->(B,Option<E>)>(self,func:F)->Map<Self,F>{
        Map{func,inner:self}
    }


    ///Provides an iterator that returns each element in bfs order.
    ///A callback version is not provided because a queue would still need to be used,
    ///So it wouldnt be able to take advantage of the stack anyway.
    fn bfs_iter(self)->BfsIter<Self>{
        let mut a=VecDeque::new();
        a.push_back(self);
        BfsIter{a}
    }

    ///Provides a dfs preorder iterator. Unlike the callback version,
    ///This one relies on dynamic allocation for its queue.
    fn dfs_preorder_iter(self)->DfsPreorderIter<Self>{
        let mut v=Vec::new();
        v.push(self);
        DfsPreorderIter{a:v}
    }

    ///Calls the closure in dfs preorder (left,right,root).
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




/*
#[derive(Copy,Clone,Debug)]
pub enum TLeaf{
    IsNotLeaf = 0,
    IsLeaf = 1
}

pub struct IsLeaf<C:CTreeIterator>(C);


impl<C:CTreeIterator> CTreeIterator for IsLeaf<C>{
    type Item=(TLeaf,C::Item);
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let (n,rest)=self.0.next();
        
        match rest{
            Some((left,right))=>{
                let left=IsLeaf(left);
                let right=IsLeaf(right);
                ((TLeaf::IsNotLeaf,n),Some((left,right)))
            },
            None=>{
                ((TLeaf::IsLeaf,n),None)
            }
        }
    }
}

*/

/*
#[derive(Copy,Clone,Debug)]
pub enum TAxis{
    XAXIS=0,
    YAXIS=1
}

impl TAxis{
    pub fn is_xaxis(&self)->bool{
        match self{
            &TAxis::XAXIS=>{
                true
            },
            &TAxis::YAXIS=>{
                false
            }

        }
    }
}

pub struct AxisIter<C:CTreeIterator>{
    is_xaxis:TAxis,
    c:C
}
impl<C:CTreeIterator> AxisIter<C>{
    fn new(is_xaxis:TAxis,c:C)->AxisIter<C>{
        AxisIter{is_xaxis,c}
    }
}
impl<C:CTreeIterator> CTreeIterator for AxisIter<C>{
    type Item=(TAxis,C::Item);
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let (n,rest)=self.c.next();
        let nn=(self.is_xaxis,n);

        match rest{
            Some((left,right))=>{
                let is_xaxis=match self.is_xaxis{
                    TAxis::XAXIS=>{
                        TAxis::YAXIS
                    },
                    TAxis::YAXIS=>{
                        TAxis::XAXIS
                    }
                };
                //let is_xaxis=!(self.is_xaxis);
                let left=AxisIter{is_xaxis,c:left};
                let right=AxisIter{is_xaxis,c:right};
                (nn,Some((left,right)))
            },
            None=>{
                (nn,None)
            }
        }
    }
}

*/

pub use extra::Extra;
mod extra{
    use super::*;       

    pub struct Extra<F:Fn(&C::Item,X)->(X,X)+Copy,X:Clone,C:CTreeIterator>{
        pub c:C,
        pub extra:X,
        pub func:F
    }


    impl<F:Fn(&C::Item,X)->(X,X)+Copy,X:Clone,C:CTreeIterator> CTreeIterator for Extra<F,X,C>{
        type Item=(X,C::Item);
        type Extra=C::Extra;
        fn next(self)->(Self::Item,Option<(Self::Extra,Self,Self)>){
            
            
            let (mut n,rest)=self.c.next();
            
            let ex=self.extra.clone();

            match rest{
                Some((extra,left,right))=>{

                    let (a,b)=(self.func)(&mut n,self.extra);
                    
                    let left=Extra{c:left,extra:a,func:self.func};
                    
                    let right=Extra{c:right,extra:b,func:self.func};
                    ((ex,n),Some((extra,left,right)))
                },
                None=>{
                    ((ex,n),None)
                }
            }
        }
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

/*
pub use wrap::WrapGen;

mod wrap{
    use super::*;
    //TODO remove this!!! make users use the concrete wrappers instead.
    ///Allows to traverse down from a visitor twice by creating a new visitor that borrows the other.
    pub struct WrapGen<'a,T:CTreeIterator+'a>{
        a:T,
        _p:PhantomData<&'a mut T>
    }
    impl<'a,T:CTreeIterator+'a> WrapGen<'a,T>{
        #[inline(always)]
        pub fn new(a:&'a mut T)->WrapGen<'a,T>{
            let ff=unsafe{
                let mut ff=std::mem::uninitialized();
                std::ptr::copy(a, &mut ff, 1);
                ff
            };
            WrapGen{a:ff,_p:PhantomData}
        }
    }
    
    pub struct Bo<'a,T:'a>{
        a:T,
        _p:PhantomData<&'a mut T>
    }
    
    impl<'a,T:'a> Bo<'a,T>{

        pub fn get_mut (&mut self)->&mut T{
            &mut self.a
        }
        pub fn get(&self)->&T{
            &self.a
        }
    }

    impl<'a,T:CTreeIterator+'a> CTreeIterator for WrapGen<'a,T>{
        type Item=Bo<'a,T::Item>;
        #[inline(always)]
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            let WrapGen{a,_p}=self;
  
            let (item,mm)=a.next();
            let item=Bo{a:item,_p:PhantomData};
            match mm{
                Some((left,right))=>{
                    let left=WrapGen{a:left,_p:PhantomData};
                    let right=WrapGen{a:right,_p:PhantomData};
                    return (item,Some((left,right)));
                },
                None=>{
                    return (item,None);
                }
            }
        }
    }
}*/




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
