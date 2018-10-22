#![feature(test)]
#![feature(trusted_len)]

extern crate test;
extern crate compt;
extern crate is_sorted;
use compt::*;
use is_sorted::IsSorted;

use compt::timer::TreeTimerTrait;
use std::time;
use std::thread;
/*
#[test]
fn test_timer(){
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0;255],8).unwrap();
	
	let t=compt::timer::TreeTimer2::new(k.get_height());

	fn recc<T:TreeTimerTrait>(a:compt::dfs_order::DownTMut<isize>,mut tt:T)->T::Bag{
		let dur = time::Duration::from_millis(10);

		tt.start();

		let (_nn,rest) = a.next();
		match rest{
			Some((_extra,left,right))=>{
				thread::sleep(dur);
				let (l,r)=tt.next();
				let a=recc(left,l);
				let b=recc(right,r);
				T::combine(a,b)
			},
			None=>{
				thread::sleep(dur);
				
				tt.leaf_finish()
			}
		}
	}

	let a=recc(k.create_down_mut(),t);

	let res=a.into_iter().collect::<Vec<f64>>();


	res.iter().is_sorted_by(|a,b|a.partial_cmp(b).unwrap());

	//println!("vals={:?}",res);
	//assert!(false);
}
*/

fn assert_length<I:std::iter::TrustedLen>(it:I){
	assert_eq!(it.size_hint().0,it.size_hint().1.unwrap());

	let len=it.size_hint().0;


	assert_eq!(it.count(),len);
}

#[test]
fn test_length(){
	{
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

		assert_length(k.create_down_mut().dfs_preorder_iter().take(3));
		assert_length(k.create_down_mut().bfs_iter().take(3));

		assert_length(k.create_down().dfs_preorder_iter().take(3));
		assert_length(k.create_down().bfs_iter().take(3));
	}
	{
	let mut k=compt::bfs_order::GenTree::from_vec(vec![0,1,2,3,4,5,6],3).unwrap();

		assert_length(k.create_down_mut().dfs_preorder_iter().take(3));
		assert_length(k.create_down_mut().bfs_iter().take(3));

		assert_length(k.create_down().dfs_preorder_iter().take(3));
		assert_length(k.create_down().bfs_iter().take(3));
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
fn dfs_inorder_mut(){
	let mut k=compt::dfs_order::GenTreeDfsOrder::from_vec(vec![3,1,2,0,4,5,6],3).unwrap();

	let mut res=Vec::new();
	for (a,_) in k.create_down_mut().dfs_inorder_iter(){
		res.push(*a);
	}
	assert_eq!(&res,&[3,1,2,0,4,5,6]);
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

