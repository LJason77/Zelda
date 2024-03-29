use rocket::{http::Status, local::asynchronous::Client};

#[rocket::async_test]
async fn not_found() {
    let rocket = zelda::rocket().await;
    let client = Client::untracked(rocket).await.unwrap();

    let response = client.get("/404").dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
}
