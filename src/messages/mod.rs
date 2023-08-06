use serde::Deserialize;

use crate::SERVER_ADDRESS;

use derive_more::{Display, Error};

static API_VERSION: &str = "v1";
static API_MESAGES: &str = "messages";
static API_ALL_MESSAGES: &str = "all";
static API_ALL_MESSAGES_WITH_STATUS: &str = "filter";
static USER_NAME: &str = "admin";
static USER_PASSWORD: &str = "admin";

#[derive(Debug, Display, Error)]
pub enum ProcessingMessageError {
    #[display(fmt = "Error while downloading message")]
    DownloadingMessageError,
    #[display(fmt = "Cannot deserialize object")]
    DeserializingObjectError,
    #[display(fmt = "You must be authorized to send request")]
    UnauthorizedRequest,
}

#[derive(Debug, Deserialize)]
pub struct MessageDTO {
    pub sender: String,
    pub title: String,
    pub message: String,
    pub status: String,
}

pub async fn download_all_messages() -> Result<Vec<MessageDTO>, ProcessingMessageError> {
    let url = format!("{}/{}/{}", SERVER_ADDRESS, API_MESAGES, API_ALL_MESSAGES);
    let client = reqwest::Client::new();

    let resposne = client
        .get(url)
        .basic_auth(USER_NAME, Some(USER_PASSWORD))
        .send()
        .await
        .map_err(|_| ProcessingMessageError::DownloadingMessageError)?;

    let response_code = resposne.status();
    if !response_code.is_success() {
        match response_code {
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(ProcessingMessageError::UnauthorizedRequest)
            }
            _ => return Err(ProcessingMessageError::DownloadingMessageError),
        }
    }

    let body = resposne
        .text()
        .await
        .map_err(|_| ProcessingMessageError::DownloadingMessageError)?;

    let messages: Vec<MessageDTO> = serde_json::from_str(body.as_str())
        .map_err(|_| ProcessingMessageError::DeserializingObjectError)?;

    Ok(messages)
}

pub async fn download_all_new_messages() -> Result<Vec<MessageDTO>, ProcessingMessageError> {
    let url = format!(
        "{}/{}/{}/New",
        SERVER_ADDRESS, API_MESAGES, API_ALL_MESSAGES_WITH_STATUS
    );
    let client = reqwest::Client::new();

    let resposne = client
        .get(url)
        .basic_auth(USER_NAME, Some(USER_PASSWORD))
        .send()
        .await
        .map_err(|_| ProcessingMessageError::DownloadingMessageError)?;

    let response_code = resposne.status();
    if !response_code.is_success() {
        match response_code {
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(ProcessingMessageError::UnauthorizedRequest)
            }
            _ => return Err(ProcessingMessageError::DownloadingMessageError),
        }
    }

    let body = resposne
        .text()
        .await
        .map_err(|_| ProcessingMessageError::DownloadingMessageError)?;

    let messages: Vec<MessageDTO> = serde_json::from_str(body.as_str())
        .map_err(|_| ProcessingMessageError::DeserializingObjectError)?;

    Ok(messages)
}
