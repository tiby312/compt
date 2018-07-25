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


/*
macro_rules! gen_test{
	($test:ident,$bench:ident,$function:ident)=>{
		#[test]
		fn $test(){

			let mut k=&tree::from_vec(vec![0,1,2,3,4,5,6,7]);

			let count=(0..);
			k.create_down_mut().dfs_preorder(|(a,_)|{
				*a=count.next().unwrap();
			})

		}

		#[bench]
		fn $bench(bench:&mut Bencher){
			let mut k=[0;4000];
			bench.iter(||{
				$function(&mut k,|a,b|{
					*a+=1;
					*b+=1;
				})
			});
			black_box(k);
		}
	}
}

gen_test!(iter_test,iter_bench,for_every_pair_iter);
gen_test!(recc_test,recc_bench,for_every_pair_recc);
gen_test!(unsafe_test,unsafe_bench,for_every_pair_unsafe_impl);
*/