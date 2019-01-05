#![feature(test)]

extern crate test;

extern crate compt;
use compt::*;
use test::*;

#[bench]
fn bench_bfs_dfs(bench: &mut Bencher) {
    let mut k = compt::bfs_order::CompleteTreeContainer::from_vec(vec![0; 16383]).unwrap();
    bench.iter(|| {
        for a in k.vistr_mut().dfs_preorder_iter() {
            *a += 1;
        }
    });

    black_box(k);
}

#[bench]
fn bench_dfs_dfs(bench: &mut Bencher) {
    let mut k = compt::dfs_order::CompleteTreeContainer::from_preorder(
        vec![0; 16383],
    )
    .unwrap();
    bench.iter(|| {
        for a in k.vistr_mut().dfs_preorder_iter() {
            *a += 1;
        }
    });

    black_box(k);
}

#[bench]
fn bench_bfs_bfs(bench: &mut Bencher) {
    let mut k = compt::bfs_order::CompleteTreeContainer::from_vec(vec![0; 16383]).unwrap();
    bench.iter(|| {
        for a in k.vistr_mut().bfs_iter() {
            *a += 1;
        }
    });

    black_box(k);
}

#[bench]
fn bench_dfs_bfs(bench: &mut Bencher) {
    let mut k = compt::dfs_order::CompleteTreeContainer::from_preorder(
        vec![0; 16383],
    )
    .unwrap();
    bench.iter(|| {
        for a in k.vistr_mut().bfs_iter() {
            *a += 1;
        }
    });

    black_box(k);
}
