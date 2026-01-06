// benches/scale_computation.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use prism::config::{ScaleDefinition, SurveyConfig, SurveySettings};
use prism::processor::process_scale;
use std::collections::HashMap;

fn create_test_config() -> SurveyConfig {
    let mut config = SurveyConfig::default();
    config.survey = SurveySettings {
        name: "Benchmark Test".to_string(),
        min_score: 1,
        max_score: 7,
        participant_id_column: None,
    };
    config
}

fn create_scale_def(n_items: usize, reverse_count: usize) -> ScaleDefinition {
    let items: Vec<String> = (1..=n_items).map(|i| format!("q{}", i)).collect();
    let reverse_scored: Vec<String> = items.iter().take(reverse_count).cloned().collect();

    ScaleDefinition {
        items,
        reverse_scored: if reverse_count > 0 {
            Some(reverse_scored)
        } else {
            None
        },
    }
}

fn create_record(n_items: usize) -> csv::StringRecord {
    let mut data = vec!["P001".to_string()];
    for i in 1..=n_items {
        data.push(((i % 7) + 1).to_string());
    }
    csv::StringRecord::from(data)
}

fn create_header_map(n_items: usize) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    map.insert("id".to_string(), 0);
    for i in 1..=n_items {
        map.insert(format!("q{}", i), i);
    }
    map
}

fn bench_scale_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("scale_computation");

    for n_items in [5, 10, 20, 50].iter() {
        let config = create_test_config();
        let scale_def = create_scale_def(*n_items, 0);
        let record = create_record(*n_items);
        let header_map = create_header_map(*n_items);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_items", n_items)),
            n_items,
            |b, _| {
                b.iter(|| {
                    process_scale(
                        black_box(&scale_def),
                        black_box(&record),
                        black_box(&header_map),
                        black_box(&config),
                    )
                });
            },
        );
    }

    group.finish();
}

fn bench_reverse_scoring(c: &mut Criterion) {
    let mut group = c.benchmark_group("reverse_scoring");

    let config = create_test_config();
    let n_items = 20;
    let record = create_record(n_items);
    let header_map = create_header_map(n_items);

    for reverse_count in [0, 5, 10, 15, 20].iter() {
        let scale_def = create_scale_def(n_items, *reverse_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_reversed", reverse_count)),
            reverse_count,
            |b, _| {
                b.iter(|| {
                    process_scale(
                        black_box(&scale_def),
                        black_box(&record),
                        black_box(&header_map),
                        black_box(&config),
                    )
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_scale_computation, bench_reverse_scoring);
criterion_main!(benches);
