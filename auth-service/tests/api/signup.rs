use crate::helpers::{get_random_email, TestApp};


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    // TODO: add more malformed input test cases
    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true,
            // "email": random_email,
        }),  
        serde_json::json!({
            "password": "qwerty",
            "requires2FA": false,
            // "email": random_email,
        }),
        serde_json::json!({
            "password": "foobarbaz",
            "requires2FA": true,
            // "email": random_email,
        }),
        serde_json::json!({
            "password": 13123123,
            "requires2FA": true,
            // "email": random_email,
        }),
        serde_json::json!({
            "password": false,
            "requires2FA": true,
            // "email": random_email,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
