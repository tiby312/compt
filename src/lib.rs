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

///The complete binary tree. Internally stores the elements in a Vec<T> so it is very compact.
///Height is atleast 1.
///Elements stored in BFS order.
///Has 2^k-1 elements where k is the height.
pub struct GenTree<T> {
    nodes: Vec<T>,
    height: usize,
}

///Compute the number of nodes in a complete binary tree based on a height.
#[inline(always)]
pub fn compute_num_nodes(height:usize)->usize{
    return (1 << height) - 1;
}

impl<T> GenTree<T> {
    
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
    ///Create a LevelDesc that can be passed to a LevelIter.
    pub fn get_level_desc(&self)->LevelDesc{
        LevelDesc{depth:0}
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
    fn first_leaf(nodes:usize)->NodeIndex{
        NodeIndex(nodes/2)
    }
}



///All binary tree visitors implement this.
pub trait CTreeIterator:Sized{
    type Item;

    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self)->(Self::Item,Option<(Self,Self)>);

    ///Combine two tree visitors.
    //TODO return impl trait instead of concrete type when that feature becomes stable.
    fn zip<F:CTreeIterator>(self,f:F)->ZippedDownTMut<Self,F>{
        ZippedDownTMut::new(self,f)
    }

    ///Calls the closure in dfs preorder (left,right,root).
    fn dfs_preorder<F:FnMut(Self::Item)>(self,mut func:F){
        fn rec<C:CTreeIterator,F:FnMut(C::Item)>(a:C,func:&mut F){
            //let d=*a.get_level();
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

    ///Calls the closure in dfs postorder (right,left,root).
    fn dfs_postorder<F:FnMut(Self::Item)>(self,mut func:F){
        fn rec<C:CTreeIterator,F:FnMut(C::Item)>(a:C,func:&mut F){
            //let d=*a.get_level();
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
}

use std::marker::PhantomData;


unsafe impl<'a,T:'a> std::marker::Send for DownTMut<'a,T>{}

///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    first_leaf:NodeIndex,
    phantom:PhantomData<&'a T>
}


impl<'a,T:'a> CTreeIterator for DownTMut<'a,T>{
    type Item=&'a mut T;
    
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


unsafe impl<'a,T:'a> std::marker::Send for DownT<'a,T>{}


///Tree visitor that returns a reference to each element in the tree.
pub struct DownT<'a,T:'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    first_leaf:NodeIndex,
}

impl<'a,T:'a> CTreeIterator for DownT<'a,T>{
    type Item=&'a T;

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
pub struct ZippedDownTMut<T1:CTreeIterator,T2:CTreeIterator>{
    a:T1,
    b:T2,
}

impl<T1:CTreeIterator,T2:CTreeIterator>  ZippedDownTMut<T1,T2>{
    #[inline(always)]
    fn new(a:T1,b:T2)->ZippedDownTMut<T1,T2>{
        ZippedDownTMut{a:a,b:b}
    }
}

impl<T1:CTreeIterator,T2:CTreeIterator> CTreeIterator for ZippedDownTMut<T1,T2>{
    type Item=(T1::Item,T2::Item);
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let (a_item,a_rest)=self.a.next();
        let (b_item,b_rest)=self.b.next();

        let item=(a_item,b_item);
        match (a_rest,b_rest){
            (Some(a_rest),Some(b_rest))=>{
                //let b_rest=b_rest.unwrap();
                let f1=ZippedDownTMut{a:a_rest.0,b:b_rest.0};
                let f2=ZippedDownTMut{a:a_rest.1,b:b_rest.1};
                (item,Some((f1,f2)))
            },
            _ =>{
                (item,None)
            }
        }
    }
}


pub use wrap::Wrap;
pub use wrap::Wrap2;
mod wrap{
    use super::*;

