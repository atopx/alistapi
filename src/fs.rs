use super::{NullResponse, Response};
use reqwest::Body;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// 新建文件夹 POST /api/fs/mkdir
pub async fn mkdir(server: &str, token: &str, path: &str) -> Result<(), String> {
    let url = format!("{}/api/fs/mkdir", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
        .post(url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&json!({"path": path}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(())
}

/// 重命名文件 POST /api/fs/rename
pub async fn rename(server: &str, token: &str, path: &str, name: &str) -> Result<(), String> {
    let url = format!("{}/api/fs/rename", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
        .post(url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&json!({"path":path,"name":name}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(())
}

/// 表单上传文件 PUT /api/fs/form
pub async fn form_upload(server: &str, token: &str, file: &str, size: u64) -> Result<(), String> {
    let url = format!("{}/api/fs/rename", server);
    let file = match File::open(file).await {
        Ok(file) => file,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);
    let resp: Response<NullResponse> = reqwest::Client::new()
        .put(url)
        .header("Authorization", token)
        .header("Content-Type", "multipart/form-data")
        .header("Content-Length", size)
        .body(file_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListdirData {
    pub content: Vec<FileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u128,
    pub is_dir: bool,
    pub modified: String,
    pub sign: String,
    pub thumb: String,
    pub r#type: isize,
    pub raw_url: Option<String>,
    pub readme: Option<String>,
    pub provider: Option<String>,
    pub related: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileParams {
    pub path: Option<String>,
    pub password: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub refresh: Option<bool>,
}

/// 列出文件目录 POST /api/fs/list
pub async fn listdir(server: &str, token: &str, params: FileParams) -> Result<ListdirData, String> {
    let url = format!("{}/api/fs/list", server);
    let resp: Response<ListdirData> = reqwest::Client::new()
        .post(url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(resp.data.unwrap())
}

/// 获取某个文件/目录信息 POST /api/fs/get
pub async fn fileinfo(server: &str, token: &str, params: FileParams) -> Result<FileInfo, String> {
    let url = format!("{}/api/fs/get", server);
    let resp: Response<FileInfo> = reqwest::Client::new()
        .post(url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    if resp.code != 200 {
        return Err(resp.message);
    }
    Ok(resp.data.unwrap())
}

/// 搜索文件或文件夹 POST /api/fs/search
pub async fn search() -> Result<(), String> {
    Ok(())
}

/// 获取目录 POST /api/fs/dirs
pub async fn get_dirs() -> Result<(), String> {
    Ok(())
}

/// 批量重命名 POST /api/fs/batch_rename
pub async fn batch_rename() -> Result<(), String> {
    Ok(())
}

/// 正则重命名 POST /api/fs/regex_rename
pub async fn regex_rename() -> Result<(), String> {
    Ok(())
}
/// 移动文件 POST /api/fs/move
pub async fn move_file() -> Result<(), String> {
    Ok(())
}
/// 聚合移动 POST /api/fs/recursive_move
pub async fn recursive_move() -> Result<(), String> {
    Ok(())
}
/// 复制文件 POST /api/fs/copy
pub async fn copy_file() -> Result<(), String> {
    Ok(())
}
/// 删除文件或文件夹 POST /api/fs/remove
pub async fn remove_directory() -> Result<(), String> {
    Ok(())
}
/// 删除空文件夹 POST /api/fs/remove_empty_directory
pub async fn remove_empty_directory() -> Result<(), String> {
    Ok(())
}
/// 流式上传文件 PUT /api/fs/put
pub async fn put_upload() -> Result<(), String> {
    Ok(())
}
/// 添加aria2下载 POST /api/fs/add_aria2
pub async fn add_aria2_task() -> Result<(), String> {
    Ok(())
}
/// 添加qBittorrent下载 POST /api/fs/add_qbit
pub async fn add_qbit_task() -> Result<(), String> {
    Ok(())
}
