use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use labello::{Config, Encoder, EncoderType};


fn labello_bench(c: &mut Criterion) {
    let enctype = EncoderType::Ordinal;
    // create encoder of <enctype>
    let mut enc: Encoder<String> = Encoder::new(Some(enctype));
    let num_categories = vec![2, 3, 4, 5, 1000];

    // // Benchmark training time 10 times for each training sample size
    let mut group = c.benchmark_group("labello");
    group.sample_size(10);

    for ncat in num_categories.iter() {
            // Load data
            let data: Vec<String> = vec!["hello".to_string(),
                                        "world".to_string(),
                                        "world1".to_string(),
                                        "world2".to_string(),
                                        "world3".to_string(),
                                        "again".to_string(),
                                        "hello".to_string(),
                                        "world4".to_string(),
                                        "world5".to_string(),
                                        "world6".to_string(),
                                        "world7".to_string(),
                                        "again".to_string(),
                                        "hello".to_string(),
                                        "world8".to_string(),
                                        "world9".to_string(),
                                        "world10".to_string(),
                                        "world11".to_string(),
                                        "again".to_string(),
                                        "hello".to_string(),
                                        "world12".to_string(),
                                        "world13".to_string(),
                                        "world14".to_string(),
                                        "world15".to_string(),
                                        "again16".to_string(),
                                        "hello".to_string(),
                                        "world".to_string(),
                                        "world".to_string(),
                                        "world".to_string(),
                                        "world".to_string(),
                                        "again".to_string(),
                                        "hello".to_string(),
                                        "world".to_string(),
                                        "world".to_string(),
                                        "world".to_string(),
                                        "world".to_string(),
                                        "again".to_string(),
                                        "hello".to_string(),
                                        "again".to_string(),
                                        "goodbye".to_string(),
                                        ];

            let config = Config{
                max_nclasses: Some(3),
                mapping_function: None
            };

            // transform original data to internal encoded representation
            // let trans_data = enc.transform(&data);

            group.bench_with_input(
                    BenchmarkId::from_parameter(ncat),
                    &data,
                    |b, d| b.iter(|| enc.fit(&d, &config)));
    }

    group.finish();
}

criterion_group!(benches, labello_bench);
criterion_main!(benches);
