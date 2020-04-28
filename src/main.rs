extern crate openssl;
extern crate diesel;

use chronicler::*;
use chronicler::models::{Post};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tide::StatusCode;
use tide::security::{CorsMiddleware, Origin};
use tide::http::headers::HeaderValue;

// type GenericError = Box<dyn std::error::Error + Send + Sync>;
type GenericError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, GenericError>;

#[derive(Serialize, Deserialize, Debug)]
struct InvalidFormatError {
  title: String,
  detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
  errors: Vec<InvalidFormatError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostById {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostByIdWithPublished {
    id: i32,
    published: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tide::new();
    let addr = format!("{host}:{port}", port=get_server_port(), host=get_server_host());

    let cors_middleware = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
        
    app.at("/").get(|_| async move { Ok("Hello, world!") });
    app.at("/create-post").post(|mut req: tide::Request<()>| async move {
        match req.body_json().await {
            Ok(json) => {
                let json_value: Post = json;
                create_post(&json_value.title, &json_value.body);
                let post = json!({
                    "data": {
                        "title": json_value.title,
                        "body": json_value.body,
                    }
                });
                Ok(tide::Response::new(StatusCode::Ok).body_json(&post)?)
            },
            Err(e) => {
                let post = json!(ErrorResponse {
                    errors: vec![
                        InvalidFormatError {
                            title: "Invalid Create".to_string(),
                            detail: e.to_string(),
                        }
                    ]
                });
                Ok(tide::Response::new(StatusCode::BadRequest).body_json(&post)?)
            }
        }
    });
    app.at("/posts").get(|_req| async move {
        let res = json!({
            "data": show_posts()
        });
        Ok(tide::Response::new(StatusCode::Ok).body_json(&res)?)
    });
    app.at("/delete-post").post(|mut req: tide::Request<()>| async move {
        match req.body_json().await {
            Ok(json) => {
                let json_value: PostById = json;
                let data = delete_post(json_value.id);
                // TODO: Add more error handling to specify reason for delete failure
                let post = match data {
                    Some(datum) => json!({
                        "data": format!("Deleted post with id: {:?}", datum)
                    }),
                    None => json!(ErrorResponse {
                        errors: vec![
                            InvalidFormatError {
                                title: "Invalid Delete".to_string(),
                                detail: format!("Failed to delete post with id: {:?}", json_value.id),
                            }
                        ]
                    }),
                };
                Ok(tide::Response::new(StatusCode::Ok).body_json(&post)?)
            },
            Err(e) => {
                let post = json!(ErrorResponse {
                    errors: vec![
                        InvalidFormatError {
                            title: "Invalid Delete".to_string(),
                            detail: e.to_string(),
                        }
                    ]
                });
                Ok(tide::Response::new(StatusCode::BadRequest).body_json(&post)?)
            }
        }
    });
    app.at("/toggle-post").post(|mut req: tide::Request<()>| async move {
        match req.body_json().await {
            Ok(json) => {
                let json_value: PostByIdWithPublished = json;
                let post = json!({
                    "data": format!("Toggled post with id: {:?}", toggle_post(json_value.id, json_value.published).unwrap())
                });
                Ok(tide::Response::new(StatusCode::Ok).body_json(&post)?)
            },
            
            Err(e) => {
                let post = json!(ErrorResponse {
                    errors: vec![
                        InvalidFormatError {
                            title: "Invalid Toggle".to_string(),
                            detail: e.to_string(),
                        }
                    ]
                });
                Ok(tide::Response::new(StatusCode::BadRequest).body_json(&post)?)
            }
        }
    });
    println!("Listening on http://{}", addr);
    app.middleware(cors_middleware);
    app.listen(addr).await?;

    Ok(())
}
