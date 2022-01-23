use rocket::http::{ContentType, Status};

#[test]
fn test_hello() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(zoo::rocket()).unwrap();
    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query": "{ hello }" }"#)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let body: serde_json::Value =
        serde_json::from_slice(resp.into_string().unwrap().as_bytes()).unwrap();
    let hello_value = &body["data"]["hello"];

    assert_eq!(hello_value, "Hello, world!");
}
