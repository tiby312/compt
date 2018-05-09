#![feature(test)]

extern crate test;
extern crate compt;
use compt::*;

use test::*;

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
