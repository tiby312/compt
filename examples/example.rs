extern crate compt;
use compt::*;

fn main() {
    // Example that performs dfs in order traversal on the left side,
    // and bfs order traversal on the right side of a tree.
    // This demonstrates the composability of the different visitor functions.
    //
    //       0
    //   1       2
    // 3   4   5    6
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_inorder(vec![3, 1, 4, 0, 5, 2, 6]).unwrap();

    let mut tree = k.as_tree_mut();
    let k = tree.vistr_mut();
    let (a, rest) = k.next();
    let [left, right] = rest.unwrap();

    let mut res: Vec<&mut usize> = Vec::new();
    res.push(a);

    left.dfs_inorder(|a| {
        res.push(a);
    });

    for a in right.dfs_preorder_iter() {
        res.push(a);
    }

    let res: Vec<usize> = res.drain(..).map(|a| *a).collect();
    assert_eq!(&res, &[0, 3, 1, 4, 2, 5, 6]);
}
