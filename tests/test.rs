#![feature(test)]
#![feature(trusted_len)]

extern crate test;
extern crate compt;
use compt::*;


fn assert_length<I:std::iter::TrustedLen>(it:I){
	assert_eq!(it.size_hint().0,it.size_hint().1.unwrap());

	let len=it.size_hint().0;

	assert_eq!(it.count(),len);
}


#[test]
fn test_length(){
	{
	let mut k=compt::dfs_order::CompleteTreeContainer::<_,compt::dfs_order::InOrder>::from_vec(vec![0,1,2,3,4,5,6]).unwrap();

		assert_length(k.vistr_mut().dfs_preorder_iter().take(3));
		assert_length(k.vistr_mut().bfs_iter().take(3));

		assert_length(k.vistr().dfs_preorder_iter().take(3));
		assert_length(k.vistr().bfs_iter().take(3));
	}
	{
	let mut k=compt::bfs_order::CompleteTreeContainer::from_vec(vec![0,1,2,3,4,5,6]).unwrap();

		assert_length(k.vistr_mut().dfs_preorder_iter().take(3));
		assert_length(k.vistr_mut().bfs_iter().take(3));

		assert_length(k.vistr().dfs_preorder_iter().take(3));
		assert_length(k.vistr().bfs_iter().take(3));
	}
}



#[test]
fn dfs_mut(){

	let mut k=compt::dfs_order::CompleteTreeContainer::<_,compt::dfs_order::InOrder>::from_vec(vec![0,1,2,3,4,5,6]).unwrap();

	let mut res=Vec::new();
	for (a,_) in k.vistr_mut().dfs_preorder_iter(){
		res.push(*a);
	}
	assert_eq!(&res,&[3,1,0,2,5,4,6]);
}



#[test]
fn dfs_inorder_mut(){
	let mut k=compt::dfs_order::CompleteTreeContainer::<_,compt::dfs_order::InOrder>::from_vec(vec![3,1,2,0,4,5,6]).unwrap();

	let mut res=Vec::new();
	for (a,_) in k.vistr_mut().dfs_inorder_iter(){
		res.push(*a);
	}
	assert_eq!(&res,&[3,1,2,0,4,5,6]);
}


#[test]
fn dfs_inorder_mut_backwards(){
	let mut k=compt::dfs_order::CompleteTreeContainer::<_,compt::dfs_order::InOrder>::from_vec(vec![3,1,2,0,4,5,6]).unwrap();

	let mut res=Vec::new();
	for (a,_) in k.vistr_mut().dfs_inorder_iter().collect::<Vec<_>>().iter_mut().rev(){
		res.push(*(*a));
	}
	assert_eq!(&res,&[6,5,4,0,2,1,3]);
}


#[test]
fn dfs_inorder2_mut(){

	let mut k=compt::dfs_order::CompleteTreeContainer::<_,compt::dfs_order::InOrder>::from_vec(vec![3,1,2,0,4,5,6]).unwrap();

	let mut res=Vec::new();
	k.vistr_mut().dfs_inorder(|a,_|res.push(*a));


	assert_eq!(&res,&[3,1,2,0,4,5,6]);
}


#[test]
fn bfs_mut(){
	//       0
	//   1       2
	// 3   4   5    6
	let mut k=compt::bfs_order::CompleteTreeContainer::from_vec(vec![0,1,2,3,4,5,6]).unwrap();

	let mut res=Vec::new();
	k.vistr_mut().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[0,1,3,4,2,5,6]);
}

#[test]
fn dfs(){
	let k=compt::dfs_order::CompleteTreeContainer::<_,compt::dfs_order::InOrder>::from_vec(vec![0,1,2,3,4,5,6]).unwrap();

	let mut res=Vec::new();
	k.vistr().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[3,1,0,2,5,4,6]);
}




#[test]
fn bfs(){
	let k=compt::bfs_order::CompleteTreeContainer::from_vec(vec![0,1,2,3,4,5,6]).unwrap();

	let mut res=Vec::new();
	k.vistr().dfs_preorder(|a,_|{
		res.push(*a);
	});
	assert_eq!(&res,&[0,1,3,4,2,5,6]);
}

