use super::*;

///Error indicating the vec that was passed is not a size that you would expect for the given height.
pub struct NotCompleteTreeSizeErr;


///Complete binary tree stored in BFS order.
///Height is atleast 1.
pub struct GenTree<T> {
    nodes: Vec<T>,
    height: usize,
}

impl<T> GenTree<T> {
    
    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn from_vec(vec:Vec<T>,height:usize)->Result<GenTree<T>,NotCompleteTreeSizeErr>{
        if 2_usize.pow(height as u32)==vec.len()+1{
            Ok(GenTree{nodes:vec,height})
        }else{
            Err(NotCompleteTreeSizeErr)
        }
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

    
    #[inline(always)]
    ///Create a immutable visitor struct
    pub fn create_down(&self)->DownT<T>{
        let k=DownT{remaining:self,nodeid:NodeIndex(0),depth:0,height:self.height};
        k
    }
    
    #[inline(always)]
    ///Create a mutable visitor struct
    pub fn create_down_mut(&mut self)->DownTMut<T>{
        let base=&mut self.nodes[0] as *mut T;
        let k=DownTMut{curr:&mut self.nodes[0],base,depth:0,height:self.height};
        k
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
}


///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:'a>{
    curr:&'a mut T,
    base:*mut T,
    depth:usize,
    height:usize
}


unsafe impl<'a,T:'a> FixedDepthCTreeIterator for DownTMut<'a,T>{}


impl<'a,T:'a> CTreeIterator for DownTMut<'a,T>{
    type Item=&'a mut T;
    type Extra=();

    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
 
        //Unsafely get a mutable reference to this nodeid.
        //Since at the start there was only one DownTMut that pointed to the root,
        //there is no danger of two DownTMut's producing a reference to the same node.
        if self.depth==self.height-1{
            (self.curr,None)
        }else{
            let (left,right)=unsafe{
                let diff=(self.curr as *mut T).offset_from(self.base);
                let left=&mut *(self.base as *mut T).offset(2*diff+1);
                let right=&mut *(self.base as *mut T).offset(2*diff+2);
                (left,right)
            };

            let j=(   
                (),  
                DownTMut{curr:left,base:self.base,depth:self.depth+1,height:self.height},
                DownTMut{curr:right,base:self.base,depth:self.depth+1,height:self.height}
            );
            (self.curr,Some(j))
        }
    }
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let diff=self.height-self.depth;
        (diff,Some(diff))
    }
}



///Tree visitor that returns a reference to each element in the tree.
pub struct DownT<'a,T:'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    depth:usize,
    height:usize
}


unsafe impl<'a,T:'a> FixedDepthCTreeIterator for DownT<'a,T>{}


impl<'a,T:'a> CTreeIterator for DownT<'a,T>{
    type Item=&'a T;
    type Extra=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
 
        let a=&self.remaining.nodes[self.nodeid.0];
        
        if self.depth==self.height-1{
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                (),
                DownT{remaining:self.remaining,nodeid:l,height:self.height,depth:self.depth+1},
                DownT{remaining:self.remaining,nodeid:r,height:self.height,depth:self.depth+1}
            );
            (a,Some(j))
        }
    }
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let diff=self.height-self.depth;
        (diff,Some(diff))
    }
}
