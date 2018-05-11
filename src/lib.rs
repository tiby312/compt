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

extern crate rayon;
extern crate smallvec;
use std::collections::vec_deque::VecDeque;
use smallvec::SmallVec;

pub mod dfs{
    use super::*;

    ///Visitor functions use this type to determine what node to visit.
    ///The nodes in the tree are kept in the tree in BFS order.
    #[derive(Copy,Clone,Debug)]
    struct NodeIndexDfs(usize);

    impl NodeIndexDfs{
        #[inline(always)]
        fn get_children(self,diff:usize) -> (NodeIndexDfs, NodeIndexDfs) {
            //println!("id={:?}",self.0);

            //000000000000000
            //       0
            //   0       0
            // 0   0   0   0
            //0 0 0 0 0 0 0 0

            let NodeIndexDfs(a) = self;
            (NodeIndexDfs(a-diff), NodeIndexDfs(a+diff))
        }
    }


    use std::marker::PhantomData;

    pub struct GenTreeDfsOrder<T>{
        nodes: Vec<T>,
        height:usize
    }
    impl<T> GenTreeDfsOrder<T>{


        #[inline(always)]
        pub fn from_vec(vec:Vec<T>,height:usize)->Result<GenTreeDfsOrder<T>,&'static str>{
            if 2_usize.pow(height as u32)==vec.len()+1{
                Ok(GenTreeDfsOrder{nodes:vec,height})
            }else{
                Err("Not a power of two")
            }
        }

        #[inline(always)]
        pub fn get_height(&self) -> usize {
            self.height
        }

        ///Create a complete binary tree using the specified node generating function.
        
        #[inline(always)]
        pub fn from_dfs_inorder<F:FnMut()->T>(mut func:F,height:usize)->GenTreeDfsOrder<T>{
            let num=compute_num_nodes(height);
            let mut nodes=Vec::with_capacity(num);
            for _ in 0..num{
                nodes.push(func());
            }
            GenTreeDfsOrder{nodes,height}
        }

        pub fn dfs_inorder_iter(&self)->std::slice::Iter<T>{
            self.nodes.iter()
        }

        pub fn dfs_inorder_iter_mut(&mut self)->std::slice::IterMut<T>{
            self.nodes.iter_mut()
        }
        #[inline(always)]
        pub fn get_nodes(&self)->&[T]{
            &self.nodes
        }
        #[inline(always)]
        pub fn create_down(&self)->DownT<T>{
            let k=self.nodes.len()+1;
            DownT{remaining:self,nodeid:NodeIndexDfs(self.nodes.len()/2),span:k/4}
        }

        #[inline(always)]
        pub fn create_down_mut(&mut self)->DownTMut<T>{
            let k=self.nodes.len()+1;
            DownTMut{remaining:self,nodeid:NodeIndexDfs(self.nodes.len()/2),span:k/4,phantom:PhantomData}
        }

