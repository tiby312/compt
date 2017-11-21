//! A Complete Binary Tree library.
//! It is internally represented as a 1D array.
//! Provides a way to get mutable references to children nodes simultaneously. Useful for paralalizing divide and conquer style problems.
//!
//!## Unsafety
//!There is some unsafe code.
//!
//!## Example
//!```
//!extern crate compt;
//!fn main()
//!{
//!        let mut tree=compt::GenTree::from_bfs(&mut ||0.0,5);
//!        {
//!            let mut down=tree.create_down_mut();
//!            let mut nn=down.next().1.unwrap();
//!            {
//!                
//!                let (mut left,mut right)=nn.get_mut_and_next();
//!                *left.get_mut()=5.0;
//!                *right.get_mut()=4.0;
//!            }
//!            {
//!                let (mut left,_)=nn.into_get_mut_and_next();
//!                *left.get_mut()=3.0;    
//!            }
//!        }
//!        {
//!            let down=tree.create_down();
//!            let (left,right)=down.next().unwrap();
//!            assert!(*left.get()==3.0);
//!            assert!(*right.get()==4.0);
//!        }
//!
//!}
//!```
//!


///The complete binary tree. Internally stores the nodes in a Vec<T>.
///Height is atleast 1.
pub struct GenTree<T> {
    nodes: Vec<T>,
    height: usize,
    //first_leaf_index:NodeIndex
}



///Compute the number of nodes in a complete binary tree based on a height.
#[inline(always)]
pub fn compute_num_nodes(height:usize)->usize{
    return (1 << height) - 1;
}

impl<T> GenTree<T> {
    
    #[inline(always)]
    pub fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }

    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn from_dfs<F:FnMut()->T>(func:&mut F,height:usize)->GenTree<T>{
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        {
            let t=tree.create_down_mut();
            t.dfs_preorder(&mut |_:&LevelDesc,node:&mut T|{
                *node=func();
            });
        }
        tree
    }

    pub fn from_dfs_backwards<F:FnMut()->T>(func:&mut F,height:usize)->GenTree<T>{
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        {
            let t=tree.create_down_mut();
            t.dfs_postorder(&mut |_:&LevelDesc,node:&mut T|{
                *node=func();
            });
        }
        tree
    }

    ///Create a complete binary tree using the specified node generating function.
    pub fn from_bfs<F:FnMut()->T>(func:&mut F,height:usize)->GenTree<T>{
        assert!(height>=1);
        let num_nodes=self::compute_num_nodes(height);

        let mut vec=Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes{
            vec.push(func())
        }
        GenTree{
            nodes:vec,
            height:height,
            //first_leaf_index:NodeIndex(num_nodes/2)
        }
    }

    //Visit every node in bfs order.
    pub fn bfs<F:FnMut(&T)>(&self,func:&mut F){
        for i in self.nodes.iter(){
            func(i);
        }
    }
    //Visit every node in bfs order.
    pub fn bfs_mut<F:FnMut(&mut T)>(&mut self,func:&mut F){
        for i in self.nodes.iter_mut(){
            func(i);
        }
    }
    


    ///Guarenteed to be a root.
    #[inline(always)]
    pub fn get_root_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.nodes.get_unchecked_mut(0) }
    }

    ///Guarenteed to be a root.
    #[inline(always)]
    pub fn get_root<'a>(&'a self) -> &'a T {
        unsafe { self.nodes.get_unchecked(0) }
    }

    #[inline(always)]
    pub fn get_level_desc(&self)->LevelDesc{
        LevelDesc{depth:0,height:self.height}
    }
    //Create a visitor struct
    #[inline(always)]
    pub fn create_down(&self)->DownT2<T>{
        DownT2{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height}}
    }
    //Create a mutable visitor struct
    #[inline(always)]
    pub fn create_down_mut(&mut self)->DownTMut2<T>{
        DownTMut2{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height},phantom:PhantomData}
    }

    pub fn into_dfs_preorder<F:FnMut(&LevelDesc,T)>(mut self,func:&mut F){

        {
            let t=DownTConsume{remaining:&mut self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height},phantom:PhantomData};
            t.dfs_preorder(func);
        }
        for a in self.nodes.drain(..){
            std::mem::forget(a);
        }
    }
}



