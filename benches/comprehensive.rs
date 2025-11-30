use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::Path;
use std::process::Command;

fn bench_rustowl_check(c: &mut Criterion) {
    let binary_path = "./target/release/rustowl";

    if !Path::new(binary_path).exists() {
        panic!("Binary not found at {}. Run 'cargo build --release --bin rustowl' first.", binary_path);
    }

    let test_fixture = "./benches/perf-tests";

    c.bench_function("rustowl_check", |b| {
        b.iter(|| {
            let output = Command::new(binary_path)
                .args(["check", test_fixture, "--all-targets", "--all-features"])
                .output()
                .expect("Failed to run rustowl check");
            black_box(output.status.success());
        })
    });
}

criterion_group!(benches, bench_rustowl_check);
criterion_main!(benches);
