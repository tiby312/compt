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
        DownT{remaining:&self.nodes}
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



///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownT<'a,T:'a>{
    remaining:&'a [T],
}


impl<'a,T:'a> DownT<'a,T>{
    pub fn create_wrap<'b>(&'b self)->DownT<'b,T>{
        DownT{remaining:self.remaining}
    }
}
impl<'a,T:'a> CTreeIterator for DownT<'a,T>{
    type Item=&'a T;
    type Extra=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
        let remaining=self.remaining;
        if remaining.len()==1{
            (&remaining[0],None)
        }else{
            let mid=remaining.len()/2;
            let (left,rest)=remaining.split_at(mid);
            let (middle,right)=rest.split_first().unwrap();
            (middle,Some(((),DownT{remaining:left},DownT{remaining:right})))
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