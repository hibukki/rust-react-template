mod common;

use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn health_check() {
    let app = common::TestApp::new().await;

    let response = app.get("/health").await;

    response.assert_ok();
    insta::assert_json_snapshot!(response.json());
}

#[tokio::test]
async fn register_and_login() {
    let mut app = common::TestApp::new().await;

    // Register
    let response = app
        .post(
            "/api/auth/register",
            json!({
                "email": "test@example.com",
                "password": "password123",
                "display_name": "Test User"
            }),
        )
        .await;

    response.assert_ok();
    let body = response.json();
    assert_eq!(body["email"], "test@example.com");
    assert_eq!(body["profile"]["display_name"], "Test User");

    // Login with same credentials (need new app instance as cookies are per-session)
    let mut app2 = common::TestApp::new().await;

    // First register
    app2.post(
        "/api/auth/register",
        json!({
            "email": "test2@example.com",
            "password": "password123",
            "display_name": "Test User 2"
        }),
    )
    .await;

    // Then login
    let response = app2
        .post(
            "/api/auth/login",
            json!({
                "email": "test2@example.com",
                "password": "password123"
            }),
        )
        .await;

    response.assert_ok();
    let body = response.json();
    assert_eq!(body["email"], "test2@example.com");
}

#[tokio::test]
async fn register_duplicate_email() {
    let mut app = common::TestApp::new().await;

    // First registration
    app.post(
        "/api/auth/register",
        json!({
            "email": "dupe@example.com",
            "password": "password123",
            "display_name": "First User"
        }),
    )
    .await
    .assert_ok();

    // Second registration with same email
    let response = app
        .post(
            "/api/auth/register",
            json!({
                "email": "dupe@example.com",
                "password": "password456",
                "display_name": "Second User"
            }),
        )
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);
    let body = response.json();
    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("already registered"));
}

#[tokio::test]
async fn login_wrong_password() {
    let mut app = common::TestApp::new().await;

    // Register
    app.post(
        "/api/auth/register",
        json!({
            "email": "wrong@example.com",
            "password": "correct_password",
            "display_name": "User"
        }),
    )
    .await
    .assert_ok();

    // Login with wrong password
    let response = app
        .post(
            "/api/auth/login",
            json!({
                "email": "wrong@example.com",
                "password": "wrong_password"
            }),
        )
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

// REST list_profiles tests removed - profiles now read via WebSocket

#[tokio::test]
async fn update_own_profile() {
    let mut app = common::TestApp::new().await;

    // Register
    let response = app
        .post(
            "/api/auth/register",
            json!({
                "email": "update@example.com",
                "password": "password123",
                "display_name": "Original Name"
            }),
        )
        .await;

    response.assert_ok();
    let body = response.json();
    let profile_id = body["profile"]["id"].as_i64().unwrap();

    // Update profile (cookies were automatically saved)
    let response = app
        .patch(
            &format!("/api/profiles/{}", profile_id),
            json!({
                "bio": "My new bio"
            }),
        )
        .await;

    response.assert_ok();
    let body = response.json();
    assert_eq!(body["bio"], "My new bio");
}

#[tokio::test]
async fn update_profile_unauthorized_no_session() {
    let app = common::TestApp::new().await;

    // Try to update without logging in
    let response = app
        .patch(
            "/api/profiles/1",
            json!({
                "bio": "Hacked!"
            }),
        )
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}