        #[inline(always)]
        ///Create a Depth that can be passed to a LevelIter.
        pub fn get_level_desc(&self)->Depth{
            Depth(0)
        }
        
    }

    ///Tree visitor that returns a reference to each element in the tree.
    pub struct DownT<'a,T:'a>{
        remaining:&'a GenTreeDfsOrder<T>,
        nodeid:NodeIndexDfs,
        span:usize,
    }
    impl<'a,T:'a> DownT<'a,T>{
        pub fn create_wrap<'b>(&'b self)->DownT<'b,T>{
            DownT{remaining:self.remaining,nodeid:self.nodeid,span:self.span}
        }
    }

    impl<'a,T:'a> CTreeIterator for DownT<'a,T>{
        type Item=&'a T;
        #[inline(always)]
        fn next(self)->(Self::Item,Option<(Self,Self)>){
     
            let a=&self.remaining.nodes[self.nodeid.0];
            
            if self.span==0{
                (a,None)
            }else{
                let (l,r)=self.nodeid.get_children(self.span);
                
                let j=(     
                    DownT{remaining:self.remaining,nodeid:l,span:self.span/2},
                    DownT{remaining:self.remaining,nodeid:r,span:self.span/2}
                );
                (a,Some(j))
            }
        }
     

    }

    ///Tree visitor that returns a mutable reference to each element in the tree.
    pub struct DownTMut<'a,T:'a>{
        remaining:*mut GenTreeDfsOrder<T>,
        nodeid:NodeIndexDfs,
        span:usize,
        phantom:PhantomData<&'a mut T>
    }

    unsafe impl<'a,T:Send+'a> std::marker::Send for DownTMut<'a,T>{}


    impl<'a,T:'a> DownTMut<'a,T>{
        pub fn create_wrap_mut<'b>(&'b mut self)->DownTMut<'b,T>{
            DownTMut{remaining:self.remaining,nodeid:self.nodeid,span:self.span,phantom:self.phantom}
        }
    }
    impl<'a,T:'a> CTreeIterator for DownTMut<'a,T>{
        type Item=&'a mut T;
        #[inline(always)]
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            
            //Unsafely get a mutable reference to this nodeid.
            //Since at the start there was only one DownTMut that pointed to the root,
            //there is no danger of two DownTMut's producing a reference to the same node.
            let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
            if self.span==0{
                (a,None)
            }else{
                //let node_len=unsafe{(*self.remaining).nodes.len()};

                let (l,r)=self.nodeid.get_children(self.span);
                //println!("id={:?} span={:?} children={:?}",self.nodeid.0,self.span,(l,r));
                let j=(     
                    DownTMut{remaining:self.remaining,nodeid:l,span:self.span/2,phantom:PhantomData},
                    DownTMut{remaining:self.remaining,nodeid:r,span:self.span/2,phantom:PhantomData}
                );
                (a,Some(j))
            }
        }
    }

}

///The complete binary tree. Internally stores the elements in a Vec<T> so it is very compact.
///Height is atleast 1.
///Elements stored in BFS order.
///Has 2<sup>k-1</sup> elements where k is the height.
pub struct GenTree<T:Send> {
    nodes: Vec<T>,
    height: usize,
}

///Compute the number of nodes in a complete binary tree based on a height.
#[inline(always)]
pub fn compute_num_nodes(height:usize)->usize{
    return (1 << height) - 1;
}

impl<T:Send> GenTree<T> {
    
    /*
    pub fn from_parr<M:par::Makerr2<Item=T>>()->GenTree<T>{
        //TODO finish
        unimplemented!();
    }
    */
    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.height
    }

    ///Create a complete binary tree using the specified node generating function.
    pub fn from_dfs<F:FnMut()->T>(mut func:F,height:usize)->GenTree<T>{
        assert!(height>=1);
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        {
            let t=tree.create_down_mut();
            t.dfs_preorder(|node:&mut T|{
                *node=func();
            });
        }
        tree
    }

    ///Create a complete binary tree using the specified node generating function.
    pub fn from_dfs_backwards<F:FnMut()->T>(mut func:F,height:usize)->GenTree<T>{
        assert!(height>=1);
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        {
            let t=tree.create_down_mut();
            t.dfs_postorder(|node:&mut T|{
                *node=func();
            });
        }
        tree
    }

    ///Create a complete binary tree using the specified node generating function.
    pub fn from_bfs<F:FnMut()->T>(mut func:F,height:usize)->GenTree<T>{
        assert!(height>=1);
        let num_nodes=self::compute_num_nodes(height);

        let mut vec=Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes{
            vec.push(func())
        }
        GenTree{
            nodes:vec,
            height:height,
        }
    }

    ///Visit every node in BFS order.
    ///Due to underlying representation of the tree, this is just a fast loop.
    pub fn bfs<F:FnMut(&T)>(&self,mut func:F){
        for i in self.nodes.iter(){
            func(i);
        }
    }

    ///Visit every node in BFS order.
    ///Due to underlying representation of the tree, this is just a fast loop.
    pub fn bfs_mut<F:FnMut(&mut T)>(&mut self,mut func:F){
        for i in self.nodes.iter_mut(){
            func(i);
        }
    }
    
    #[inline(always)]
    ///Create a Depth that can be passed to a LevelIter.
    pub fn get_level_desc(&self)->Depth{
        Depth(0)
    }
    
    #[inline(always)]
    ///Create a immutable visitor struct
    pub fn create_down(&self)->DownT<T>{
        let k=DownT{remaining:self,nodeid:NodeIndex(0),first_leaf:NodeIndex::first_leaf(self.nodes.len())};
        k
    }

    #[inline(always)]
    ///Create a mutable visitor struct
    pub fn create_down_mut(&mut self)->DownTMut<T>{
        let k=DownTMut{remaining:self,nodeid:NodeIndex(0),first_leaf:NodeIndex::first_leaf(self.nodes.len()),phantom:PhantomData};
        k
    }

    #[inline(always)]
    ///Consume the tree and return each element to the user in dfs order.
    pub fn into_dfs_preorder<F:FnMut(T)>(self,func:F){
        cons::downt_into_dfs_preorder(self,func);
    }

    #[inline(always)]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn get_nodes(&self)->&[T]{
        &self.nodes
    }

    #[inline(always)]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self)->Vec<T>{
        let GenTree{nodes,height:_}=self;
        nodes
    }
}



