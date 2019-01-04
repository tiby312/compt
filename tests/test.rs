#![feature(test)]
#![feature(trusted_len)]

extern crate compt;
extern crate test;
use compt::*;

use compt::dfs_order::*;

fn assert_length<I: std::iter::TrustedLen>(it: I) {
    assert_eq!(it.size_hint().0, it.size_hint().1.unwrap());

    let len = it.size_hint().0;

    assert_eq!(it.count(), len);
}

#[test]
fn test_length() {
    {
        let mut k =
            compt::dfs_order::CompleteTreeContainer::from_vec(
                vec![0, 1, 2, 3, 4, 5, 6],InOrder
            )
            .unwrap();

        assert_length(k.vistr_mut().dfs_preorder_iter().take(3));
        assert_length(k.vistr_mut().bfs_iter().take(3));

        assert_length(k.vistr().dfs_preorder_iter().take(3));
        assert_length(k.vistr().bfs_iter().take(3));
    }
    {
        let mut k =
            compt::bfs_order::CompleteTreeContainer::from_vec(vec![0, 1, 2, 3, 4, 5, 6]).unwrap();

        assert_length(k.vistr_mut().dfs_preorder_iter().take(3));
        assert_length(k.vistr_mut().bfs_iter().take(3));

        assert_length(k.vistr().dfs_preorder_iter().take(3));
        assert_length(k.vistr().bfs_iter().take(3));
    }

}




#[test]
fn dfs_mut() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_vec(vec![
            0, 1, 2, 3, 4, 5, 6,
        ],InOrder)
        .unwrap();

    let mut res = Vec::new();
    for a in k.vistr_mut().dfs_preorder_iter() {
        res.push(*a);
    }
    assert_eq!(&res, &[3, 1, 0, 2, 5, 4, 6]);
}

#[test]
fn dfs_inorder_mut() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_vec(vec![
            3, 1, 2, 0, 4, 5, 6,
        ],InOrder)
        .unwrap();

    let mut res = Vec::new();
    for a in k.vistr_mut().dfs_inorder_iter() {
        res.push(*a);
    }
    assert_eq!(&res, &[3, 1, 2, 0, 4, 5, 6]);
}

#[test]
fn dfs_inorder_mut_backwards() {
    let mut k =
        compt::dfs_order::CompleteTreeContainer::from_vec(vec![
            3, 1, 2, 0, 4, 5, 6,
        ],InOrder)
        .unwrap();

    let mut res = Vec::new();
    for a in k
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
        compt::dfs_order::CompleteTreeContainer::from_vec(vec![
            3, 1, 2, 0, 4, 5, 6,
        ],InOrder)
        .unwrap();

    let mut res = Vec::new();
    k.vistr_mut().dfs_inorder(|a| res.push(*a));

    assert_eq!(&res, &[3, 1, 2, 0, 4, 5, 6]);
}

#[test]
fn bfs_mut() {
    //       0
    //   1       2
    // 3   4   5    6
    let mut k =
        compt::bfs_order::CompleteTreeContainer::from_vec(vec![0, 1, 2, 3, 4, 5, 6]).unwrap();

    let mut res = Vec::new();
    k.vistr_mut().dfs_preorder(|a| {
        res.push(*a);
    });
    assert_eq!(&res, &[0, 1, 3, 4, 2, 5, 6]);
}

#[test]
fn dfs() {
    let k =
        compt::dfs_order::CompleteTreeContainer::from_vec(vec![
            0, 1, 2, 3, 4, 5, 6,
        ],InOrder)
        .unwrap();

    let mut res = Vec::new();

    assert_eq!(k.get_height(), 3);

    assert_eq!(k.vistr().level_remaining_hint().0, 3);

    k.vistr().dfs_preorder(|a| {
        res.push(*a);
    });
    assert_eq!(&res, &[3, 1, 0, 2, 5, 4, 6]);
}

#[test]
fn bfs() {
    //    0
    // 1     2
    //3  4  5  6

    let k = compt::bfs_order::CompleteTreeContainer::from_vec(vec![0, 1, 2, 3, 4, 5, 6]).unwrap();

    let mut res = Vec::new();

    assert_eq!(k.get_height(), 3);

    assert_eq!(k.vistr().level_remaining_hint().0, 3);

    k.vistr().dfs_preorder(|a| {
        res.push(*a);
    });
    assert_eq!(&res, &[0, 1, 3, 4, 2, 5, 6]);
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
