//! A Complete Binary Tree library.
//! It is internally represented as a 1D array.
//! Provides a way to get mutable references to children nodes simultaneously. Useful for paralalizing divide and conquer style problems.
//!
//!## Unsafety
//!There is some unsafe code.
//!
//!## Example
//!```
//!let mut tree=GenTree::from_bfs(&mut ||0.0,5);
//!{
//!     let mut down=tree.create_down_mut();
//!
//!     {
//!         let (_,b)=down.next();
//!         let (mut left,mut right)=b.unwrap();
//!
//!         let (val1,_)=left.next();
//!         let (val2,_)=right.next();
//!         *val1=5.0;
//!         *val2=4.0;
//!     }
//!     {
//!         let (_,b)=down.next();
//!         let (mut left,_)=b.unwrap();
//!         *left.get_mut()=3.0;    
//!     }
//!}
//!{
//!     let down=tree.create_down();
//!     let (left,right)=down.next().unwrap();
//!     assert!(*left.get()==3.0);
//!     assert!(*right.get()==4.0);
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
}



///Compute the number of nodes in a complete binary tree based on a height.
pub fn compute_num_nodes(height:usize)->usize{
    return (1 << height) - 1;
}

impl<T> GenTree<T> {
    
    pub fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_height(&self) -> usize {
        self.height
    }


    pub fn from_vec(vec:Vec<T>,height:usize)->GenTree<T>{
        let num_nodes=self::compute_num_nodes(height);
        assert!(num_nodes==vec.len());

        GenTree{
            nodes:vec,
            height:height
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
            height:height
        }
    }

    ///Guarenteed to be a root.
    pub fn get_root_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.nodes.get_unchecked_mut(0) }
    }

    ///Guarenteed to be a root.
    pub fn get_root<'a>(&'a self) -> &'a T {
        unsafe { self.nodes.get_unchecked(0) }
    }

    pub fn get_level_desc(&self)->LevelDesc{
        LevelDesc{depth:0,height:self.height}
    }
    //Create a visitor struct
    pub fn create_down(&self)->DownT<T>{
        DownT{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height}}
    }

    //Create a mutable visitor struct
    pub fn create_down_mut(&mut self)->DownTMut<T>{
        DownTMut{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height},phantom:PhantomData}
    }


    //Visit every node in bfs order.
    pub fn bfs<F:FnMut(&T)>(&self,func:&mut F){
        for i in self.nodes.iter(){
            func(i);
        }
    }
    
    pub fn dfs_mut<'a,F:FnMut(&'a mut T)>(&'a mut self,func:&mut F){
        fn rec<'a,T:'a,F:FnMut(&'a mut T)>(a:DownTMut<'a,T>,func:&mut F){
            
            match a.into_get_mut_and_next(){
                (xx,Some((left,right)))=>{
                    rec(left,func);

                    func(xx);
                    
                    rec(right,func);
                },
                (xx,None)=>{
                    func(xx);
                }
            }
        }
        let a2=self.create_down_mut();
        rec(a2,func);
    
    }




      pub fn dfs_backwards_mut<'a,F:FnMut(&'a mut T)>(&'a mut self,func:&mut F){
        //TODO comgine with dfs_mut
        fn rec<'a,T:'a,F:FnMut(&'a mut T)>( a:DownTMut<'a,T>,func:&mut F){
            
            match a.into_get_mut_and_next(){
                (xx,Some((left,right)))=>{
                    rec(right,func);
                    
                    func(xx);
                    
                    rec(left,func);

                },
                (xx,None)=>{
                    func(xx);
                }
            }
        }
        let a2=self.create_down_mut();
        rec(a2,func);
    
    }


    //TODO combine with dfs_comp
    //Visit every node in in order traversal.
    /*
    pub fn dfs<'a,F:FnMut(&'a T,&LevelDesc)>(&'a self,func:&mut F){

        fn rec<'a,T:'a,F:FnMut(&'a T,&LevelDesc)>(a:DownT<'a,T>,func:&mut F){
            let l=a.get_level();
            //let n=a.get();
            match a.next(){
                Some((left,right))=>{
                    rec(left,func);

                    func(a.into_inner(),l);
                    
                    rec(right,func);
                },
                None=>{
                    func(a.into_inner(),l);
                }
            }
        }
        let a2=self.create_down();
        rec(a2,func);
    }*/


    //Visit every node in in order traversal.
    pub fn dfs<'a,F:FnMut(&'a T)>(&'a self,func:&mut F){
        let mut func2=|a:&'a T,_:<Nothin as DX>::Item|{
            func(a);
        };
        self.dfs_comp(&mut func2,Nothin{});
    }

    //Visit every node in in order traversal.
    pub fn dfs_comp<'a,I,X:DX<Item=I>,F:FnMut(&'a T,I)>(&'a self,func:&mut F,dx:X){

        fn rec<'a,T:'a,I,X:DX<Item=I>,F:FnMut(&'a T,I)>(a:DownT<'a,T>,func:&mut F,dx:X){
            
            match a.next(){
                Some((left,right))=>{
                    
                    let aaa=dx.next();

                    rec(left,func,aaa.clone());

                    func(a.into_inner(),dx.get());
                    
                    rec(right,func,aaa);
                },
                None=>{
                    func(a.into_inner(),dx.get());
                }
            }
        }
        let a2=self.create_down();
        rec(a2,func,dx);
    }


    //This will move every node to the passed closure in dfs order before consuming itself.
    pub fn dfs_consume<F:FnMut(T)>(mut self,func:&mut F){
        
        //TODO verify this

        fn rec<T,F:FnMut(T)>(mut a:DownTMut<T>,func:&mut F){
            
            match a.get_mut_and_next(){
                (nn,Some((left,right)))=>{
                    
                    rec(left,func);
                    {
                        let node=unsafe{
                            let mut node=std::mem::uninitialized::<T>();
                            std::ptr::copy(nn,&mut node,1);
                            node
                        };

                        func(node);

                    }
                    rec(right,func);
                },
                (nn,None)=>{
                    {
                        let node=unsafe{
                            let mut node=std::mem::uninitialized::<T>();
                            std::ptr::copy(nn,&mut node,1);
                            node
                        };

                        func(node);

                    }
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
    fn get_children(self) -> (NodeIndex, NodeIndex) {
        let NodeIndex(a) = self;
        (NodeIndex(2 * a + 1), NodeIndex(2 * a + 2))
    }
}



///A visitor struct. 
///Since this is a read only visitor, the children DownT's can safely have the same lifetime as their parent.
///This means multiple instances of DownT may end up pointing to the same node (if next() is called multiple times).
pub struct DownT<'a,T:'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc
}

impl<'a,T> DownT<'a,T>{

    ///Get the node the visitor is pointing to.
    pub fn get(&self)->&T{
        &self.remaining.nodes[self.nodeid.0]
    }

    pub fn into_inner(self)->&'a T{
         &self.remaining.nodes[self.nodeid.0]        
    }
    ///Create children visitors
    pub fn next(&self)->Option<(DownT<'a,T>,DownT<'a,T>)>{

        if self.leveld.is_leaf(){
            return None
        }

        let (l,r)=self.nodeid.get_children();
        Some((
            DownT{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down()},
            DownT{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down()}
        ))
    }

    ///Get information about the level we are on.
    pub fn get_level(&self)->&LevelDesc{
        &self.leveld
    }
}

unsafe impl<'a,T:'a> std::marker::Sync for DownTMut<'a,T>{}
unsafe impl<'a,T:'a> std::marker::Send for DownTMut<'a,T>{}

///A mutable visitor struct.
///Unlike DownT, the children's lifetime may be smaller that the lifetime of the parent.
///This way next() can be called multiple times, but still only one DownTMut will ever point to a particular node.
pub struct DownTMut<'a,T:'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc,
    phantom:PhantomData<&'a T>
}


