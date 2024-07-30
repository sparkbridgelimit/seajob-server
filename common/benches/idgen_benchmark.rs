use criterion::{criterion_group, criterion_main, Criterion};

use seajob_common::id_gen::id_generator::GLOBAL_IDGEN;

fn benchmark_next_id(c: &mut Criterion) {
    let mut idgen = GLOBAL_IDGEN.lock().unwrap();
    c.bench_function("next_id", |b| {
        b.iter(|| {
            idgen.next_id().unwrap();
        })
    });
}

criterion_group!(benches, benchmark_next_id);
criterion_main!(benches);
