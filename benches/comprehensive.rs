use std::{hint::black_box, path::Path, process::Command};

use criterion::{Criterion, criterion_group, criterion_main};

fn bench_rustowl_check(c: &mut Criterion) {
    let binary_path = "./target/release/rustowl";

    assert!(
        Path::new(binary_path).exists(),
        "Binary not found at {binary_path}. Run 'cargo build --release --bin rustowl' first."
    );

    let test_fixture = "./benches/perf-tests";

    c.bench_function("rustowl_check", |b| {
        b.iter(|| {
            let output = Command::new(binary_path)
                .args(["check", test_fixture, "--all-targets", "--all-features"])
                .output()
                .expect("Failed to run rustowl check");
            black_box(output.status.success());
        });
    });
}

criterion_group!(benches, bench_rustowl_check);
criterion_main!(benches);
