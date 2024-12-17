use axum::http::StatusCode;

#[tokio::test]
async fn test_ping() {
    let router = monocle_cli::test::create_test_app();
    let server = axum_test::TestServer::new(router).expect("error setting up server");
    let request = server.get("/ping");
    let response = request.await;
    response.assert_status(StatusCode::OK);
}
