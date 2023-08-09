use reqwest::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::SERVER_ADDRESS;

use derive_more::{Display, Error};

static API_VERSION: &str = "v1";
static API_MESAGES: &str = "messages";
static API_ALL_MESSAGES: &str = "all";
static API_ALL_MESSAGES_WITH_STATUS: &str = "filter";
static API_ACK_MESSAGES: &str = "ack";
static API_ACK_RECEPTION: &str = "reception";
static USER_NAME: &str = "admin";
static USER_PASSWORD: &str = "admin";

#[derive(Debug, Display, Error, Serialize)]
pub enum ProcessingMessageError {
    #[display(fmt = "Error while downloading message")]
    DownloadingMessageError,
    #[display(fmt = "Error while sending ack for message reception")]
    AckMessageReceptionError,
    #[display(fmt = "Cannot deserialize object")]
    DeserializingObjectError,
    #[display(fmt = "You must be authorized to send request")]
    UnauthorizedRequest,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDTO {
    pub id: String,
    pub sender: String,
    pub title: String,
    pub message: String,
    pub status: String,
}

async fn send_request(
    api: &str,
    default_error: ProcessingMessageError,
) -> Result<Response, ProcessingMessageError> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", SERVER_ADDRESS, api);

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
            _ => return Err(default_error),
        }
    }

    Ok(resposne)
}

pub async fn ack_message_reception(message_id: &str) -> Result<(), ProcessingMessageError> {
    let api = format!("/{}/{}/{}", API_ACK_MESSAGES, API_ACK_RECEPTION, message_id);

    let _ = send_request(
        api.as_str(),
        ProcessingMessageError::AckMessageReceptionError,
    )
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn download_all_messages() -> Result<Vec<MessageDTO>, ProcessingMessageError> {
    let api = format!("/{}/{}", API_MESAGES, API_ALL_MESSAGES);

    let response = send_request(
        api.as_str(),
        ProcessingMessageError::DownloadingMessageError,
    )
    .await?;

    let body = response
        .text()
        .await
        .map_err(|_| ProcessingMessageError::DownloadingMessageError)?;

    let messages: Vec<MessageDTO> = serde_json::from_str(body.as_str())
        .map_err(|_| ProcessingMessageError::DeserializingObjectError)?;

    Ok(messages)
}

pub async fn download_all_new_messages() -> Result<Vec<MessageDTO>, ProcessingMessageError> {
    let api = format!("/{}/{}/New", API_MESAGES, API_ALL_MESSAGES_WITH_STATUS);

    let resposne = send_request(
        api.as_str(),
        ProcessingMessageError::DownloadingMessageError,
    )
    .await?;

    let body = resposne
        .text()
        .await
        .map_err(|_| ProcessingMessageError::DownloadingMessageError)?;

    let messages: Vec<MessageDTO> = serde_json::from_str(body.as_str())
        .map_err(|_| ProcessingMessageError::DeserializingObjectError)?;

    Ok(messages)
}
