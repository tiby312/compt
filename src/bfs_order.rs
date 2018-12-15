use super::*;
use std::marker::PhantomData;

///Error indicating the vec that was passed is not a size that you would expect for the given height.
#[derive(Copy,Clone,Debug)]
pub struct NotCompleteTreeSizeErr;


///Contains of a Complete tree. Internally uses a Vec.
pub struct CompleteTreeContainer<T>{
    nodes:Vec<T>
}
impl<T> CompleteTreeContainer<T>{

    #[inline]
    pub fn from_vec(vec:Vec<T>)->Result<CompleteTreeContainer<T>,NotCompleteTreeSizeErr>{
        if (vec.len()+1).is_power_of_two() && !vec.is_empty(){
            Ok(CompleteTreeContainer{nodes:vec})
        }else{
            Err(NotCompleteTreeSizeErr)
        }
    }


    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self)->Vec<T>{
        let CompleteTreeContainer{nodes}=self;
        nodes
    }
}


impl<T> std::ops::Deref for CompleteTreeContainer<T>{
    type Target=CompleteTree<T>;
    fn deref(&self)->&CompleteTree<T>{
        unsafe{&*(self.nodes.as_slice() as *const [T] as *const bfs_order::CompleteTree<T>)}
        //unsafe{std::mem::transmute(self.nodes.as_slice())}
    }
}
impl<T> std::ops::DerefMut for CompleteTreeContainer<T>{
    fn deref_mut(&mut self)->&mut CompleteTree<T>{
        unsafe{&mut *(self.nodes.as_mut_slice() as *mut [T] as *mut bfs_order::CompleteTree<T>)}
        //unsafe{std::mem::transmute(self.nodes.as_mut_slice())}
    }
}


///Complete binary tree stored in BFS order.
///Height is atleast 1.
pub struct CompleteTree<T> {
    nodes: [T],
}

impl<T> CompleteTree<T> {
    
    #[inline]
    pub fn get_height(&self) -> usize {
        compute_height(self.nodes.len())
    }


    
    #[inline]
    ///Create a immutable visitor struct
    pub fn vistr(&self)->Vistr<T>{
        Vistr{remaining:self,nodeid:NodeIndex(0),depth:0,height:self.get_height()-1}
    }
    
    #[inline]
    ///Create a mutable visitor struct
    pub fn vistr_mut(&mut self)->VistrMut<T>{
        let base=std::ptr::Unique::new(self.nodes.as_mut_ptr()).unwrap();
        VistrMut{current:0,base,depth:0,_p:PhantomData,height:self.get_height()-1}
    }

  
    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn get_nodes(&self)->&[T]{
        &self.nodes
    }


    #[inline]
    pub fn get_nodes_mut(&mut self)->&mut [T]{
        &mut self.nodes
    }
    
}



///Visitor functions use this type to determine what node to visit.
///The nodes in the tree are kept in the tree in BFS order.
#[derive(Copy,Clone,Debug)]
struct NodeIndex(usize);

impl NodeIndex{
    #[inline]
    fn get_children(self) -> (NodeIndex, NodeIndex) {
        let NodeIndex(a) = self;
        (NodeIndex(2 * a + 1), NodeIndex(2 * a + 2))
    }
}


///Tree visitor that returns a mutable reference to each element in the tree.
pub struct VistrMut<'a,T:'a>{
    current:usize,
    base:std::ptr::Unique<T>,
    depth:usize,
    height:usize,
    _p:PhantomData<&'a mut T>
}


unsafe impl<'a,T:'a> FixedDepthVisitor for VistrMut<'a,T>{}


impl<'a,T:'a> Visitor for VistrMut<'a,T>{
    type Item=&'a mut T;
    type NonLeafItem=();

    #[inline]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
        let curr=unsafe{&mut *self.base.as_ptr().add(self.current)};
        //Unsafely get a mutable reference to this nodeid.
        //Since at the start there was only one VistrMut that pointed to the root,
        //there is no danger of two VistrMut's producing a reference to the same node.
        if self.depth==self.height-1{
            (curr,None)
        }else{
            let (left,right)={
                let left=2*self.current+1;
                let right=2*self.current+2;
                (left,right)
            };

            let j=(   
                (),  
                VistrMut{current:left,base:self.base,depth:self.depth+1,height:self.height,_p:PhantomData},
                VistrMut{current:right,base:self.base,depth:self.depth+1,height:self.height,_p:PhantomData}
            );
            (curr,Some(j))
        }
    }
    #[inline]
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let diff=self.height-self.depth;
        (diff,Some(diff))
    }
}



///Tree visitor that returns a reference to each element in the tree.
pub struct Vistr<'a,T:'a>{
    remaining:&'a CompleteTree<T>,
    nodeid:NodeIndex,
    depth:usize,
    height:usize
}


unsafe impl<'a,T:'a> FixedDepthVisitor for Vistr<'a,T>{}


impl<'a,T:'a> Visitor for Vistr<'a,T>{
    type Item=&'a T;
    type NonLeafItem=();
    #[inline]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
 
        let a=&self.remaining.nodes[self.nodeid.0];
        
        if self.depth==self.height-1{
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                (),
                Vistr{remaining:self.remaining,nodeid:l,height:self.height,depth:self.depth+1},
                Vistr{remaining:self.remaining,nodeid:r,height:self.height,depth:self.depth+1}
            );
            (a,Some(j))
        }
    }
    #[inline]
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let diff=self.height-self.depth;
        (diff,Some(diff))
    }
}
