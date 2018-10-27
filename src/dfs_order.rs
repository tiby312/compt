use super::*;


///Error indicating the vec that was passed is not a size that you would expect for the given height.
#[derive(Copy,Clone,Debug)]
pub struct NotCompleteTreeSizeErr;

///Complete binary tree stored in DFS inorder order.
///Height is atleast 1.
pub struct CompleteTree<T>{
    nodes: Vec<T>,
    height:usize
}
impl<T> CompleteTree<T>{


    #[inline(always)]
    pub fn from_vec(vec:Vec<T>,height:usize)->Result<CompleteTree<T>,NotCompleteTreeSizeErr>{
        assert!(height>0,"Height must be atleast 1");
        if 2_usize.pow(height as u32)==vec.len()+1{
            Ok(CompleteTree{nodes:vec,height})
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
    pub fn from_dfs_inorder<F:FnMut()->T>(mut func:F,height:usize)->CompleteTree<T>{
        assert!(height>0,"Height must be atleast 1");
        let num=compute_num_nodes(height);
        let nodes=(0..num).map(|_|func()).collect();
        CompleteTree{nodes,height}
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
    pub fn create_down(&self)->Vistr<T>{
        Vistr{remaining:&self.nodes}
    }

    #[inline(always)]
    pub fn create_down_mut(&mut self)->VistrMut<T>{
        VistrMut{remaining:&mut self.nodes}
    }  


    #[inline(always)]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self)->Vec<T>{
        let CompleteTree{nodes,height:_}=self;
        nodes
    }  
}



///Tree visitor that returns a reference to each element in the tree.
pub struct Vistr<'a,T:'a>{
    remaining:&'a [T],
}


impl<'a,T:'a> Vistr<'a,T>{
    #[inline(always)]
    pub fn create_wrap<'b>(&'b self)->Vistr<'b,T>{
        Vistr{remaining:self.remaining}
    }
}
impl<'a,T:'a> Visitor for Vistr<'a,T>{
    type Item=&'a T;
    type NonLeafItem=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
        let remaining=self.remaining;
        if remaining.len()==1{
            (&remaining[0],None)
        }else{
            let mid=remaining.len()/2;
            let (left,rest)=remaining.split_at(mid);
            let (middle,right)=rest.split_first().unwrap();
            (middle,Some(((),Vistr{remaining:left},Vistr{remaining:right})))
        }
    }

    #[inline(always)]
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let left=((self.remaining.len()+1) as f64).log2() as usize;
        (left,Some(left))
    }
}
unsafe impl<'a,T:'a> FixedDepthVisitor for Vistr<'a,T>{}

///Tree visitor that returns a mutable reference to each element in the tree.
pub struct VistrMut<'a,T:'a>{
    remaining:&'a mut [T],
}


impl<'a,T:'a> VistrMut<'a,T>{
    #[inline(always)]
    pub fn create_wrap_mut<'b>(&'b mut self)->VistrMut<'b,T>{
        VistrMut{remaining:self.remaining}
    }
}
unsafe impl<'a,T:'a> FixedDepthVisitor for VistrMut<'a,T>{}

impl<'a,T:'a> Visitor for VistrMut<'a,T>{
    type Item=&'a mut T;
    type NonLeafItem=();
    #[inline(always)]
    fn next(self)->(Self::Item,Option<((),Self,Self)>){
        let remaining=self.remaining;
        if remaining.len()==1{
            (&mut remaining[0],None)
        }else{
            let mid=remaining.len()/2;
            let (left,rest)=remaining.split_at_mut(mid);
            let (middle,right)=rest.split_first_mut().unwrap();
            (middle,Some(((),VistrMut{remaining:left},VistrMut{remaining:right})))
        }
    }

    #[inline(always)]
    fn level_remaining_hint(&self)->(usize,Option<usize>){
        let left=((self.remaining.len()+1) as f64).log2() as usize;
        (left,Some(left))
    }
}