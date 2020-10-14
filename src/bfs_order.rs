use super::*;
use alloc::boxed::Box;

///Error indicating the vec that was passed is not a size that you would expect for the given height.
#[derive(Copy, Clone, Debug)]
pub struct NotCompleteTreeSizeErr;

///Contains of a Complete tree. Internally uses a Vec.
pub struct CompleteTreeContainer<T> {
    nodes: Box<[T]>,
}
impl<T> CompleteTreeContainer<T> {
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> Result<CompleteTreeContainer<T>, NotCompleteTreeSizeErr> {
        if valid_node_num(vec.len()) {
            Ok(CompleteTreeContainer { nodes: vec.into_boxed_slice() })
        } else {
            Err(NotCompleteTreeSizeErr)
        }
    }

    #[inline]
    ///Returns the underlying elements as they are, in BFS order.
    pub fn into_nodes(self) -> Box<[T]> {
        let CompleteTreeContainer { nodes } = self;
        nodes
    }
}

impl<T> core::ops::Deref for CompleteTreeContainer<T> {
    type Target = CompleteTree<T>;
    fn deref(&self) -> &CompleteTree<T> {
        unsafe { &*(&self.nodes as &[T] as *const [T] as *const bfs_order::CompleteTree<T>) }
    }
}
impl<T> core::ops::DerefMut for CompleteTreeContainer<T> {
    fn deref_mut(&mut self) -> &mut CompleteTree<T> {
        unsafe { &mut *(&mut self.nodes as &mut [T] as *mut [T] as *mut bfs_order::CompleteTree<T>) }
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
        Vistr {
            current: 0,
            arr: &self.nodes,
        }
    }

    #[inline]
    ///Create a mutable visitor struct
    pub fn vistr_mut(&mut self) -> VistrMut<T> {
        VistrMut {
            current: 0,
            arr: &mut self.nodes,
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
    arr: &'a mut [T],
}

unsafe impl<'a, T: 'a> FixedDepthVisitor for VistrMut<'a, T> {}

impl<'a, T: 'a> Visitor for VistrMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let arr_left =
            unsafe { core::slice::from_raw_parts_mut(self.arr.as_mut_ptr(), self.arr.len()) };
        let arr_right =
            unsafe { core::slice::from_raw_parts_mut(self.arr.as_mut_ptr(), self.arr.len()) };

        let len = self.arr.len();
        let curr = &mut self.arr[self.current];

        if self.current >= len / 2 {
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
                    arr: arr_left,
                },
                VistrMut {
                    current: right,
                    arr: arr_right,
                },
            ];
            (curr, Some(j))
        }
    }
    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        let depth = compute_height(self.current);
        let height = compute_height(self.arr.len());
        let diff = height - depth;
        (diff, Some(diff))
    }
}

impl<'a, T> core::ops::Deref for VistrMut<'a, T> {
    type Target = Vistr<'a, T>;
    fn deref(&self) -> &Vistr<'a, T> {
        unsafe { &*(self as *const VistrMut<T> as *const Vistr<T>) }
    }
}

//                    a
//          b                  b
//      c        c         c       c
//    d   d   d    d     d   d   d   d
//   e e e e e  e e e   e e e e e e e e
//
//  a bb cccc dddddddd
//

///Tree visitor that returns a mutable reference to each element in the tree.

///Tree visitor that returns a mutable reference to each element in the tree.
pub struct Vistr<'a, T: 'a> {
    current: usize,
    arr: &'a [T],
}

unsafe impl<'a, T: 'a> FixedDepthVisitor for Vistr<'a, T> {}

impl<'a, T: 'a> Visitor for Vistr<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(self) -> (Self::Item, Option<[Self; 2]>) {
        let len = self.arr.len();
        let curr = &self.arr[self.current];

        if self.current >= len / 2 {
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
                    arr: self.arr,
                },
                Vistr {
                    current: right,
                    arr: self.arr,
                },
            ];
            (curr, Some(j))
        }
    }
    #[inline]
    fn level_remaining_hint(&self) -> (usize, Option<usize>) {
        let depth = compute_height(self.current);
        let height = compute_height(self.arr.len());
        let diff = height - depth;
        (diff, Some(diff))
    }
}
