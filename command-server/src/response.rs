// resp.rs
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    io::Cursor,
};

use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::Errors;

#[derive(Debug, Clone)]
pub struct Response<S: AsyncRead + Unpin> {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub data: S,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Status {
    NotFound,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::NotFound => write!(f, "404 Not Found"),
        }
    }
}

impl<S: AsyncRead + Unpin> Response<S> {
    pub fn status_and_headers(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");

        format!("HTTP/1.1 {}\r\n{headers}\r\n\r\n", self.status)
    }

    pub async fn write<O: AsyncWrite + Unpin>(mut self, stream: &mut O) -> Result<(), Errors> {
        stream
            .write_all(self.status_and_headers().as_bytes())
            .await
            .map_err(|_| Errors::CannotWriteToStream)?;

        tokio::io::copy(&mut self.data, stream)
            .await
            .map_err(|_| Errors::CopyError)?;

        Ok(())
    }
}