    ///Allows to traverse down from a visitor twice by creating a new visitor that borrows the other.
    pub struct Wrap<'a,T:'a>{
        a:LevelIter<DownTMut<'a,T>>
    }
    impl<'a,T:'a> Wrap<'a,T>{
        #[inline(always)]
        pub fn new(a:&'a mut LevelIter<DownTMut<T>>)->Wrap<'a,T>{
            let inner=&a.a;
            let k=DownTMut{remaining:inner.remaining,nodeid:inner.nodeid,first_leaf:inner.first_leaf,phantom:inner.phantom};
 
            let j=LevelIter{a:k,leveld:a.leveld};
            Wrap{a:j}
        }
    }
    
    impl<'a,T:'a> CTreeIterator for Wrap<'a,T>{
        type Item=(LevelDesc,&'a mut T);
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            let Wrap{a}=self;
  
            let (item,mm)=a.next();

            match mm{
                Some((left,right))=>{
                    let left=Wrap{a:left};
                    let right=Wrap{a:right};
                    return (item,Some((left,right)));
                },
                None=>{
                    return (item,None);
                }
            }
        }
    }


    ///Allows to traverse down from a visitor twice by creating a new visitor that borrows the other.
    pub struct Wrap2<'a,T:'a>{
        a:DownT<'a,T>
    }
    impl<'a,T:'a> Wrap2<'a,T>{
        #[inline(always)]
        pub fn new(a:&'a DownT<'a,T>)->Wrap2<'a,T>{
            //let inner=&a.a;
            //let k=DownTMut{remaining:inner.remaining,nodeid:inner.nodeid,first_leaf:inner.first_leaf,phantom:inner.phantom};
 
            //let j=LevelIter{a:k,leveld:a.leveld};
            let ff=unsafe{
                let mut ff=std::mem::uninitialized();
                std::ptr::copy(a, &mut ff, 1);
                ff
            };
            Wrap2{a:ff}
        }
    }
    
    impl<'a,T:'a> CTreeIterator for Wrap2<'a,T>{
        type Item=&'a T;
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            let Wrap2{a}=self;
  
            let (item,mm)=a.next();

            match mm{
                Some((left,right))=>{
                    let left=Wrap2{a:left};
                    let right=Wrap2{a:right};
                    return (item,Some((left,right)));
                },
                None=>{
                    return (item,None);
                }
            }
        }
    }
}

mod cons{
    use super::*;
    struct DownTConsume<'a,T:'a>{
        remaining:*mut GenTree<T>,
        nodeid:NodeIndex,
        first_leaf:NodeIndex,
        phantom:PhantomData<&'a T>
    }

    pub fn downt_into_dfs_preorder<T,F:FnMut(T)>(mut tree:GenTree<T>,func:F){
        {
            let t=DownTConsume{remaining:&mut tree,nodeid:NodeIndex(0),first_leaf:NodeIndex::first_leaf(tree.nodes.len()),phantom:PhantomData};
            t.dfs_preorder(func);
        }
        for a in tree.nodes.drain(..){
            std::mem::forget(a);
        }
    }

    impl<'a,T:'a> CTreeIterator for DownTConsume<'a,T>{
        type Item=T;

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
pub struct LevelDesc{
    depth:usize
}

impl LevelDesc{
    #[inline(always)]
    fn next_down(&self)->LevelDesc{
        LevelDesc{depth:self.depth+1}
    }

    #[inline(always)]
    pub fn get_depth(&self)->usize{
        self.depth
    } 
}

///A wrapper iterator that will additionally return the depth of each element.
pub struct LevelIter<T:CTreeIterator>{
    a:T,
    leveld:LevelDesc
}
impl <T:CTreeIterator> LevelIter<T>{
    #[inline(always)]
    pub fn new(a:T,leveld:LevelDesc)->LevelIter<T>{
        return LevelIter{a,leveld};
    }
}

impl<T:CTreeIterator> CTreeIterator for LevelIter<T>{
    type Item=(LevelDesc,T::Item);
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let LevelIter{a,leveld}=self;
        let (nn,rest)=a.next();

        let r=(leveld,nn);
        match rest{
            Some((left,right))=>{
                let ln=leveld.next_down();
                let ll=LevelIter{a:left,leveld:ln};
                let rr=LevelIter{a:right,leveld:ln};
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
