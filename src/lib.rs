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

use std::marker::PhantomData;


///The complete binary tree. Internally stores the nodes in a Vec<T>.
///Height is atleast 1.
#[derive(Debug,Clone)]
pub struct GenTree<T> {
    nodes: Vec<T>,
    height: usize,
    first_leaf_index:NodeIndex
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


    pub fn from_vec(vec:Vec<T>,height:usize)->GenTree<T>{
        let num_nodes=self::compute_num_nodes(height);
        assert!(num_nodes==vec.len());

        GenTree{
            nodes:vec,
            height:height,
            first_leaf_index:NodeIndex(num_nodes/2)
        }
    }


    pub fn from_dfs<F:FnMut()->T>(func:&mut F,height:usize)->GenTree<T>{
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        tree.dfs_mut(&mut |node:&mut T|{
            *node=func();
        });
        tree
    }

    pub fn from_dfs_backwards<F:FnMut()->T>(func:&mut F,height:usize)->GenTree<T>{
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        tree.dfs_backwards_mut(&mut |node:&mut T|{
            *node=func();
        });
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
            first_leaf_index:NodeIndex(num_nodes/2)
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

    //Visit every node in bfs order.
    pub fn bfs<F:FnMut(&T)>(&self,func:&mut F){
        for i in self.nodes.iter(){
            func(i);
        }
    }
    
    pub fn dfs_mut<'a,F:FnMut(&'a mut T)>(&'a mut self,func:&mut F){
        fn rec<'a,T:'a,F:FnMut(&'a mut T)>(a:DownTMut2<'a,T>,func:&mut F){
            
            match a.next(){
                (xx,None)=>{
                    func(xx)
                },
                (xx,Some((left,right)))=>{
                    //let (left,right)=sec.into_get_mut_and_next();
                    func(xx);
                    rec(left,func);                    
                    rec(right,func); 
                }
            }
            
        }
        let a2=self.create_down_mut();
        rec(a2,func);
    
    }




