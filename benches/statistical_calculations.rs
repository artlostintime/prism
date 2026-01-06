// benches/statistical_calculations.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use prism::stats::{calculate_cronbachs_alpha, Stats};

fn generate_data(n_participants: usize, n_items: usize) -> Vec<Vec<f64>> {
    (0..n_participants)
        .map(|i| (0..n_items).map(|j| ((i + j) % 7) as f64 + 1.0).collect())
        .collect()
}

fn bench_descriptive_stats(c: &mut Criterion) {
    let mut group = c.benchmark_group("descriptive_stats");

    for n in [10, 100, 1000, 10000].iter() {
        let values: Vec<f64> = (0..*n).map(|i| (i % 7) as f64 + 1.0).collect();

        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, _| {
            b.iter(|| Stats::calculate(black_box(&values)));
        });
    }

    group.finish();
}

fn bench_cronbachs_alpha(c: &mut Criterion) {
    let mut group = c.benchmark_group("cronbachs_alpha");

    // Vary number of participants
    for n_participants in [10, 50, 100, 500, 1000].iter() {
        let data = generate_data(*n_participants, 10);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}p_10i", n_participants)),
            n_participants,
            |b, _| {
                b.iter(|| calculate_cronbachs_alpha(black_box(&data)));
            },
        );
    }

    // Vary number of items
    for n_items in [5, 10, 20, 50].iter() {
        let data = generate_data(100, *n_items);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("100p_{}i", n_items)),
            n_items,
            |b, _| {
                b.iter(|| calculate_cronbachs_alpha(black_box(&data)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_descriptive_stats, bench_cronbachs_alpha);
criterion_main!(benches);
