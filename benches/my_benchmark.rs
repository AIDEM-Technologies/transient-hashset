use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("perfect-struct");
    let charset = "1234567890qwertyuioplkjhgfdsazxcvbnm";
    let different_key_values = (0..1_000_000).map(|_| (random_string::generate(150, charset), random_string::generate(200, charset))).collect_vec();
    let different_values = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), random_string::generate(200, charset))).collect_vec();
    let same_everything = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), "4c67d2af-ca5d-4c64-937b-85b944544e37".to_string())).collect_vec();
    group.sample_size(10);
    group.bench_function("different-key-different-values", |b| {
        b.iter(|| check_impressions(different_key_values.clone()))
    });
    group.bench_function("same-key-different-values", |b| {
        b.iter(|| check_impressions(different_values.clone()))
    });
    group.bench_function("same-key-same-value", |b| {
        b.iter(|| check_impressions(same_everything.clone()))
    });
    group.finish();

    let mut group = c.benchmark_group("naive-struct");
    let charset = "1234567890qwertyuioplkjhgfdsazxcvbnm";
    let different_key_values = (0..1_000_000).map(|_| (random_string::generate(150, charset), random_string::generate(200, charset))).collect_vec();
    let different_values = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), random_string::generate(200, charset))).collect_vec();
    let same_everything = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), "4c67d2af-ca5d-4c64-937b-85b944544e37".to_string())).collect_vec();
    group.sample_size(10);
    group.bench_function("different-key-different-values", |b| {
        b.iter(|| naive_check_impressions(different_key_values.clone()))
    });
    group.bench_function("same-key-different-values", |b| {
        b.iter(|| naive_check_impressions(different_values.clone()))
    });
    group.bench_function("same-key-same-value", |b| {
        b.iter(|| naive_check_impressions(same_everything.clone()))
    });
    group.finish();


    let mut group = c.benchmark_group("evmap-struct");
    let charset = "1234567890qwertyuioplkjhgfdsazxcvbnm";
    let different_key_values = (0..1_000_000).map(|_| (random_string::generate(150, charset), random_string::generate(200, charset))).collect_vec();
    let different_values = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), random_string::generate(200, charset))).collect_vec();
    let same_everything = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), "4c67d2af-ca5d-4c64-937b-85b944544e37".to_string())).collect_vec();
    group.sample_size(10);
    group.bench_function("different-key-different-values", |b| {
        b.iter(|| evmap_check_impressions(different_key_values.clone()))
    });
    group.bench_function("same-key-different-values", |b| {
        b.iter(|| evmap_check_impressions(different_values.clone()))
    });
    group.bench_function("same-key-same-value", |b| {
        b.iter(|| evmap_check_impressions(same_everything.clone()))
    });
    group.finish();

    let mut group = c.benchmark_group("flurry-struct");
    let charset = "1234567890qwertyuioplkjhgfdsazxcvbnm";
    let different_key_values = (0..1_000_000).map(|_| (random_string::generate(150, charset), random_string::generate(200, charset))).collect_vec();
    let different_values = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), random_string::generate(200, charset))).collect_vec();
    let same_everything = (0..1_000_000).map(|_| ("http://example.com/?arithmetic=battle#attraction".to_string(), "4c67d2af-ca5d-4c64-937b-85b944544e37".to_string())).collect_vec();
    group.sample_size(10);
    group.bench_function("different-key-different-values", |b| {
        b.iter(|| flurry_check_impressions(different_key_values.clone()))
    });
    group.bench_function("same-key-different-values", |b| {
        b.iter(|| flurry_check_impressions(different_values.clone()))
    });
    group.bench_function("same-key-same-value", |b| {
        b.iter(|| flurry_check_impressions(same_everything.clone()))
    });
    group.finish();
}

async fn check_impressions(items: Vec<(String, String)>) {
    let perfect = transient_hashset::Perfect::<String, String>::new(tokio::time::Duration::from_secs(30));
    let mut futures = Vec::new();
    for (key, value) in items.into_iter() {
        futures.push(perfect.contains(key, value));
    }
    let _ = futures::future::join_all(futures).await;
}

async fn naive_check_impressions(items: Vec<(String, String)>) {
    let naive = transient_hashset::Naive::<String, String>::new(tokio::time::Duration::from_secs(30));
    let mut futures = Vec::new();
    for (key, value) in items.into_iter() {
        futures.push(naive.contains(key, value));
    }
    let _ = futures::future::join_all(futures).await;
}

async fn evmap_check_impressions(items: Vec<(String, String)>) {
    let evmap = transient_hashset::TransientHashSet::<String, String>::new(tokio::time::Duration::from_secs(30));
    let mut futures = Vec::new();
    for (key, value) in items.into_iter() {
        futures.push(evmap.contains(key, value));
    }
    let _ = futures::future::join_all(futures).await;
}

async fn flurry_check_impressions(items: Vec<(String, String)>) {
    let flurry = transient_hashset::Flurry::new(tokio::time::Duration::from_secs(30));
    for (key, value) in items.into_iter() {
        let _ = flurry.contains(key, value);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
