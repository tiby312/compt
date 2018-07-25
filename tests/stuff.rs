#![feature(test)]
#![feature(trusted_len)]

extern crate test;
extern crate compt;
use compt::*;



fn assert_length<I:std::iter::TrustedLen>(it:I){
	let len=it.size_hint().0;

	assert_eq!(it.count(),len);
}

#[test]
fn test_length(){
	{
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	assert_length(k.create_down_mut().dfs_preorder_iter());
	assert_length(k.create_down_mut().bfs_iter());

	assert_length(k.create_down().dfs_preorder_iter());
	assert_length(k.create_down().bfs_iter());
	}
	{
	let mut k=compt::bfs_order::GenTree::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	assert_length(k.create_down_mut().dfs_preorder_iter());
	assert_length(k.create_down_mut().bfs_iter());

	assert_length(k.create_down().dfs_preorder_iter());
	assert_length(k.create_down().bfs_iter());
	}
}



#[test]
fn dfs_mut(){
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	let mut res=Vec::new();
	for (a,_) in k.create_down_mut().dfs_preorder_iter(){
		res.push(*a);
	}
	assert_eq!(&res,&[3,1,0,2,5,4,6]);
}
#[test]
fn bfs_mut(){
	let mut k=compt::bfs_order::GenTree::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	let mut res=Vec::new();
	k.create_down_mut().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[0,1,3,4,2,5,6]);
}

#[test]
fn dfs(){
	let k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	let mut res=Vec::new();
	k.create_down().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[3,1,0,2,5,4,6]);
}
#[test]
fn bfs(){
	let k=compt::bfs_order::GenTree::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	let mut res=Vec::new();
	k.create_down().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[0,1,3,4,2,5,6]);
}

