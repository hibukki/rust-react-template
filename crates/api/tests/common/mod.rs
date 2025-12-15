use api::{config::Config, routes, state::AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use tower::ServiceExt;

pub struct TestApp {
    app: Router,
    cookies: Option<String>,
}

impl TestApp {
    pub async fn new() -> Self {
        let config = Config {
            port: 0,
            host: "127.0.0.1".to_string(),
            database_url: "sqlite::memory:".to_string(),
        };

        let pool = db::pool::create_pool(&config.database_url)
            .await
            .expect("Failed to create test pool");

        db::pool::run_migrations(&pool)
            .await
            .expect("Failed to run migrations");

        let state = AppState::new(config, pool);
        let app = routes::router(state);

        Self { app, cookies: None }
    }

    pub async fn get(&self, uri: &str) -> TestResponse {
        let mut req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        if let Some(ref cookies) = self.cookies {
            req.headers_mut().insert("Cookie", cookies.parse().unwrap());
        }

        let response = self.app.clone().oneshot(req).await.unwrap();
        TestResponse::from_response(response).await
    }

    pub async fn post(&mut self, uri: &str, body: serde_json::Value) -> TestResponse {
        let mut req = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();

        if let Some(ref cookies) = self.cookies {
            req.headers_mut().insert("Cookie", cookies.parse().unwrap());
        }

        let response = self.app.clone().oneshot(req).await.unwrap();
        let test_response = TestResponse::from_response(response).await;

        // Save cookies if set
        if let Some(ref cookie) = test_response.set_cookie {
            self.cookies = Some(cookie.clone());
        }

        test_response
    }

    pub async fn patch(&self, uri: &str, body: serde_json::Value) -> TestResponse {
        let mut req = Request::builder()
            .method("PATCH")
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();

        if let Some(ref cookies) = self.cookies {
            req.headers_mut().insert("Cookie", cookies.parse().unwrap());
        }

        let response = self.app.clone().oneshot(req).await.unwrap();
        TestResponse::from_response(response).await
    }
}

pub struct TestResponse {
    pub status: StatusCode,
    pub body: String,
    pub set_cookie: Option<String>,
}

impl TestResponse {
    async fn from_response(response: axum::response::Response) -> Self {
        let status = response.status();
        let set_cookie = response
            .headers()
            .get("Set-Cookie")
            .map(|v| v.to_str().unwrap().to_string());

        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        Self {
            status,
            body,
            set_cookie,
        }
    }

    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.body).unwrap_or_else(|_| {
            panic!("Failed to parse JSON: {}", self.body);
        })
    }

    pub fn assert_ok(&self) {
        assert!(
            self.status.is_success(),
            "Expected success status, got {}: {}",
            self.status,
            self.body
        );
    }

    pub fn assert_status(&self, expected: StatusCode) {
        assert_eq!(
            self.status, expected,
            "Expected {}, got {}: {}",
            expected, self.status, self.body
        );
    }
}
