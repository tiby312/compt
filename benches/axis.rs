#![feature(test)]

extern crate test;
extern crate compt;
use compt::*;

use test::*;

fn dostuff(a:bool){
	let mut bla=0;
	for i in 0..10{
		if !a{
			for j in 0..10{
				if a{
					bla+=j-i;
				}
			}
		}
	}
	black_box(bla);
}
#[bench]
fn bench_dynamic_iter(a:&mut Bencher){
	
	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},12);

	a.iter(||{
		let i=tree.create_down_mut().with_axis(TAxis::XAXIS);
		for (is_xaxis,item) in i.dfs_preorder_iter(){
			if is_xaxis.is_xaxis(){
				*item=5;
			}else{
				*item=3;
			}

			dostuff(is_xaxis.is_xaxis());
		}
	});
}

#[bench]
fn bench_dynamic_rec(a:&mut Bencher){
	
	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},12);

	a.iter(||{
		let i=tree.create_down_mut().with_axis(TAxis::XAXIS);
		i.dfs_preorder(|(is_xaxis,item)|{
			if is_xaxis.is_xaxis(){
				*item=5;
			}else{
				*item=3;
			}


			dostuff(is_xaxis.is_xaxis());
		});
	});
}


#[bench]
fn bench_static(a:&mut Bencher){
	
	let mut tree=dfs::GenTreeDfsOrder::from_dfs_inorder(||{0},12);
	

	fn recc<'a,A:par::AxisTrait,C:CTreeIterator<Item=&'a mut usize>>(axis:A,a:C){
		
		let (item,rest)=a.next();

		if axis.is_xaxis(){
			*item=5;
		}else{
			*item=3;
		}


		dostuff(axis.is_xaxis());

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