///Visitor functions pass this as an argument to be used by the user.
///It is never used to index into the tree, but a user might have some use for it.
///The nodes in the tree are kept in the tree in bfs order.
#[derive(Copy,Clone,Debug)]
struct NodeIndex(usize); //todo dont make private

impl NodeIndex{
    #[inline(always)]
    fn get_children(self) -> (NodeIndex, NodeIndex) {
        let NodeIndex(a) = self;
        (NodeIndex(2 * a + 1), NodeIndex(2 * a + 2))
    }
}





///This is what all tree visitors implement.
pub trait CTreeIterator:Sized{
    type Item;
    fn next(self)->(Self::Item,Option<(Self,Self)>);
    fn get_level(&self)->&LevelDesc;

    ///Combine two tree visitors that have the same depth left.
    fn zip<F:CTreeIterator>(self,f:F)->ZippedDownTMut<Self,F>{
        ZippedDownTMut::new(self,f)
    }

    ///left,right,root
    fn dfs_preorder<F:FnMut(&LevelDesc,Self::Item)>(self,func:&mut F){
        fn rec<C:CTreeIterator,F:FnMut(&LevelDesc,C::Item)>(a:C,func:&mut F){
            let d=*a.get_level();
            let (nn,rest)=a.next();
            func(&d,nn);
            match rest{
                Some((left,right))=>{
                    rec(left,func);
                    rec(right,func);
                },
                None=>{

                }
            }
        }
        rec(self,func);
    }

    ///right,left,root
    fn dfs_postorder<F:FnMut(&LevelDesc,Self::Item)>(self,func:&mut F){
        fn rec<C:CTreeIterator,F:FnMut(&LevelDesc,C::Item)>(a:C,func:&mut F){
            let d=*a.get_level();
            let (nn,rest)=a.next();
            match rest{
                Some((left,right))=>{
                    rec(right,func);
                    rec(left,func);
                },
                None=>{

                }
            }
            func(&d,nn);
        }
        rec(self,func);
    }
}

use std::marker::PhantomData;


unsafe impl<'a,T:'a> std::marker::Send for DownTMut2<'a,T>{}

///Tree visitor that returns a mutable reference to each element
pub struct DownTMut2<'a,T:'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc,
    phantom:PhantomData<&'a T>
}


impl<'a,T:'a> CTreeIterator for DownTMut2<'a,T>{
    type Item=&'a mut T;
    ///Returns either the contents of this node, or a struct that allows
    ///retrieval of children nodes.
     fn next(self)->(Self::Item,Option<(Self,Self)>){
 
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        if self.leveld.is_leaf(){
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                DownTMut2{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down(),phantom:PhantomData},
                DownTMut2{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down(),phantom:PhantomData}
            );
            (a,Some(j))
        }
    }
    #[inline(always)]
    fn get_level(&self)->&LevelDesc{
        &self.leveld
    }

}


///Tree visitor that returns a reference to each element
pub struct DownT2<'a,T:'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc,
}

impl<'a,T:'a> CTreeIterator for DownT2<'a,T>{
    type Item=&'a T;
    ///Returns either the contents of this node, or a struct that allows
    ///retrieval of children nodes.
     fn next(self)->(Self::Item,Option<(Self,Self)>){
 
        let a=unsafe{&(*self.remaining).nodes.get_unchecked(self.nodeid.0)};
        if self.leveld.is_leaf(){
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                DownT2{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down()},
                DownT2{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down()}
            );
            (a,Some(j))
        }
    }
    #[inline(always)]
    fn get_level(&self)->&LevelDesc{
        &self.leveld
    }

}


