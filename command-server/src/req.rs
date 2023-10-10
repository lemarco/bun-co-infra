use std::{collections::HashMap, hash::Hash};

use tokio::io::{AsyncBufRead, AsyncBufReadExt};

use crate::Errors;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
}
impl TryFrom<&str> for Method {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            _ => Err(Errors::UnsupportedHttpMethod),
        }
    }
}

type ParsedQuery = (String, Option<HashMap<String, String>>);
fn parse_query(path: &str) -> Result<ParsedQuery, Errors> {
    let mut splitted_path = path.split('?');
    let path = splitted_path.next().ok_or(Errors::MissingPath)?.to_string();

    match splitted_path.next() {
        None => Ok((path, None)),
        Some(qs) => {
            let query_map: HashMap<String, String> = qs
                .split('&')
                .filter_map(|querypair| {
                    let mut qs = querypair.split('=');
                    let k = qs.next();
                    let v = qs.next();
                    if k.is_some() || v.is_some() {
                        Some((k.unwrap().into(), v.unwrap().into()))
                    } else {
                        None
                    }
                })
                .collect();
            if query_map.len() > 0 {
                Ok((path, Some(query_map)))
            } else {
                Ok((path, None))
            }
        }
    }
}

pub async fn parse_request(mut stream: impl AsyncBufRead + Unpin) -> Result<Request, Errors> {
    let mut line_buffer = String::new();
    stream
        .read_line(&mut line_buffer)
        .await
        .map_err(|_| Errors::CannotReadLineWhileParsingRequest)?;

    let mut parts = line_buffer.split_whitespace();

    let method: Method = parts
        .next()
        .ok_or(Errors::MissingMethod)
        .and_then(TryInto::try_into)?;

    let p = parts
        .next()
        .ok_or(Errors::MissingPath)
        .map(parse_query)?
        .unwrap();

    let mut headers = HashMap::new();

    loop {
        line_buffer.clear();
        stream
            .read_line(&mut line_buffer)
            .await
            .map_err(|_| Errors::CannotReadLineWhileParsingRequest)?;

        if line_buffer.is_empty() || line_buffer == "\n" || line_buffer == "\r\n" {
            break;
        }

        let mut comps = line_buffer.split(":");
        let key = comps.next().ok_or(Errors::MissingHeaderName)?;
        let value = comps.next().ok_or(Errors::MissingHeaderValue)?.trim();

        headers.insert(key.to_string(), value.to_string());
    }

    Ok(Request {
        method,
        path: p.0,
        headers,
    })
}
