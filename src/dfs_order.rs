use super::*;

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
        DownTMut{remaining:&mut self.nodes}
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
    type Extra=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
 
        let a=&self.remaining.nodes[self.nodeid.0];
        
        if self.span==0{
            (a,None)
        }else{
            let (l,r)=self.nodeid.get_children(self.span);
            
            let j=( 
                (),    
                DownT{remaining:self.remaining,nodeid:l,span:self.span/2},
                DownT{remaining:self.remaining,nodeid:r,span:self.span/2}
            );
            (a,Some(j))
        }
    }
 

}
///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:'a>{
    remaining:&'a mut [T],
}


impl<'a,T:'a> DownTMut<'a,T>{
    pub fn create_wrap_mut<'b>(&'b mut self)->DownTMut<'b,T>{
        DownTMut{remaining:self.remaining}
    }
}
impl<'a,T:'a> CTreeIterator for DownTMut<'a,T>{
    type Item=&'a mut T;
    type Extra=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
        let remaining=self.remaining;
        if remaining.len()==1{
            (&mut remaining[0],None)
        }else{
            let mid=remaining.len()/2;
            let (left,rest)=remaining.split_at_mut(mid);
            let (middle,right)=rest.split_first_mut().unwrap();
            (middle,Some(((),DownTMut{remaining:left},DownTMut{remaining:right})))
        }
    }
}
/*
///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:'a>{
    remaining:*mut GenTreeDfsOrder<T>,
    nodeid:NodeIndexDfs,
    span:usize,
    phantom:PhantomData<&'a mut T>
}


impl<'a,T:'a> DownTMut<'a,T>{
    pub fn create_wrap_mut<'b>(&'b mut self)->DownTMut<'b,T>{
        DownTMut{remaining:self.remaining,nodeid:self.nodeid,span:self.span,phantom:self.phantom}
    }
}
impl<'a,T:'a> CTreeIterator for DownTMut<'a,T>{
    type Item=&'a mut T;
    type Extra=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
        
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
                (),   
                DownTMut{remaining:self.remaining,nodeid:l,span:self.span/2,phantom:PhantomData},
                DownTMut{remaining:self.remaining,nodeid:r,span:self.span/2,phantom:PhantomData}
            );
            (a,Some(j))
        }
    }
}
*/