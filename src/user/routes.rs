use crate::user::{User, UserRequest};

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use sqlx::postgres::PgPool;
// use std::env;
// use std::process::exit;

// use openidconnect::core::{
//     CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClient, CoreClientAuthMethod, CoreGrantType,
//     CoreIdToken, CoreIdTokenVerifier, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse,
//     CoreJweContentEncryptionAlgorithm, CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm,
//     CoreResponseMode, CoreResponseType, CoreSubjectIdentifierType,
// };
// use openidconnect::reqwest::async_http_client;
// use openidconnect::{
//     AdditionalProviderMetadata, ClientId, ClientSecret, IssuerUrl, ProviderMetadata, RedirectUrl,
// };

// #[derive(Clone, Debug, Deserialize, Serialize)]
// struct RevocationEndpointProviderMetadata {
//     revocation_endpoint: String,
// }
// impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}
// type GoogleProviderMetadata = ProviderMetadata<
//     RevocationEndpointProviderMetadata,
//     CoreAuthDisplay,
//     CoreClientAuthMethod,
//     CoreClaimName,
//     CoreClaimType,
//     CoreGrantType,
//     CoreJweContentEncryptionAlgorithm,
//     CoreJweKeyManagementAlgorithm,
//     CoreJwsSigningAlgorithm,
//     CoreJsonWebKeyType,
//     CoreJsonWebKeyUse,
//     CoreJsonWebKey,
//     CoreResponseMode,
//     CoreResponseType,
//     CoreSubjectIdentifierType,
// >;

// fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
//     let mut err_msg = format!("ERROR: {}", msg);
//     let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
//     while let Some(cause) = cur_fail {
//         err_msg += &format!("\n    caused by: {}", cause);
//         cur_fail = cause.source();
//     }
//     println!("{}", err_msg);
// }

#[get("/users/")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::BadRequest().body("Error trying to read all users from database"),
    }
}

// Get user by ID endpoint
//
// For testing purposes, temporarily requires the request to contain the
// corresponding user's session token for the endpoint to return the user.
// Otherwise, a bad request error is thrown
#[get("/user/{id}/")]
async fn find(req: HttpRequest, id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let auth_token = req.head().headers().get("authorization");
    match auth_token {
        Some(value) => {
            let val = str::replace(value.to_str().ok().unwrap_or(""), "Bearer ", "");
            let result = User::find_by_id(id.into_inner(), db_pool.get_ref()).await;
            match result {
                Ok(user) => {
                    if user.session_token == val {
                        HttpResponse::Ok().json(user)
                    } else {
                        HttpResponse::BadRequest().body("Auth token invalid")
                    }
                }
                _ => HttpResponse::BadRequest().body("User not found"),
            }
        }
        _ => HttpResponse::BadRequest().body("Not authorized"),
    }
}

#[post("/user/")]
async fn create(user: web::Json<UserRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to create new user"),
    }
}

#[put("/user/{id}/")]
async fn update(
    id: web::Path<i32>,
    user: web::Json<UserRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = User::update(id.into_inner(), user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

#[delete("/user/{id}/")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("User not found")
            }
        }
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

// Endpoint for authentication
//
// Currently creates a new user if a user doesn't exist, and returns the existing
// user if that user is found. Upon creating a user, generates a hashed session
// token (temporarily permanent)
#[post("/authenticate/")]
async fn authenticate(user: web::Json<UserRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    // Verify token id using OpenID Connect
    //
    // let google_client_id = ClientId::new(
    //     env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    // );
    // let google_client_secret = ClientSecret::new(
    //     env::var("GOOGLE_CLIENT_SECRET")
    //         .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    // );
    // let issuer_url =
    //     IssuerUrl::new("https://accounts.google.com".to_string()).expect("Invalid issuer URL");

    // let provider_metadata = GoogleProviderMetadata::discover_async(issuer_url, async_http_client)
    //     .await
    //     .unwrap_or_else(|err| {
    //         handle_error(&err, "Failed to discover OpenID Provider");
    //         unreachable!();
    //     });

    // // Set up the config for the Google OAuth2 process.
    // let client = CoreClient::from_provider_metadata(
    //     provider_metadata,
    //     google_client_id,
    //     None,
    // )
    // .set_redirect_uri(
    //     RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    // );

    // let id_token_verifier: CoreIdTokenVerifier = client.id_token_verifier();
    // let id_token =
    //     serde_json::from_str::<CoreIdToken>(&user.id_token).expect("failed to deserialize");

    // HttpResponse::Ok().body(format!("Google returned ID token: {:?}", id_token))

    let result = User::find_by_id(user.user_id, db_pool.get_ref()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => {
            let new_user = User::create(user.into_inner(), db_pool.get_ref()).await;
            match new_user {
                Ok(user) => HttpResponse::Ok().json(user),
                _ => HttpResponse::BadRequest().body("Error trying to create new user"),
            }
        }
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(authenticate);
}