///Visitor functions use this type to determine what node to visit.
///The nodes in the tree are kept in the tree in BFS order.
#[derive(Copy,Clone,Debug)]
struct NodeIndex(usize);

impl NodeIndex{
    #[inline(always)]
    fn get_children(self) -> (NodeIndex, NodeIndex) {
        let NodeIndex(a) = self;
        (NodeIndex(2 * a + 1), NodeIndex(2 * a + 2))
    }
    #[inline(always)]
    fn first_leaf(nodes:usize)->NodeIndex{
        NodeIndex(nodes/2)
    }
}



///Dfs iterator. Each call to next() will return the next element
///in dfs order.
///Internally uses a Vec for the stack.
pub struct DfsPreorderIter<C:CTreeIterator>{
    a:SmallVec<[C;16]>
}

impl<C:CTreeIterator> Iterator for DfsPreorderIter<C>{
    type Item=C::Item;
    fn next(&mut self)->Option<Self::Item>{
        match self.a.pop(){
            Some(x)=>{
                let (i,next)=x.next();
                match next{
                    Some((left,right))=>{
                        self.a.push(left);
                        self.a.push(right);
                    },
                    _=>{}
                }
                Some(i)
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
    type Item=C::Item;
    fn next(&mut self)->Option<Self::Item>{
        let queue=&mut self.a;
        match queue.pop_front(){
            Some(e)=>{
                let (nn,rest)=e.next();
                //func(nn);
                match rest{
                    Some((left,right))=>{
                        queue.push_back(left);
                        queue.push_back(right);
                    },
                    None=>{

                    }
                }
                Some(nn)
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
impl<B,C:CTreeIterator,F:Fn(C::Item)->B> CTreeIterator for Map<C,F>{
    type Item=B;
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let (a,rest)=self.inner.next();
        
        let res=(self.func)(a);
        match rest{
            Some((left,right))=>{
                
                //Fn Closures don't automatically implement clone.
                let (ll,rr)=unsafe{
                    let mut ll:Map<C,F>=std::mem::uninitialized();
                    let mut rr:Map<C,F>=std::mem::uninitialized();
                    ll.inner=left;
                    rr.inner=right;
                    std::ptr::copy(&self.func,&mut ll.func,1);
                    std::ptr::copy(&self.func,&mut rr.func,1);
                    (ll,rr)
                };
                (res,Some((ll,rr)))
            },
            None=>{
                (res,None)
            }
        }
    }
}




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
}

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

///All binary tree visitors implement this.
pub trait CTreeIterator:Sized{
    type Item;

    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self)->(Self::Item,Option<(Self,Self)>);

    fn with_axis(self,xaxis:TAxis)->AxisIter<Self>{
        AxisIter::new(xaxis,self)
    }

    fn with_leaf(self)->IsLeaf<Self>{
        IsLeaf(self)
    }

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

    fn map<B,F:Fn(Self::Item)->B>(self,func:F)->Map<Self,F>{
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
        let mut v=SmallVec::new();
        v.push(self);
        DfsPreorderIter{a:v}
    }

    ///Calls the closure in dfs preorder (left,right,root).
    ///Takes advantage of the callstack to do dfs.
    fn dfs_preorder(self,mut func:impl FnMut(Self::Item)){
        fn rec<C:CTreeIterator>(a:C,func:&mut impl FnMut(C::Item)){
            
            let (nn,rest)=a.next();
            func(nn);
            match rest{
                Some((left,right))=>{
                    rec(left,func);
                    rec(right,func);
                },
                None=>{

                }
            }
        }
        rec(self,&mut func);
    }


    ///Calls the closure in dfs preorder (left,right,root).
    ///Takes advantage of the callstack to do dfs.
    fn dfs_inorder(self,mut func:impl FnMut(Self::Item)){
        fn rec<C:CTreeIterator>(a:C,func:&mut impl FnMut(C::Item)){
            
            let (nn,rest)=a.next();
            
            match rest{
                Some((left,right))=>{
                    rec(left,func);
                    func(nn);
                    rec(right,func);
                },
                None=>{
                    func(nn);
                }
            }
        }
        rec(self,&mut func);
    }


    ///Calls the closure in dfs postorder (right,left,root).
    ///Takes advantage of the callstack to do dfs.
    fn dfs_postorder(self,mut func:impl FnMut(Self::Item)){
        fn rec<C:CTreeIterator>(a:C,func:&mut impl FnMut(C::Item)   ){
            
            let (nn,rest)=a.next();
            match rest{
                Some((left,right))=>{
                    rec(right,func);
                    rec(left,func);
                },
                None=>{

                }
            }
            func(nn);
        }
        rec(self,&mut func);
    }
    /*
    fn dfs_inorder_axis<A:par::AxisTrait,AC:AxisCallback<Item=Self::Item>>(self,axis:A,mut callback:AC){
        fn recc<X,C:CTreeIterator<Item=X>,AC:AxisCallback<Item=X>,A:par::AxisTrait>(a:C,mut b:&mut AC,axis:A){

            let (n,rest)=a.next();
            
            b.handle::<A>(n);

            match rest{
                Some((left,right))=>{
                    recc(left,b,axis.next());
                    recc(right,b,axis.next());
                },
                None=>{

                }
            }
        }

        recc(self,&mut callback,axis);
    }*/
}





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



pub use extra::Extra;
mod extra{
    use super::*;       

    pub struct Extra<F:Fn(&C::Item,X)->(X,X)+Copy,X:Clone,C:CTreeIterator>{
        pub c:C,
        pub extra:X,
        pub func:F
    }

    /*
    fn clone_func<CI,F:Fn(&CI,X)->(X,X)+Copy,X:Clone>(func:&F)->F{
        unsafe{
            let mut k=std::mem::uninitialized();
            std::ptr::copy(func,&mut k,1);
            k
        }
    }*/

    impl<F:Fn(&C::Item,X)->(X,X)+Copy,X:Clone,C:CTreeIterator> CTreeIterator for Extra<F,X,C>{
        type Item=(X,C::Item);
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            
            
            let (mut n,rest)=self.c.next();
            
            let ex=self.extra.clone();

            match rest{
                Some((left,right))=>{

                    let (a,b)=(self.func)(&mut n,self.extra);
                    
                    let left=Extra{c:left,extra:a,func:self.func};
                    
                    let right=Extra{c:right,extra:b,func:self.func};
                    ((ex,n),Some((left,right)))
                },
                None=>{
                    ((ex,n),None)
                }
            }
        }
    }
}



use std::marker::PhantomData;

///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:Send+'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    first_leaf:NodeIndex,
    phantom:PhantomData<&'a mut T>
}

unsafe impl<'a,T:Send+'a> std::marker::Send for DownTMut<'a,T>{}

impl<'a,T:Send+'a> CTreeIterator for DownTMut<'a,T>{
    type Item=&'a mut T;
    #[inline(always)]
    fn next(self)->(Self::Item,Option<(Self,Self)>){
 
        //Unsafely get a mutable reference to this nodeid.
        //Since at the start there was only one DownTMut that pointed to the root,
        //there is no danger of two DownTMut's producing a reference to the same node.
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        if self.nodeid.0>=self.first_leaf.0{
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                DownTMut{remaining:self.remaining,nodeid:l,first_leaf:self.first_leaf,phantom:PhantomData},
                DownTMut{remaining:self.remaining,nodeid:r,first_leaf:self.first_leaf,phantom:PhantomData}
            );
            (a,Some(j))
        }
    }
}


