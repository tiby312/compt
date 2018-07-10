use super::*;

///The complete binary tree. Internally stores the elements in a Vec<T> so it is very compact.
///Height is atleast 1.
///Elements stored in BFS order.
///Has 2<sup>k-1</sup> elements where k is the height.
pub struct GenTree<T:Send> {
    nodes: Vec<T>,
    height: usize,
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
            t.dfs_preorder(|node:&mut T,_|{
                *node=func();
            });
        }
        tree
    }
    /*
    ///Create a complete binary tree using the specified node generating function.
    pub fn from_dfs_backwards<F:FnMut()->T>(mut func:F,height:usize)->GenTree<T>{
        assert!(height>=1);
        let mut tree=GenTree::from_bfs(&mut ||{unsafe{std::mem::uninitialized()}},height);
        {
            let t=tree.create_down_mut();
            t.dfs_postorder(|node:&mut T,_|{
                *node=func();
            });
        }
        tree
    }*/

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
    type Extra=();

    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
 
        //Unsafely get a mutable reference to this nodeid.
        //Since at the start there was only one DownTMut that pointed to the root,
        //there is no danger of two DownTMut's producing a reference to the same node.
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        if self.nodeid.0>=self.first_leaf.0{
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(   
                (),  
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
    type Extra=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
 
        let a=&self.remaining.nodes[self.nodeid.0];
        
        if self.nodeid.0>=self.first_leaf.0{
            (a,None)
        }else{
 
            let (l,r)=self.nodeid.get_children();
            
            let j=(     
                (),
                DownT{remaining:self.remaining,nodeid:l,first_leaf:self.first_leaf},
                DownT{remaining:self.remaining,nodeid:r,first_leaf:self.first_leaf}
            );
            (a,Some(j))
        }
    }
 

}

mod cons{
    use super::*;
    struct DownTConsume<'a,T:Send+'a>{
        remaining:*mut GenTree<T>,
        nodeid:NodeIndex,
        first_leaf:NodeIndex,
        phantom:PhantomData<&'a T>
    }

    pub fn downt_into_dfs_preorder<T:Send,F:FnMut(T)>(mut tree:GenTree<T>,mut func:F){
        {
            let t=DownTConsume{remaining:&mut tree,nodeid:NodeIndex(0),first_leaf:NodeIndex::first_leaf(tree.nodes.len()),phantom:PhantomData};
            
            t.dfs_preorder(|a,_|func(a));
        }
        for a in tree.nodes.drain(..){
            std::mem::forget(a);
        }
    }

    impl<'a,T:Send+'a> CTreeIterator for DownTConsume<'a,T>{
        type Item=T;
        type Extra=();
        #[inline(always)]
        fn next(self)->(Self::Item,Option<((),Self,Self)>){
     
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
                    (),   
                    DownTConsume{remaining:self.remaining,nodeid:l,first_leaf:self.first_leaf,phantom:PhantomData},
                    DownTConsume{remaining:self.remaining,nodeid:r,first_leaf:self.first_leaf,phantom:PhantomData}
                );
                (val,Some(j))
            }
        }  
    }
}