use super::*;
use std::marker::PhantomData;

///Error indicating the vec that was passed is not a size that you would expect for the given height.
#[derive(Copy, Clone, Debug)]
pub struct NotCompleteTreeSizeErr;

///Contains of a Complete tree. Internally uses a Vec.
pub struct CompleteTreeContainer<T> {
    nodes: Vec<T>,
}
impl<T> CompleteTreeContainer<T> {
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> Result<CompleteTreeContainer<T>, NotCompleteTreeSizeErr> {
        if valid_node_num(vec.len()) {
            Ok(CompleteTreeContainer { nodes: vec })
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self) -> Vec<T> {
        let CompleteTreeContainer { nodes } = self;
        nodes
    }
}

impl<T> std::ops::Deref for CompleteTreeContainer<T> {
    type Target = CompleteTree<T>;
    fn deref(&self) -> &CompleteTree<T> {
        unsafe { &*(self.nodes.as_slice() as *const [T] as *const bfs_order::CompleteTree<T>) }
    }
}
impl<T> std::ops::DerefMut for CompleteTreeContainer<T> {
    fn deref_mut(&mut self) -> &mut CompleteTree<T> {
        unsafe { &mut *(self.nodes.as_mut_slice() as *mut [T] as *mut bfs_order::CompleteTree<T>) }
    }
}

///Complete binary tree stored in BFS order.
///Height is atleast 1.
pub struct CompleteTree<T> {
    nodes: [T],
}

impl<T> CompleteTree<T> {
    #[inline]
    pub fn from_slice(arr: &[T]) -> Result<&CompleteTree<T>, NotCompleteTreeSizeErr> {
        if valid_node_num(arr.len()) {
            let tree = unsafe { &*(arr as *const [T] as *const bfs_order::CompleteTree<T>) };
            Ok(tree)
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline]
    pub fn from_slice_mut(arr: &mut [T]) -> Result<&mut CompleteTree<T>, NotCompleteTreeSizeErr> {
        if valid_node_num(arr.len()) {
            let tree = unsafe { &mut *(arr as *mut [T] as *mut bfs_order::CompleteTree<T>) };
            Ok(tree)
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        compute_height(self.nodes.len())
    }

    #[inline]
    ///Create a immutable visitor struct
    pub fn vistr(&self) -> Vistr<T> {
        let base = self.nodes.as_ptr();
        Vistr {
            current: 0,
            base,
            depth: 0,
            _p: PhantomData,
            height: self.get_height(),
        }
    }

    #[inline]
    ///Create a mutable visitor struct
    pub fn vistr_mut(&mut self) -> VistrMut<T> {
        let base = std::ptr::Unique::new(self.nodes.as_mut_ptr()).unwrap();
        VistrMut {
            current: 0,
            base,
            depth: 0,
            _p: PhantomData,
            height: self.get_height(),
        }
    }

    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn get_nodes(&self) -> &[T] {
        &self.nodes
    }

    #[inline]
    pub fn get_nodes_mut(&mut self) -> &mut [T] {
        &mut self.nodes
    }
}

///Visitor functions use this type to determine what node to visit.
///The nodes in the tree are kept in the tree in BFS order.
#[derive(Copy, Clone, Debug)]
struct NodeIndex(usize);


///Tree visitor that returns a mutable reference to each element in the tree.
pub struct VistrMut<'a, T: 'a> {
    current: usize,
    base: std::ptr::Unique<T>,
    depth: usize,
    height: usize,
    _p: PhantomData<&'a mut T>,
}

unsafe impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T> {}

impl<'a, T: 'a> Visitor for VistrMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let curr = unsafe { &mut *self.base.as_ptr().add(self.current) };
        //Unsafely get a mutable reference to this nodeid.
        //Since at the start there was only one VistrMut that pointed to the root,
        //there is no danger of two VistrMut's producing a reference to the same node.
        if self.depth == self.height - 1 {
            (curr, None)
        } else {
            let (left, right) = {
                let left = 2 * self.current + 1;
                let right = 2 * self.current + 2;
                (left, right)
            };

            let j = [
                VistrMut {
                    current: left,
                    base: self.base,
                    depth: self.depth + 1,
                    height: self.height,
                    _p: PhantomData,
                },
                VistrMut {
                    current: right,
                    base: self.base,
                    depth: self.depth + 1,
                    height: self.height,
                    _p: PhantomData,
                },
            ];
            (curr, Some(j))
        }
    }
    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        let diff = self.height - self.depth;
        (diff, Some(diff))
    }
}

impl<'a, T> std::ops::Deref for VistrMut<'a, T> {
    type Target = Vistr<'a, T>;
    fn deref(&self) -> &Vistr<'a, T> {
        unsafe { &*(self as *const VistrMut<T> as *const Vistr<T>) }
    }
}

///Tree visitor that returns a mutable reference to each element in the tree.
pub struct Vistr<'a, T: 'a> {
    current: usize,
    base: *const T,
    depth: usize,
    height: usize,
    _p: PhantomData<&'a T>,
}

unsafe impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T> {}

impl<'a, T: 'a> Visitor for Vistr<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let curr = unsafe { &*self.base.add(self.current) };
        //Unsafely get a mutable reference to this nodeid.
        //Since at the start there was only one VistrMut that pointed to the root,
        //there is no danger of two VistrMut's producing a reference to the same node.
        if self.depth == self.height - 1 {
            (curr, None)
        } else {
            let (left, right) = {
                let left = 2 * self.current + 1;
                let right = 2 * self.current + 2;
                (left, right)
            };

            let j = [
                Vistr {
                    current: left,
                    base: self.base,
                    depth: self.depth + 1,
                    height: self.height,
                    _p: PhantomData,
                },
                Vistr {
                    current: right,
                    base: self.base,
                    depth: self.depth + 1,
                    height: self.height,
                    _p: PhantomData,
                },
            ];
            (curr, Some(j))
        }
    }
    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        let diff = self.height - self.depth;
        (diff, Some(diff))
    }
}
