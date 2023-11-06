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
    pub content: Vec<DirFileInfo>,
    // 总数
    pub total: usize,
    // 说明
    pub readme: String,
    // 是否可写入
    pub write: bool,
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirFileInfo {
    pub name: String,
    pub size: u128,
    pub is_dir: bool,
    pub modified: String,
    pub sign: String,
    pub thumb: String,
    pub r#type: isize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileParams {
    // 路径
    pub path: Option<String>,
    // 密码
    pub password: Option<String>,
    // 页数
    pub page: Option<usize>,
    // 每页数目
    pub per_page: Option<usize>,
    // 是否强制刷新
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u128,
    pub is_dir: bool,
    pub modified: String,
    pub sign: String,
    pub thumb: String,
    pub r#type: isize,
    pub row_url: String,
    pub readme: String,
    pub provider: String,
    pub related: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchParams {
    // 搜索目录
    pub parent: String,
    // 关键词
    pub keywords: String,
    // scope 0-全部 1-文件夹 2-文件
    pub scope: Option<u8>,
    // 页数
    pub page: Option<usize>,
    // 每页数目
    pub per_page: Option<usize>,
    // 密码
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFileData {
    pub content: Vec<SearchFileInfo>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFileInfo {
    pub name: String,
    pub parent: String,
    pub size: u128,
    pub is_dir: bool,
    pub r#type: isize,
}

/// 搜索文件或文件夹 POST /api/fs/search
pub async fn search(
    server: &str,
    token: &str,
    params: SearchParams,
) -> Result<SearchFileData, String> {
    let url = format!("{}/api/fs/search", server);
    let resp: Response<SearchFileData> = reqwest::Client::new()
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetDirParams {
    // 搜索目录
    pub parent: String,
    // 页数
    pub page: Option<usize>,
    // 每页数目
    pub per_page: Option<usize>,
    // 密码
    pub password: Option<String>,
    pub force_root: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchDirData {
    pub content: Vec<SearchDirInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchDirInfo {
    pub name: String,
    pub modified: String,
}

/// 获取目录 POST /api/fs/dirs
pub async fn get_dirs(
    server: &str,
    token: &str,
    params: GetDirParams,
) -> Result<SearchDirData, String> {
    let url = format!("{}/api/fs/dirs", server);
    let resp: Response<SearchDirData> = reqwest::Client::new()
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameParams {
    pub src_name: String,
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRenameParams {
    pub src_dir: String,
    pub rename_objects: Vec<RenameParams>,
}

/// 批量重命名 POST /api/fs/batch_rename
pub async fn batch_rename(
    server: &str,
    token: &str,
    params: BatchRenameParams,
) -> Result<(), String> {
    let url = format!("{}/api/fs/batch_rename", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegexRenameParams {
    pub src_name_regex: String,
    pub new_name_regex: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRegexRenameParams {
    pub src_dir: String,
    pub rename_objects: Vec<RegexRenameParams>,
}

/// 正则重命名 POST /api/fs/regex_rename
pub async fn regex_rename(
    server: &str,
    token: &str,
    params: BatchRegexRenameParams,
) -> Result<(), String> {
    let url = format!("{}/api/fs/regex_rename", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveParams {
    pub src_dir: String,
    pub dst_dir: String,
    pub names: Vec<String>,
}

/// 移动文件 POST /api/fs/move
pub async fn move_file(server: &str, token: &str, params: MoveParams) -> Result<(), String> {
    let url = format!("{}/api/fs/move", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecursiveMoveParams {
    pub src_dir: String,
    pub dst_dir: String,
}

/// 聚合移动 POST /api/fs/recursive_move
pub async fn recursive_move(
    server: &str,
    token: &str,
    params: RecursiveMoveParams,
) -> Result<(), String> {
    let url = format!("{}/api/fs/recursive_move", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyParams {
    pub src_dir: String,
    pub dst_dir: String,
    pub names: Vec<String>,
}

/// 复制文件 POST /api/fs/copy
pub async fn copy_file(server: &str, token: &str, params: CopyParams) -> Result<(), String> {
    let url = format!("{}/api/fs/copy", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteParams {
    pub dir: String,
    pub names: Vec<String>,
}

/// 删除文件或文件夹 POST /api/fs/remove
pub async fn remove_directory(
    server: &str,
    token: &str,
    params: DeleteParams,
) -> Result<(), String> {
    let url = format!("{}/api/fs/remove", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

/// 删除空文件夹 POST /api/fs/remove_empty_directory
pub async fn remove_empty_directory(
    server: &str,
    token: &str,
    src_dir: String,
) -> Result<(), String> {
    let url = format!("{}/api/fs/remove_empty_directory", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
        .post(url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&json!({"src_dir": src_dir}))
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
/// 流式上传文件 PUT /api/fs/put
pub async fn put_upload() -> Result<(), String> {
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineTaskParams {
    pub path: String,
    pub urls: Vec<String>,
}

/// 添加aria2下载 POST /api/fs/add_aria2
pub async fn add_aria2_task(
    server: &str,
    token: &str,
    params: OfflineTaskParams,
) -> Result<(), String> {
    let url = format!("{}/api/fs/add_aria2", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}

/// 添加qBittorrent下载 POST /api/fs/add_qbit
pub async fn add_qbit_task(
    server: &str,
    token: &str,
    params: OfflineTaskParams,
) -> Result<(), String> {
    let url = format!("{}/api/fs/add_qbit", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
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
    Ok(())
}
