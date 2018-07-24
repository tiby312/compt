#![feature(test)]

extern crate test;
extern crate compt;
use compt::*;

use test::*;



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
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	let mut res=Vec::new();
	k.create_down().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[3,1,0,2,5,4,6]);
}
#[test]
fn bfs(){
	let mut k=compt::bfs_order::GenTree::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

	let mut res=Vec::new();
	k.create_down().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[0,1,3,4,2,5,6]);
}



/*
#[test]
fn test(){

	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},4);

	{
		println!("height={:?}",tree.get_height());
		let c=tree.create_down_mut();

		for (a,i) in c.bfs_iter().enumerate(){
			*i=a
		}
	}
	for b in tree.create_down().bfs_iter(){
		println!("{:?} ",b);
	}
	/*
	let c=Extra{c,extra:1024,func:&|num|{(num/2,num/2)}};

	for (extra,item) in c.bfs_iter(){
		println!("{:?}",extra);
	}
	*/
	assert!(false);
}
*/

/*
#[test]
fn test_parallel(){

	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},4);

	println!("height={:?}",tree.get_height());
	{
		let c=tree.create_down_mut();
		let res=par::in_preorder_parallel(c,|val,rect|{*val=rect;(rect/2,rect/2)},|l,r|{l+r},|val,rect|{*val=rect;rect},Depth(0),1024);
	}
	for b in tree.create_down().bfs_iter(){
		println!("{:?} ",b);
	}

	assert!(false);
}
*/