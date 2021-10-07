use rocket::local::blocking::Client;

#[test]
fn not_found() {
    let rocket = zelda::rocket();
    let client = Client::debug(rocket).unwrap();

    let response = client.get("/404").dispatch();
    assert_eq!(response.status().code, 404);
}
