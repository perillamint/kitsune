use bytes::Bytes;
use core::convert::Infallible;
use http_body_util::Full;
use hyper::{Request, Response};
use kitsune_http_client::{Body, Client};
use sonic_rs::{JsonValueTrait, Value};
use tower::service_fn;

#[tokio::test]
async fn json_request() {
    let client = service_fn(|req: Request<_>| async move {
        assert_eq!(req.headers()["Accept"], "application/activity+json");
        Ok::<_, Infallible>(Response::new(Full::new(Bytes::from(
            r#"{"preferredUsername":"0x0"}"#,
        ))))
    });

    let client = Client::builder()
        .default_header("Accept", "application/activity+json")
        .unwrap()
        .service(client);

    let req = Request::builder()
        .uri("https://corteximplant.com/users/0x0")
        .body(Body::empty())
        .unwrap();

    let response = client.execute(req).await.unwrap();
    assert!(response.status().is_success());

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["preferredUsername"].as_str(), Some("0x0"));
}
