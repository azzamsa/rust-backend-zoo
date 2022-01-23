use rocket::http::{ContentType, Status};

#[test]
fn test_health() {
    use rocket::local::blocking::Client;
    use zoo::health;

    let client = Client::tracked(zoo::rocket()).unwrap();
    let resp = client
        .post("/graphql")
        .header(ContentType::JSON)
        .body(r#"{"query": "{ health { status } }" }"#)
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    let body: serde_json::Value = serde_json::from_slice(&resp.into_bytes().unwrap()).unwrap();
    let health_value = &body["data"]["health"];
    let health: health::schema::Health = serde_json::from_value(health_value.clone()).unwrap();

    assert_eq!(health.status, "running");
}
