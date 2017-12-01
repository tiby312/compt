## Summary
A Complete Binary Tree library.
It is internally represented as a 1D vec.
Provides a way to get mutable references to children nodes simultaneously. Useful for parallelizing divide and conquer style problems.
There is no api to add and remove nodes. The existence of the tree implies that 2k-1 elements already exist. It is a full tree.
Provides tree visitors that implement the below trait. They can be combined together using zip().

```
pub trait CTreeIterator:Sized{
    type Item;
    ///Consume this visitor, and produce the element it was pointing to
    ///along with it's children visitors.
    fn next(self)->(Self::Item,Option<(Self,Self)>);
}
```

## Goals

To create a safe and compact complete binary tree data structure that provides an api
that parallel algorithms can exploit.

## Unsafety

With a regular slice, getting one mutable reference to an element will borrow the
entire slice. The slice that GenTree uses, however, internally has the invariant that it is laid out
in BFS order. Therefore one can safely assume that if (starting at the root),
one had a mutable reference to a parent k, and one were to get the children using 2k+1 and 2k+2
to get *two* mutable references to the children,
they would be guarenteed to be distinct (from each other and also the parent) despite the fact that they belong to the same slice.

## Example
```
extern crate compt;
fn main()
{
        use compt::CTreeIterator;
        //Create a tree of height 2 with elemenets set to zero.
        let mut tree=compt::GenTree::from_bfs(||0,2);
        {
            //Create a mutable tree visitor.
            let mut down=tree.create_down_mut();
            //Call the iterator's next() function.
            let (e,maybe_children)=down.next();
            //Set the root to 1.
            *e=1;
            //Set the children to 2 and 3.
            let (mut left,mut right)=maybe_children.unwrap();
            *left.next().0=2;
            *right.next().0=3;
        }
        {
            //Create a immutable tree visitor.
            let down=tree.create_down();
            //Iterate dfs over our constructed tree.
            let mut v=Vec::new();
            down.dfs_postorder(|a|{
                 v.push(*a);
            });
            assert_eq!(v,vec!(3,2,1));
        }
}
```