///Tree visitor that returns a reference to each element in the tree.
pub struct DownT<'a,T:Send+'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    first_leaf:NodeIndex,
}

impl<'a,T:Send+'a> CTreeIterator for DownT<'a,T>{
    type Item=&'a T;
    #[inline(always)]
    fn next(self)->(Self::Item,Option<(Self,Self)>){
 
        let a=&self.remaining.nodes[self.nodeid.0];
        
        if self.nodeid.0>=self.first_leaf.0{
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                DownT{remaining:self.remaining,nodeid:l,first_leaf:self.first_leaf},
                DownT{remaining:self.remaining,nodeid:r,first_leaf:self.first_leaf}
            );
            (a,Some(j))
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
    #[inline(always)]
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let (a_item,a_rest)=self.a.next();
        let (b_item,b_rest)=self.b.next();

        let item=(a_item,b_item);
        match (a_rest,b_rest){
            (Some(a_rest),Some(b_rest))=>{
                //let b_rest=b_rest.unwrap();
                let f1=Zip{a:a_rest.0,b:b_rest.0};
                let f2=Zip{a:a_rest.1,b:b_rest.1};
                (item,Some((f1,f2)))
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


mod cons{
    use super::*;
    struct DownTConsume<'a,T:Send+'a>{
        remaining:*mut GenTree<T>,
        nodeid:NodeIndex,
        first_leaf:NodeIndex,
        phantom:PhantomData<&'a T>
    }

    pub fn downt_into_dfs_preorder<T:Send,F:FnMut(T)>(mut tree:GenTree<T>,func:F){
        {
            let t=DownTConsume{remaining:&mut tree,nodeid:NodeIndex(0),first_leaf:NodeIndex::first_leaf(tree.nodes.len()),phantom:PhantomData};
            t.dfs_preorder(func);
        }
        for a in tree.nodes.drain(..){
            std::mem::forget(a);
        }
    }

    impl<'a,T:Send+'a> CTreeIterator for DownTConsume<'a,T>{
        type Item=T;
        #[inline(always)]
        fn next(self)->(Self::Item,Option<(Self,Self)>){
     
            //Unsafely copy each element and give it to the user.
            //We will make sure not to call drop() on the source
            //after we iterate through all of the tree.
            let mut val=unsafe{std::mem::uninitialized()};
            let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
            unsafe{std::ptr::copy(&mut val,a,1)};

            if self.nodeid.0>=self.first_leaf.0{
                (val,None)
            }else{
     
                let (l,r)=self.nodeid.get_children();
                
                let j=(     
                    DownTConsume{remaining:self.remaining,nodeid:l,first_leaf:self.first_leaf,phantom:PhantomData},
                    DownTConsume{remaining:self.remaining,nodeid:r,first_leaf:self.first_leaf,phantom:PhantomData}
                );
                (val,Some(j))
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


#[test]
fn testy(){
    let mut k=dfs::GenTreeDfsOrder::from_dfs_inorder(||0,5);

    let j=k.create_down_mut();

    let k:LevelIter<dfs::DownTMut<usize>>=j.with_depth();//LevelIter<NdIter<T>>;


    {
        let (depth,bla)=k.into_inner();
        
        let wrap=bla.create_wrap().with_depth(depth);
    }

}


///A wrapper iterator that will additionally return the depth of each element.
pub struct LevelIter<T:CTreeIterator>{
    pub inner:T,
    leveld:Depth
}
impl <T:CTreeIterator> LevelIter<T>{
    #[inline(always)]
    fn new(a:T,leveld:Depth)->LevelIter<T>{
        return LevelIter{inner:a,leveld};
    }

}


impl<T:CTreeIterator> CTreeIterator for LevelIter<T>{
    type Item=(Depth,T::Item);
    #[inline(always)]
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let LevelIter{inner,leveld}=self;
        let (nn,rest)=inner.next();

        let r=(leveld,nn);
        match rest{
            Some((left,right))=>{
                let ln=leveld.next_down();
                let ll=LevelIter{inner:left,leveld:ln};
                let rr=LevelIter{inner:right,leveld:ln};
                (r,Some((ll,rr)))
            },
            None=>{
                (r,None)
            }
        }
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //TODO!
    }
}
