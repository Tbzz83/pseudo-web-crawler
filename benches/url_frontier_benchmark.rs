use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pseudo_web_crawler::url_frontier_aos::{Url as UrlAos, UrlFrontier as UrlFrontierAos};
use pseudo_web_crawler::url_frontier_oop::{Url as UrlOop, UrlFrontier as UrlFrontierOop};
use pseudo_web_crawler::url_frontier_soa::{Url as UrlSoa, UrlFrontier as UrlFrontierSoa};

fn create_oop_frontier(size: usize) -> UrlFrontierOop {
    let mut frontier = UrlFrontierOop::with_capacity(size);
    for i in 0..size {
        frontier.push_url(UrlOop::new(&format!("https://example{}.com", i)));
    }
    frontier
}

fn create_aos_frontier(size: usize) -> UrlFrontierAos {
    let mut frontier = UrlFrontierAos::with_capacity(size);
    for i in 0..size {
        frontier.push_url(UrlAos::new(&format!("https://example{}.com", i)));
    }
    frontier
}

fn create_soa_frontier(size: usize) -> UrlFrontierSoa {
    let mut frontier = UrlFrontierSoa::with_capacity(size);
    for i in 0..size {
        frontier.push_url(UrlSoa::new(&format!("https://example{}.com", i)));
    }
    frontier
}

fn benchmark_oop_prioritize(c: &mut Criterion) {
    let mut group = c.benchmark_group("OOP Layout");

    for size in [10000, 100000, 1000000, 5000000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut frontier = create_oop_frontier(size);
            b.iter(|| black_box(frontier.prioritize_urls()));
        });
    }

    group.finish();
}

fn benchmark_aos_prioritize(c: &mut Criterion) {
    let mut group = c.benchmark_group("AOS Layout");

    for size in [10000, 100000, 1000000, 5000000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let frontier = create_aos_frontier(size);
            b.iter(|| black_box(frontier.prioritize_urls()));
        });
    }

    group.finish();
}

fn benchmark_soa_prioritize(c: &mut Criterion) {
    let mut group = c.benchmark_group("SOA Layout");

    for size in [10000, 100000, 1000000, 5000000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let frontier = create_soa_frontier(size);
            b.iter(|| black_box(frontier.prioritize_urls()));
        });
    }

    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("OOP vs AOS vs SOA Comparison");
    let size = 2000000;

    group.bench_function("OOP", |b| {
        let mut frontier = create_oop_frontier(size);
        b.iter(|| black_box(frontier.prioritize_urls()));
    });

    group.bench_function("AOS", |b| {
        let frontier = create_aos_frontier(size);
        b.iter(|| black_box(frontier.prioritize_urls()));
    });

    group.bench_function("SOA", |b| {
        let frontier = create_soa_frontier(size);
        b.iter(|| black_box(frontier.prioritize_urls()));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_oop_prioritize,
    benchmark_aos_prioritize,
    benchmark_soa_prioritize,
    benchmark_comparison
);
criterion_main!(benches);