///Tree visitor that zips up two seperate visitors.
pub struct ZippedDownTMut<T1:CTreeIterator,T2:CTreeIterator>{
    a:T1,
    b:T2,
}
impl<T1:CTreeIterator,T2:CTreeIterator>  ZippedDownTMut<T1,T2>{
    fn new(a:T1,b:T2)->ZippedDownTMut<T1,T2>{
        assert!(a.get_level().get_depth_left()==b.get_level().get_depth_left());
        ZippedDownTMut{a:a,b:b}
    }
}
impl<T1:CTreeIterator,T2:CTreeIterator> CTreeIterator for ZippedDownTMut<T1,T2>{
    type Item=(T1::Item,T2::Item);
    fn next(self)->(Self::Item,Option<(Self,Self)>){
        let (a_item,a_rest)=self.a.next();
        let (b_item,b_rest)=self.b.next();

        let item=(a_item,b_item);
        match a_rest{
            Some(a_rest)=>{
                let b_rest=b_rest.unwrap();
                let f1=ZippedDownTMut{a:a_rest.0,b:b_rest.0};
                let f2=ZippedDownTMut{a:a_rest.1,b:b_rest.1};
                (item,Some((f1,f2)))
            },
            None=>{
                (item,None)
            }
        }
    }
    #[inline(always)]
    fn get_level(&self)->&LevelDesc{
        self.a.get_level()
    }

}


pub use wrap::Wrap;
mod wrap{
    use super::*;

    ///Allows to traverse down from a visitor twice by creating a new visitor that borrows the other.
    pub struct Wrap<'a,T:'a>{
        a:DownTMut2<'a,T>
    }
    impl<'a,T:'a> Wrap<'a,T>{
        pub fn new(a:&'a mut DownTMut2<T>)->Wrap<'a,T>{
            
            let k=DownTMut2{remaining:a.remaining,nodeid:a.nodeid,leveld:a.leveld,phantom:a.phantom};
 
            Wrap{a:k}
        }
    }
    
    impl<'a,T:'a> CTreeIterator for Wrap<'a,T>{
        type Item=&'a mut T;
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            let Wrap{a}=self;
            //let a=ManuallyDrop::into_inner(a);

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
        #[inline(always)]
        fn get_level(&self)->&LevelDesc{
            self.a.get_level()
        }
    }
}


struct DownTConsume<'a,T:'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc,
    phantom:PhantomData<&'a T>
}


impl<'a,T:'a> CTreeIterator for DownTConsume<'a,T>{
    type Item=T;
    ///Returns either the contents of this node, or a struct that allows
    ///retrieval of children nodes.
     fn next(self)->(Self::Item,Option<(Self,Self)>){
 
        let mut val=unsafe{std::mem::uninitialized()};
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        unsafe{std::ptr::copy(&mut val,a,1)};

        if self.leveld.is_leaf(){
            (val,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                DownTConsume{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down(),phantom:PhantomData},
                DownTConsume{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down(),phantom:PhantomData}
            );
            (val,Some(j))
        }
    }
    #[inline(always)]
    fn get_level(&self)->&LevelDesc{
        &self.leveld
    }
}



///A level descriptor.
///The root has depth 0.
#[derive(Debug,Copy,Clone)]
pub struct LevelDesc{
    height:usize,
    depth:usize
}

impl LevelDesc{
    #[inline(always)]
    fn next_down(&self)->LevelDesc{
        LevelDesc{height:self.height,depth:self.depth+1}
    }

    #[inline(always)]
    pub fn get_height(&self)->usize{
        self.height
    }

    ///Returns the height-depth
    #[inline(always)]
    pub fn get_depth_left(&self)->usize{
        self.height-self.depth
    }

    #[inline(always)]
    pub fn get_depth(&self)->usize{
        self.depth
    }  

    #[inline(always)]
    pub fn is_leaf(&self)->bool{
        self.depth==self.height-1
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut tree=GenTree::from_bfs(&mut ||0.0,5);
        {
            let down=tree.create_down_mut();
            let mut nn=down.next().1.unwrap();
            {
                
                let (mut left,mut right)=nn.get_mut_and_next();
                *left.get_mut()=5.0;
                *right.get_mut()=4.0;
            }
            {
                let (mut left,_)=nn.into_get_mut_and_next();
                *left.get_mut()=3.0;    
            }
        }
        {
            let down=tree.create_down();
            let (left,right)=down.next().unwrap();
            assert!(*left.get()==3.0);
            assert!(*right.get()==4.0);
        }
    }
}
