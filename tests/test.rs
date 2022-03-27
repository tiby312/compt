extern crate compt;
use compt::*;

#[test]
fn dfs_mut() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_inorder(vec![0, 1, 2, 3, 4, 5, 6]).unwrap();

    let mut res = Vec::new();
    for a in k.as_tree_mut().vistr_mut().dfs_preorder_iter() {
        res.push(*a);
    }
    assert_eq!(&res, &[3, 1, 0, 2, 5, 4, 6]);
}

#[test]
fn dfs_inorder_mut() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_inorder(vec![3, 1, 2, 0, 4, 5, 6]).unwrap();

    let mut res = Vec::new();
    for a in k.as_tree_mut().vistr_mut().dfs_inorder_iter() {
        res.push(*a);
    }
    assert_eq!(&res, &[3, 1, 2, 0, 4, 5, 6]);
}

#[test]
fn dfs_inorder_mut_backwards() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_inorder(vec![3, 1, 2, 0, 4, 5, 6]).unwrap();

    let mut res = Vec::new();
    for a in k
        .as_tree_mut()
        .vistr_mut()
        .dfs_inorder_iter()
        .collect::<Vec<_>>()
        .iter_mut()
        .rev()
    {
        res.push(*(*a));
    }
    assert_eq!(&res, &[6, 5, 4, 0, 2, 1, 3]);
}

#[test]
fn dfs_inorder2_mut() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_inorder(vec![3, 1, 2, 0, 4, 5, 6]).unwrap();

    let mut res = Vec::new();
    k.as_tree_mut().vistr_mut().dfs_inorder(|a| res.push(*a));

    assert_eq!(&res, &[3, 1, 2, 0, 4, 5, 6]);
}

#[test]
fn dfs() {
    let k =
        compt::dfs_order::CompleteTreeContainer::from_inorder(vec![0, 1, 2, 3, 4, 5, 6]).unwrap();

    let mut res = Vec::new();

    assert_eq!(k.as_tree().get_height(), 3);

    assert_eq!(k.as_tree().vistr().level_remaining_hint().0, 3);

    k.as_tree().vistr().dfs_preorder(|a| {
        res.push(*a);
    });
    assert_eq!(&res, &[3, 1, 0, 2, 5, 4, 6]);
}

/*
#[test]
fn test_derefs(){
    let mut k=compt::bfs_order::CompleteTreeContainer::from_vec(vec![0usize,1,2,3,4,5,6]).unwrap();


    let k:&compt::bfs_order::Vistr<usize>=&k.vistr_mut();

    let ans:Vec<_>=k.dfs_preorder_iter().map(|(a,_)|*a).collect();

    assert_eq!(ans,&[0usize,1,3,4,2,5,6]);
}
*/
