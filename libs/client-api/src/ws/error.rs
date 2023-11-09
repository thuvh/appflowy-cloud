use reqwest::StatusCode;
use tokio_tungstenite::tungstenite::Error;

#[derive(Debug, thiserror::Error)]
pub enum WSError {
  #[error(transparent)]
  TungsteniteError(Error),

  #[error("{0}")]
  LostConnection(String),

  #[error("Auth error: {0}")]
  AuthError(String),

  #[error(transparent)]
  Internal(#[from] anyhow::Error),
}

impl From<Error> for WSError {
  fn from(value: Error) -> Self {
    match &value {
      Error::ConnectionClosed | Error::AlreadyClosed => WSError::LostConnection(value.to_string()),
      Error::Http(resp) => {
        let status = resp.status();
        if status == StatusCode::UNAUTHORIZED || status == StatusCode::NOT_FOUND {
          WSError::AuthError("Unauthorized websocket connection".to_string())
        } else {
          WSError::TungsteniteError(value)
        }
      },
      _ => WSError::TungsteniteError(value),
    }
  }
}
