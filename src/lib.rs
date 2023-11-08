use serde::{Deserialize, Serialize};

pub mod admin;
pub mod auth;
pub mod fs;
pub mod public;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<Data> {
    code: isize,
    message: String,
    data: Option<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullResponse;

#[cfg(test)]
mod tests {
    const SERVER: &str = "http://127.0.0.1:5244";
    const USERNAME: &str = "admin";
    const PASSWORD: &str = "123456";

    use super::*;

    #[tokio::test]
    async fn test_get_user_info() {
        let token = auth::login(SERVER, USERNAME, PASSWORD).await.unwrap();
        println!("{token}");
        match auth::get_user_info(SERVER, &token).await {
            Ok(user) => {
                println!("{:?}", user);
                assert!(true);
            }
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_mkdir() {
        let token = auth::login(SERVER, USERNAME, PASSWORD).await.unwrap();
        match fs::mkdir(SERVER, &token, "/cloud/test_mkdir").await {
            Ok(()) => assert!(true),
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_rename() {
        let token = auth::login(SERVER, USERNAME, PASSWORD).await.unwrap();
        match fs::rename(SERVER, &token, "/cloud/test_mkdir", "test_rename").await {
            Ok(()) => assert!(true),
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_upload() {
        let token = auth::login(SERVER, USERNAME, PASSWORD).await.unwrap();
        match fs::upload(
            SERVER,
            &token,
            fs::UploadParams {
                local_file: ".gitignore".to_string(),
                remote_path: "/cloud/test_rename".to_string(),
                remote_name: "gitignore.txt".to_string(),
            },
        )
        .await
        {
            Ok(()) => assert!(true),
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_listdir() {
        let token = auth::login(SERVER, USERNAME, PASSWORD).await.unwrap();
        let mut params = fs::FileParams::default();
        params.path = Some("/cloud/test_rename".to_string());
        match fs::listdir(SERVER, &token, params).await {
            Ok(n) => {
                println!("{:?}", n);
                assert_eq!(n.total, 1);
            }
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    }
}
