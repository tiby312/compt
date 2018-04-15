#![feature(test)]

extern crate test;
extern crate compt;
use compt::*;

use test::*;
#[bench]
fn bench_dynamic_iter(a:&mut Bencher){
	
	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},18);

	a.iter(||{
		let i=tree.create_down_mut();
		for item in i.dfs_preorder_iter(){
			*item=5;
		}
	});
}

#[bench]
fn bench_dynamic_rec(a:&mut Bencher){
	
	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},18);

	a.iter(||{
		let i=tree.create_down_mut();
		i.dfs_preorder(|item|{
			*item=5;
		});
	});
}


#[bench]
fn bench_static(a:&mut Bencher){
	
	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},18);
	

	fn recc<'a,A:par::AxisTrait,C:CTreeIterator<Item=&'a mut usize>>(axis:A,a:C){
		
		let (item,rest)=a.next();

		*item=5;

		match rest{
			Some((left,right))=>{

				recc(axis.next(),left);

				recc(axis.next(),right);

			},
			None=>{}
		}	}


	a.iter(||{
		let i=tree.create_down_mut();
		recc(par::XAXIS,i);
	});
}