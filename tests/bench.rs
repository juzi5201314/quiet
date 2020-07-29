#![feature(test)]

extern crate test;

use test::Bencher;

use crate::database;

#[bench]
fn bench_get(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(|| runtime.block_on(async {
        let posts = Post::get_all().await?;
        dbg!(posts);
    }));
}