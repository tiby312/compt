use std::marker::PhantomData;
//use std;


pub struct TreeProp{
    height:usize,
    num_nodes:usize
}



#[derive(Debug,Clone)]
pub struct GenTree<T> {
    nodes: Vec<T>,
    height: usize,
}

pub fn compute_num_nodes(height:usize)->usize{
    return (1 << height) - 1;
}

impl<T> GenTree<T> {
    
    pub fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn create_tree_prop(&self)->TreeProp{
        TreeProp{height:self.height,num_nodes:self.nodes.len()}
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn from_vec(nodes:Vec<T>,height:usize)->GenTree<T>{
        assert!(height>=1);
        assert!( nodes.len() == self::compute_num_nodes(height));

        GenTree {
            nodes: nodes,
            height: height,
        } 
    }

    pub fn get_root_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.nodes.get_unchecked_mut(0) }
    }

    pub fn get_root<'a>(&'a self) -> &'a T {
        unsafe { self.nodes.get_unchecked(0) }
    }
    
    pub fn create_down_mut(&mut self)->DownTMut<T>{
        DownTMut{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height},phantom:PhantomData}
    }
    pub fn create_down(&self)->DownT<T>{
        DownT{remaining:self,nodeid:NodeIndex(0),leveld:LevelDesc{depth:0,height:self.height}}
    }


    //todo make a dfs_mut
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



    
    //TODO verify this
    //go in order
    pub fn dfs_consume<F:FnMut(T)>(mut self,func:&mut F){
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





#[derive(Copy,Clone,Debug)]
struct NodeIndex(pub usize); //todo dont make private

impl NodeIndex{
    fn get_children(self) -> (NodeIndex, NodeIndex) {
        let NodeIndex(a) = self;
        (NodeIndex(2 * a + 1), NodeIndex(2 * a + 2))
    }
}




//Safe to traverse downwards and get a mutable reference to every element
pub struct DownT<'a,T:'a>{
    remaining:&'a GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc
}

impl<'a,T> DownT<'a,T>{

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
    pub fn get_level(&self)->&LevelDesc{
        &self.leveld
    }
    pub fn get(&self)->&T{
        &self.remaining.nodes[self.nodeid.0]
    }
}

unsafe impl<'a,T:'a> std::marker::Sync for DownTMut<'a,T>{}
unsafe impl<'a,T:'a> std::marker::Send for DownTMut<'a,T>{}

//Safe to traverse downwards and get a mutable reference to every element
pub struct DownTMut<'a,T:'a>{
    remaining:*mut GenTree<T>,
    nodeid:NodeIndex,
    leveld:LevelDesc,
    phantom:PhantomData<&'a T>
}


impl<'a,T:'a> DownTMut<'a,T>{

    pub fn get_mut(&mut self)->&mut T{
        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        a
    }
    pub fn next<'c>(&'c mut self)->(&'c mut T,Option<(DownTMut<'c,T>,DownTMut<'c,T>)>){

        let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
        
        if self.leveld.is_leaf(){
            //let a=unsafe{&mut (*self.remaining).nodes[self.nodeid.0]};
            return (a,None)
        }

        let (l,r)=self.nodeid.get_children();
        
        (a,Some((     
            DownTMut{remaining:self.remaining,nodeid:l,leveld:self.leveld.next_down(),phantom:PhantomData},
            DownTMut{remaining:self.remaining,nodeid:r,leveld:self.leveld.next_down(),phantom:PhantomData}
        )))
    }

    pub fn get_level(&self)->&LevelDesc{
        &self.leveld
    }
}

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
    #[test]
    fn it_works() {
    }
}