      pub fn dfs_backwards_mut<'a,F:FnMut(&'a mut T)>(&'a mut self,func:&mut F){
        //TODO comgine with dfs_mut
        fn rec<'a,T:'a,F:FnMut(&'a mut T)>( a:DownTMut2<'a,T>,func:&mut F){
            match a.next(){
                (xx,None)=>{
                    func(xx)
                },
                (xx,Some((left,right)))=>{
                    //let (left,right)=sec.into_get_mut_and_next();
                    rec(right,func);
                    rec(left,func);
                    func(xx);
                }
            }
            
        }
        let a2=self.create_down_mut();
        rec(a2,func);
    
    }



    //Visit every node in in order traversal.
    pub fn dfs<'a,F:FnMut(&'a T)>(&'a self,func:&mut F){
        let mut func2=|a:&'a T,_:<Nothin as DX>::Item|{
            func(a);
        };
        self.dfs_comp(&mut func2,Nothin{});
    }

    //Visit every node in pre order traversal.
    pub fn dfs_comp<'a,I,X:DX<Item=I>,F:FnMut(&'a T,I)>(&'a self,func:&mut F,dx:X){

        fn rec<'a,T:'a,I,X:DX<Item=I>,F:FnMut(&'a T,I)>(a:DownT2<'a,T>,func:&mut F,dx:X){
            
            
            match a.next(){
                (nn,Some((left,right)))=>{
                    
                    let aaa=dx.next();


                    func(nn,dx.get());
                    rec(left,func,aaa.clone());                    
                    rec(right,func,aaa);
                },
                (nn,None)=>{
                    func(nn,dx.get());
                }
            }
        }
        let a2=self.create_down();
        rec(a2,func,dx);
    }


    //This will move every node to the passed closure in dfs order before consuming itself.
    pub fn dfs_consume<F:FnMut(T)>(mut self,func:&mut F){
        
        //TODO verify this

        fn rec<T,F:FnMut(T)>(a:DownTMut2<T>,func:&mut F){
            

            match a.next(){
                (nn,None)=>{
                    {
                        let node=unsafe{
                            let mut node=std::mem::uninitialized::<T>();
                            std::ptr::copy(nn,&mut node,1);
                            node
                        };

                        func(node);

                    }
                },
                (nn,Some((left,right)))=>{
                    //let (left,right)=sec.into_get_mut_and_next();
                    {
                        let node=unsafe{
                            let mut node=std::mem::uninitialized::<T>();
                            std::ptr::copy(nn,&mut node,1);
                            node
                        };

                        func(node);

                    }
                    
                    rec(left,func);
                    rec(right,func);
                }
            }
            
        }
        {
            let a=self.create_down_mut();
            rec(a,func);
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
struct NodeIndex(pub usize); //todo dont make private

impl NodeIndex{
    #[inline(always)]
    fn get_children(self) -> (NodeIndex, NodeIndex) {
        let NodeIndex(a) = self;
        (NodeIndex(2 * a + 1), NodeIndex(2 * a + 2))
    }
}




trait WrapTrait:CTreeIterator{
    fn clone(&self)->Self;
}

pub use wrap::Wrap;
mod wrap{
    use super::*;
    use std::mem::ManuallyDrop;
    pub struct Wrap<'a,T:CTreeIterator+'a>{
        a:ManuallyDrop<T>,
        phantom:PhantomData<&'a mut T>
    }
    impl<'a,T:CTreeIterator+'a> Wrap<'a,T>{
        pub fn new(a:&'a mut T)->Wrap<'a,T>{
            let mut m=unsafe{std::mem::uninitialized()};
            unsafe{std::ptr::copy(a,&mut m,1)};
            Wrap{a:ManuallyDrop::new(m),phantom:PhantomData}
        }
    }
    
    impl<'a,T:CTreeIterator+'a> CTreeIterator for Wrap<'a,T>{
        type Item=T::Item;
        fn next(self)->(Self::Item,Option<(Self,Self)>){
            let Wrap{a,phantom}=self;
            let a=ManuallyDrop::into_inner(a);

            let (item,mm)=a.next();

            match mm{
                Some((left,right))=>{
                    let left=Wrap{a:ManuallyDrop::new(left),phantom:PhantomData};
                    let right=Wrap{a:ManuallyDrop::new(right),phantom:PhantomData};
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

        fn get(&self)->Self::Item{
            self.a.get()
        }
    }
}


pub trait CTreeIterator:Sized{
    type Item;
    fn next(self)->(Self::Item,Option<(Self,Self)>);
    fn get_level(&self)->&LevelDesc;
    fn get(&self)->Self::Item;
}

unsafe impl<'a,T:'a> std::marker::Send for DownTMut2<'a,T>{}


///A mutable visitor struct.
///Unlike DownT, the children's lifetime may be smaller that the lifetime of the parent.
///This way next() can be called multiple times, but still only one DownTMut will ever point to a particular node.
pub struct DownTMut2<'a,T:'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc,
    phantom:PhantomData<&'a T>
}

/*
impl<'a,T:'a> WrapTrait for DownTMut2<'a,T>{
    fn clone(&self)->Self{
        DownTMut2{remaining:self.remaining,nodeid:self.nodeid,leveld:self.leveld,phantom:PhantomData}
    }
}*/

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
    #[inline(always)]
    fn get(&self)->Self::Item{
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        a
    }

}

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
 
        let a=unsafe{&(*self.remaining).nodes[self.nodeid.0]};
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
    #[inline(always)]
    fn get(&self)->Self::Item{
        let a=unsafe{& (*self.remaining).nodes[self.nodeid.0]};
        a
    }

}




pub struct ZippedDownTMut<T1:CTreeIterator,T2:CTreeIterator>{
    a:T1,
    b:T2,
}
impl<T1:CTreeIterator,T2:CTreeIterator>  ZippedDownTMut<T1,T2>{
    pub fn new(a:T1,b:T2)->ZippedDownTMut<T1,T2>{
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

    fn get(&self)->Self::Item{
        let a=self.a.get();
        let b=self.b.get();
        (a,b)
    }
}



pub trait DX:std::clone::Clone{
    type Item;
    fn next(&self)->Self;
    fn get(&self)->Self::Item;
}

#[derive(Debug,Copy,Clone)]
pub struct LevelDescIter{
    l:LevelDesc
}
impl LevelDescIter{
    #[inline(always)]
    pub fn new(l:LevelDesc)->LevelDescIter{
        LevelDescIter{l:l}
    }
}
impl DX for LevelDescIter{
    type Item=LevelDesc;
    #[inline(always)]
    fn next(&self)->LevelDescIter{
        LevelDescIter{l:self.l.next_down()}
    }
    #[inline(always)]
    fn get(&self)->LevelDesc{
        self.l
    }
}

#[derive(Copy,Clone)]
struct Nothin{
}
impl DX for Nothin{
    type Item=();
    #[inline(always)]
    fn next(&self)->Nothin{
        Nothin{}
    }
    #[inline(always)]
    fn get(&self)->(){}
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
