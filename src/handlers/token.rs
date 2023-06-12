use axum::{Form, Json};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenUrl,
};

pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_url: String,
}

pub async fn get_token(
) -> Json<oauth2::StandardTokenResponse<EmptyExtraTokenFields, oauth2::basic::BasicTokenType>> {
    // let req = body.i
    let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://localhost:8080/default/authorize".to_string()).expect("abcd"),
        Some(
            TokenUrl::new("http://localhost:8080/default/token".to_string()).expect("Handle error"),
        ),
    )
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string()).expect("redirect error"));
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .expect("RequestTokenError");
    Json(token_result)
    // println!("{:?}", token_result);
}