impl<'a,T:'a> DownTMut<'a,T>{

    ///Get the node the visitor is pointing to.
    pub fn get_mut(&mut self)->&mut T{
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        a
    }

    pub fn next<'c>(&'c mut self)->Option<(DownTMut<'c,T>,DownTMut<'c,T>)>{

        if self.leveld.is_leaf(){
            return None
        }

        let (l,r)=self.nodeid.get_children();
        
        Some((     
            DownTMut{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down(),phantom:PhantomData},
            DownTMut{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down(),phantom:PhantomData}
        ))
    }


    ///Create the children visitors and also return the node this visitor is pointing to.
    pub fn into_get_mut_and_next<'c>(self)->(&'c mut T,Option<(DownTMut<'c,T>,DownTMut<'c,T>)>){
        //TODO code duplication

        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        //TODO reuse next()
        if self.leveld.is_leaf(){
            return (a,None)
        }

        let (l,r)=self.nodeid.get_children();
        
        (a,Some((     
            DownTMut{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down(),phantom:PhantomData},
            DownTMut{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down(),phantom:PhantomData}
        )))
    }

    ///Create the children visitors and also return the node this visitor is pointing to.
    pub fn get_mut_and_next<'c>(&'c mut self)->(&'c mut T,Option<(DownTMut<'c,T>,DownTMut<'c,T>)>){

        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        //TODO reuse next()
        if self.leveld.is_leaf(){
            return (a,None)
        }

        let (l,r)=self.nodeid.get_children();
        
        (a,Some((     
            DownTMut{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down(),phantom:PhantomData},
            DownTMut{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down(),phantom:PhantomData}
        )))
    }

    ///Get information about the level we are on.
    pub fn get_level(&self)->&LevelDesc{
        &self.leveld
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
    pub fn new(l:LevelDesc)->LevelDescIter{
        LevelDescIter{l:l}
    }
}
impl DX for LevelDescIter{
    type Item=LevelDesc;
    fn next(&self)->LevelDescIter{
        LevelDescIter{l:self.l.next_down()}
    }
    fn get(&self)->LevelDesc{
        self.l
    }
}

#[derive(Copy,Clone)]
struct Nothin{
}
impl DX for Nothin{
    type Item=();
    fn next(&self)->Nothin{
        Nothin{}
    }
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
    fn next_down(&self)->LevelDesc{
        LevelDesc{height:self.height,depth:self.depth+1}
    }

    pub fn get_height(&self)->usize{
        self.height
    }

    ///Returns the height-depth
    pub fn get_depth_left(&self)->usize{
        self.height-self.depth
    }

    pub fn get_depth(&self)->usize{
        self.depth
    }  

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
            let mut down=tree.create_down_mut();

            {
                let (_,b)=down.next();
                let (mut left,mut right)=b.unwrap();
                
                let (val1,_)=left.next();
                let (val2,_)=right.next();
                *val1=5.0;
                *val2=4.0;
            
            }
            {
                let (_,b)=down.next();
                let (mut left,_)=b.unwrap();
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
