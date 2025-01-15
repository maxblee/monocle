use axum_test::TestServer;
use monocle_cli::test::create_test_app;

use divan::Bencher;
use tokio::runtime::Runtime;

fn main() {
    divan::main();
}

#[divan::bench]
fn ping_benchmark(bencher: Bencher) {
    let router = create_test_app();
    let server = TestServer::new(router).expect("error starting server");
    let tokio_runtime = tokio::runtime::Runtime::new().expect("error creating tokio runtime");

    let server = std::sync::Arc::new(server);
    let server_clone = server.clone();
    let rt = Runtime::new().unwrap();

    bencher.bench_local(move || {
        rt.block_on(async {
            let server = server_clone.clone();
            let req = server.get("/ping");
            req.await;
        })
    })
}
