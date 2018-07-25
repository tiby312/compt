use super::*;


///Error indicating the vec that was passed is not a size that you would expect for the given height.
pub struct NotCompleteTreeSizeErr;

///Complete binary tree stored in DFS inorder order.
///Height is atleast 1.
pub struct GenTreeDfsOrder<T>{
    nodes: Vec<T>,
    height:usize
}
impl<T> GenTreeDfsOrder<T>{


    #[inline(always)]
    pub fn from_vec(vec:Vec<T>,height:usize)->Result<GenTreeDfsOrder<T>,NotCompleteTreeSizeErr>{
        assert!(height>0,"Height must be atleast 1");
        if 2_usize.pow(height as u32)==vec.len()+1{
            Ok(GenTreeDfsOrder{nodes:vec,height})
        }else{
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline(always)]
    pub fn get_height(&self) -> usize {
        self.height
    }

    ///Create a complete binary tree using the specified node generating function.
    
    #[inline(always)]
    pub fn from_dfs_inorder<F:FnMut()->T>(mut func:F,height:usize)->GenTreeDfsOrder<T>{
        assert!(height>0,"Height must be atleast 1");
        let num=compute_num_nodes(height);
        let nodes=(0..num).map(|_|func()).collect();
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
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self)->Vec<T>{
        let GenTreeDfsOrder{nodes,height:_}=self;
        nodes
    }  
}



///Tree visitor that returns a reference to each element in the tree.
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

    #[inline(always)]
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let left=((self.remaining.len()+1) as f64).log2() as usize;
        (left,Some(left))
    }
}
unsafe impl<'a,T:'a> FixedDepthCTreeIterator for DownT<'a,T>{}

///Tree visitor that returns a mutable reference to each element in the tree.
pub struct DownTMut<'a,T:'a>{
    remaining:&'a mut [T],
}


impl<'a,T:'a> DownTMut<'a,T>{
    pub fn create_wrap_mut<'b>(&'b mut self)->DownTMut<'b,T>{
        DownTMut{remaining:self.remaining}
    }
}
unsafe impl<'a,T:'a> FixedDepthCTreeIterator for DownTMut<'a,T>{}

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

    #[inline(always)]
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let left=((self.remaining.len()+1) as f64).log2() as usize;
        (left,Some(left))
    }
}