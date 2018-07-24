use super::*;


///The complete binary tree. Internally stores the elements in a Vec<T> so it is very compact.
///Height is atleast 1.
///Elements stored in BFS order.
///Has 2<sup>k-1</sup> elements where k is the height.
///## Unsafety
///
/// With a regular slice, getting one mutable reference to an element will borrow the
/// entire slice. The slice that GenTree uses, however, internally has the invariant that it is laid out
/// in BFS order. Therefore one can safely assume that if (starting at the root),
/// one had a mutable reference to a parent k, and one were to get the children using 2k+1 and 2k+2
/// to get *two* mutable references to the children,
/// they would be guarenteed to be distinct (from each other and also the parent) despite the fact that they belong to the same slice.
pub struct GenTree<T:Send> {
    nodes: Vec<T>,
    height: usize,
}

impl<T:Send> GenTree<T> {
    
    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.height
    }
    /*
    ///Create a complete binary tree using the specified node generating function.
    pub fn from_dfs<F:FnMut()->T>(mut func:F,height:usize)->GenTree<T>{
        assert!(height>=1);
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        {
            let t=tree.create_down_mut();
            t.dfs_preorder(|node:&mut T,_|{
                *node=func();
            });
        }
        tree
    }

    */

    #[inline(always)]
    pub fn from_vec(vec:Vec<T>,height:usize)->Result<GenTree<T>,&'static str>{
        if 2_usize.pow(height as u32)==vec.len()+1{
            Ok(GenTree{nodes:vec,height})
        }else{
            Err("Not a power of two")
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
        let k=DownT{remaining:self,nodeid:NodeIndex(0),depth:0,height:self.height};
        k
    }
    
    #[inline(always)]
    ///Create a mutable visitor struct
    pub fn create_down_mut(&mut self)->DownTMut<T>{
        let base=(&mut self.nodes[0] as *mut T);
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
    #[inline(always)]
    fn first_leaf(nodes:usize)->NodeIndex{
        NodeIndex(nodes/2)
    }
}


///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:Send+'a>{
    curr:&'a mut T,
    base:*mut T,
    depth:usize,
    height:usize
}

impl<'a,T:Send+'a> CTreeIterator for DownTMut<'a,T>{
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
                let left=unsafe{&mut *(self.base as *mut T).offset(2*diff+1)};
                let right=unsafe{&mut *(self.base as *mut T).offset(2*diff+2)};
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
pub struct DownT<'a,T:Send+'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    depth:usize,
    height:usize
}

impl<'a,T:Send+'a> CTreeIterator for DownT<'a,T>{
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
