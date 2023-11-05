use serde::{Deserialize, Serialize};

use super::{NullResponse, Response};

/// ping检测 GET /ping
pub async fn ping(server: &str) -> Result<(), String> {
    let url = format!("{}/ping", server);
    let resp: Response<NullResponse> = reqwest::Client::new()
        .get(url)
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
pub struct Settings {
    pub allow_indexed: String,
    pub allow_mounted: String,
    pub announcement: String,
    pub audio_autoplay: String,
    pub audio_cover: String,
    pub auto_update_index: String,
    pub default_page_size: String,
    pub external_previews: String,
    pub favicon: String,
    pub filename_char_mapping: String,
    pub forward_direct_link_params: String,
    pub hide_files: String,
    pub home_container: String,
    pub home_icon: String,
    pub iframe_previews: String,
    pub logo: String,
    pub main_color: String,
    pub ocr_api: String,
    pub package_download: String,
    pub pagination_type: String,
    pub robots_txt: String,
    pub search_index: String,
    pub settings_layout: String,
    pub site_title: String,
    pub sso_login_enabled: String,
    pub sso_login_platform: String,
    pub version: String,
    pub video_autoplay: String,
}

/// 获取站点设置 GET /api/public/settings
pub async fn get_settings(server: &str) -> Result<Settings, String> {
    let url = format!("{}/api/public/settings", server);
    let resp: Response<Settings> = reqwest::Client::new()
        .get(url)
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
