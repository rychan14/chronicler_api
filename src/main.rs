use chronicler::*;
use chronicler::models::{NewPost,Post};
use serde::{Serialize, Deserialize};
use serde_json::json;

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
    let addr = format!("0.0.0.0:{port}", port=get_server_port());

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
                tide::Response::new(200).body_json(&post).unwrap()
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
                tide::Response::new(400).body_json(&post).unwrap()
            }
        }
    });
    app.at("/posts").get(|_req| async move {
        let res = json!({
            "data": show_posts()
        });
        tide::Response::new(200).body_json(&res).unwrap()
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
                tide::Response::new(200).body_json(&post).unwrap()
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
                tide::Response::new(400).body_json(&post).unwrap()
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
                tide::Response::new(200).body_json(&post).unwrap()
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
                tide::Response::new(400).body_json(&post).unwrap()
            }
        }
    });
    println!("Listening on http://{}", addr);
    app.listen(addr).await?;

    Ok(())
}
