#![feature(test)]

extern crate test;

extern crate compt;
use test::*;
use compt::*;




#[bench]
fn bench_bfs_dfs(bench:&mut Bencher){
	
	let mut k=compt::bfs_order::GenTree::from_vec(vec![0;16383],14).unwrap();
	bench.iter(||{
		for (a,_) in k.create_down_mut().dfs_preorder_iter(){
			*a+=1;
		}
	});

	black_box(k);
}

#[bench]
fn bench_dfs_dfs(bench:&mut Bencher){
	
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0;16383],14).unwrap();
	bench.iter(||{
		for (a,_) in k.create_down_mut().dfs_preorder_iter(){
			*a+=1;
		}
	});


	black_box(k);
}

#[bench]
fn bench_bfs_bfs(bench:&mut Bencher){
	
	let mut k=compt::bfs_order::GenTree::from_vec(vec![0;16383],14).unwrap();
	bench.iter(||{
		for (a,_) in k.create_down_mut().bfs_iter(){
			*a+=1;
		}
	});

	black_box(k);
}

#[bench]
fn bench_dfs_bfs(bench:&mut Bencher){
	
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0;16383],14).unwrap();
	bench.iter(||{
		for (a,_) in k.create_down_mut().bfs_iter(){
			*a+=1;
		}
	});

	black_box(k);
}
