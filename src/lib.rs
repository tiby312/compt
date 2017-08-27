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


    ///Create a complete binary tree using the specified node generating function.
    pub fn from_bfs<F:Fn()->T>(func:&mut F,height:usize)->GenTree<T>{
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

    //Create a visitor struct
    pub fn create_down(&self)->DownT<T>{
        DownT{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height}}
    }

    //Create a mutable visitor struct
    pub fn create_down_mut(&mut self)->DownTMut<T>{
        DownTMut{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height},phantom:PhantomData}
    }


    //Visit every node in bfs order.
    //Unimplemented
    pub fn bfs<F:FnMut(&T,&LevelDesc)>(&self,_func:&mut F){
        //unimplemented!();
    }

    //Visit every node in dfs order.
    pub fn dfs<F:FnMut(&T,&LevelDesc)>(&self,func:&mut F){
        fn rec<T,F:FnMut(&T,&LevelDesc)>(a:DownT<T>,func:&mut F){
            let l=a.get_level();
            let n=a.get();
            match a.next(){
                Some((left,right))=>{
                    func(n,&l);
                    rec(left,func);
                    rec(right,func);
                },
                None=>{
                    func(n,&l);
                }
            }
        }
        let a=self.create_down();
        rec(a,func);
    }

    //This will move every node to the passed closure in dfs order before consuming itself.
    pub fn dfs_consume<F:FnMut(T)>(mut self,func:&mut F){
        
        //TODO verify this

        fn rec<T,F:FnMut(T)>(mut a:DownTMut<T>,func:&mut F){
            
            match a.next(){
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

    ///Create the children visitors and also return the node this visitor is pointing to.
    pub fn next<'c>(&'c mut self)->(&'c mut T,Option<(DownTMut<'c,T>,DownTMut<'c,T>)>){

        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        
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

///A level descriptor.
///The root has depth 0.
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
