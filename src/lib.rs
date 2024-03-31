use uuid::Uuid;

pub const MOWOJANG_API_URI: &str = "https://mowojang.matdoes.dev/";

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct MowojangApiResponse {
    pub name: String,
    pub id: Uuid,
}

impl MowojangApiResponse {
    pub fn new<T: ToString, U: Into<Uuid>>(username: T, uuid: U) -> Self {
        Self {
            name: username.to_string(),
            id: uuid.into(),
        }
    }
}

pub fn valid_java_username(username: &str) -> bool {
    (3..=16).contains(&username.len())
        && username
            .chars()
            .all(|char| (char.is_ascii_alphanumeric() || char == '_'))
}

pub fn valid_geyser_username(username: &str) -> bool {
    let mut chars = username.chars();
    (3..=12).contains(&username.len().saturating_sub(1))
        && chars.next() == Some('.')
        && chars.next().expect("wha").is_ascii_alphabetic()
        && chars.all(|char| (char.is_ascii_alphanumeric() || char == '_'))
}

pub async fn check_username(username: &str) -> Option<MowojangApiResponse> {
    if !valid_java_username(username) {
        return None;
    };
    api_internal(MOWOJANG_API_URI, username).await
}

pub async fn check_uuid<T>(uuid: T) -> Option<MowojangApiResponse>
where
    Uuid: From<T>,
{
    let uuid = Uuid::from(uuid);
    api_internal(MOWOJANG_API_URI, uuid).await
}

async fn api_internal<T: ToString>(url: &str, query: T) -> Option<MowojangApiResponse> {
    match reqwest::get(format!("{url}{}", urlencoding::encode(&query.to_string()))).await {
        Ok(response) => match response.json().await {
            Ok(json) => json,
            Err(_) => None,
        },
        Err(_) => None,
    }
}
