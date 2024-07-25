use axum_extra::headers::authorization::Bearer;

pub fn check_auth(bearer: &Bearer) -> Option<ApiKey> {
    // TODO: implement api key retrieval
    let valid_api_keys = vec!["sk-1234"];

    if valid_api_keys.contains(&bearer.token()) {
        Some(ApiKey {
            key: bearer.token().to_string(),
        })
    } else {
        None
    }
}

pub fn auth_request(bearer: &Bearer) -> Result<ApiKey, ()> {
    if let Some(api_key) = check_auth(bearer) {
        Ok(api_key)
    } else {
        Err(())
    }
}

#[derive(Clone, Debug)]
pub struct ApiKey {
    pub key: String,
}