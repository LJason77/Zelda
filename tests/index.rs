use rocket::{http::Status, local::asynchronous::Client};

/// 小贴士
#[rocket::async_test]
async fn tips() {
    let rocket = zelda::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    let response = client.get("/tips").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

/// 健康检查
#[rocket::async_test]
async fn health_check() {
    let rocket = zelda::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    let response = client.get("/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
