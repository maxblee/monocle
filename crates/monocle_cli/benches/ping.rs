use axum_test::TestServer;
use monocle_cli::test::create_test_app;

use criterion::{criterion_group, criterion_main, Criterion};

fn ping_benchmark(c: &mut Criterion) {
    let router = create_test_app();
    let server = TestServer::new(router).expect("error starting server");
    let tokio_runtime = tokio::runtime::Runtime::new().expect("error creating tokio runtime");

    let server = std::sync::Arc::new(server);
    let server_clone = server.clone();

    c.bench_function("ping_endpoint", |b| {
        b.to_async(&tokio_runtime).iter(|| {
            let server = server_clone.clone();
            async move {
                let req = server.get("/ping");
                req.await;
            }
        })
    });
}

criterion_group!(benches, ping_benchmark);
criterion_main!(benches);
