use criterion::{criterion_group, criterion_main, Criterion};
use crossbeam::sync::WaitGroup;
use std::error::Error;
use std::thread;
use visdom::Vis;

fn bench_selector() -> Result<(), Box<dyn Error>> {
	let html: String = format!(
		r##"
      <ul>
        {}
      </ul>
    "##,
		String::from("<li></li>").repeat(1000)
	);
	let root = Vis::load(&html)?;
	let list = root.children("ul");
	list.find("li:nth-child(2n)");
	Ok(())
}
fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("bench-selector", |b| b.iter(bench_selector));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
