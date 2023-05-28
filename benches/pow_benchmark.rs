use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use word_of_wisdom_pow::common::pow::PowSolver;

fn benchmarking(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let diffs = vec![1, 2, 3];

    for d in diffs {
        let bench_name = format!("find_nonce with difficulty {}", d);
        c.bench_function(bench_name.as_str(), |b| b.iter(|| {
            let mut challenge = [0u8; 8];
            rng.fill(&mut challenge);
            let solver = PowSolver::new(challenge.to_vec(), d);
            solver.find_nonce().expect("Can't find solution")
        }));
    }
}

criterion_group!(benches, benchmarking);
criterion_main!(benches);
