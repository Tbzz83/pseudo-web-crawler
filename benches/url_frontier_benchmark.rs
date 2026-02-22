use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pseudo_web_crawler::url_frontier_aos::{Url as UrlAos, UrlFrontier as UrlFrontierAos};
use pseudo_web_crawler::url_frontier_oop::{Url as UrlOop, UrlFrontier as UrlFrontierOop};

fn create_oop_frontier(size: usize) -> UrlFrontierOop {
    let mut frontier = UrlFrontierOop::new();
    for i in 0..size {
        frontier.push_url(UrlOop::new(&format!("https://example{}.com", i)));
    }
    frontier
}

fn create_aos_frontier(size: usize) -> UrlFrontierAos {
    let mut frontier = UrlFrontierAos::new();
    for i in 0..size {
        frontier.push_url(UrlAos::new(&format!("https://example{}.com", i)));
    }
    frontier
}

fn benchmark_oop_prioritize(c: &mut Criterion) {
    let mut group = c.benchmark_group("OOP Layout");

    for size in [1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut frontier = create_oop_frontier(size);
            b.iter(|| {
                frontier.prioritize_urls();
            });
        });
    }

    group.finish();
}

fn benchmark_aos_prioritize(c: &mut Criterion) {
    let mut group = c.benchmark_group("AOS Layout");

    for size in [1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut frontier = create_aos_frontier(size);
            b.iter(|| {
                frontier.prioritize_urls();
            });
        });
    }

    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("OOP vs AOS Comparison");
    let size = 50000;

    group.bench_function("OOP", |b| {
        let mut frontier = create_oop_frontier(size);
        b.iter(|| {
            frontier.prioritize_urls();
        });
    });

    group.bench_function("AOS", |b| {
        let mut frontier = create_aos_frontier(size);
        b.iter(|| {
            frontier.prioritize_urls();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_oop_prioritize,
    benchmark_aos_prioritize,
    benchmark_comparison
);
criterion_main!(benches);
