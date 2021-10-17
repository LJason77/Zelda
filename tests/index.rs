use rocket::{http::Status, local::asynchronous::Client};

#[rocket::async_test]
async fn health_check() {
    let rocket = zelda::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    let response = client.get("/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
