// benches/quality_checks.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use prism::quality::{check_alternating_pattern, check_block_pattern, check_diagonal_pattern};

fn generate_item_values(n: usize, pattern: &str) -> Vec<f64> {
    match pattern {
        "diagonal" => (1..=n).map(|i| ((i % 7) + 1) as f64).collect(),
        "alternating" => (1..=n)
            .map(|i| if i % 2 == 0 { 7.0 } else { 1.0 })
            .collect(),
        "block" => (1..=n).map(|i| if i < n / 2 { 1.0 } else { 7.0 }).collect(),
        "random" => (1..=n).map(|i| ((i * 13 % 7) + 1) as f64).collect(),
        _ => vec![1.0; n],
    }
}

fn bench_diagonal_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagonal_pattern");

    for n in [5, 10, 20, 50].iter() {
        let values = generate_item_values(*n, "diagonal");
        let quality_flags = Vec::new();
        let quality_issues = Vec::new();

        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, _| {
            b.iter(|| {
                check_diagonal_pattern(
                    black_box("test_scale"),
                    black_box(&values),
                    black_box("P001"),
                    black_box(&mut quality_flags.clone()),
                    black_box(&mut quality_issues.clone()),
                );
            });
        });
    }

    group.finish();
}

fn bench_alternating_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("alternating_pattern");

    for n in [5, 10, 20, 50].iter() {
        let values = generate_item_values(*n, "alternating");
        let quality_flags = Vec::new();
        let quality_issues = Vec::new();

        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, _| {
            b.iter(|| {
                check_alternating_pattern(
                    black_box("test_scale"),
                    black_box(&values),
                    black_box("P001"),
                    black_box(&mut quality_flags.clone()),
                    black_box(&mut quality_issues.clone()),
                );
            });
        });
    }

    group.finish();
}

fn bench_block_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_pattern");

    for n in [6, 10, 20, 50].iter() {
        let values = generate_item_values(*n, "block");
        let quality_flags = Vec::new();
        let quality_issues = Vec::new();

        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, _| {
            b.iter(|| {
                check_block_pattern(
                    black_box("test_scale"),
                    black_box(&values),
                    black_box("P001"),
                    black_box(&mut quality_flags.clone()),
                    black_box(&mut quality_issues.clone()),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_diagonal_pattern,
    bench_alternating_pattern,
    bench_block_pattern
);
criterion_main!(benches);
