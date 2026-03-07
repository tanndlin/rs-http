use std::{collections::HashMap, sync::Arc};

use crate::{
    request::{Method, Request},
    response::{Response, ResponseBuilder, StatusCode},
    types::ContentType,
};

pub fn u32_from_3_bytes(buf: [u8; 3]) -> u32 {
    u32::from(buf[0]) << 16 | u32::from(buf[1]) << 8 | u32::from(buf[2])
}

pub fn handle_request(
    request: &Request,
    cache: &Arc<HashMap<String, Vec<u8>>>,
) -> Result<Response, String> {
    match request.method {
        Method::GET => handle_get(request, cache),
        Method::HEAD => handle_head(request, cache),
        _ => Ok(Response::method_not_allowed(request.stream_id)),
    }
}

fn handle_get(
    request: &Request,
    cache: &Arc<HashMap<String, Vec<u8>>>,
) -> Result<Response, String> {
    let file_extension = request
        .path
        .split('.')
        .next_back()
        .ok_or("No file extension found")?;
    let content_type = ContentType::from_extension(file_extension);
    if content_type == ContentType::Unknown {
        return Ok(Response::bad_request(request.stream_id));
    }

    let slice = if let Some(range) = request.headers.get("Content-Range") {
        let range = range
            .strip_prefix("bytes=")
            .ok_or("Invalid Content-Range header")?;
        let mut split = range.split('-');
        let start = split
            .next()
            .ok_or("Invalid Content-Range header")?
            .parse::<usize>()
            .map_err(|_| "Invalid Content-Range header")?;
        let end = split
            .next()
            .ok_or("Invalid Content-Range header")?
            .parse::<usize>()
            .map_err(|_| "Invalid Content-Range header")?;
        Some((start, end))
    } else {
        None
    };

    match cache.get(&request.path) {
        Some(bytes) => {
            let bytes = if let Some((start, end)) = slice {
                if start >= bytes.len() || end >= bytes.len() || start > end {
                    return Ok(Response::bad_request(request.stream_id));
                }
                bytes[start..=end].to_vec()
            } else {
                bytes.clone()
            };

            Ok(ResponseBuilder::new()
                .status_code(StatusCode::Ok)
                .header("Content-Type".to_string(), content_type.into())
                .stream_id(request.stream_id)
                .body(bytes.clone())
                .build())
        }
        None => Ok(Response::not_found(request.stream_id)),
    }
}

fn handle_head(
    request: &Request,
    cache: &Arc<HashMap<String, Vec<u8>>>,
) -> Result<Response, String> {
    let file_extension = request
        .path
        .split('.')
        .next_back()
        .ok_or("No file extension found")?;
    let content_type = ContentType::from_extension(file_extension);
    if content_type == ContentType::Unknown {
        return Ok(Response::bad_request(request.stream_id));
    }

    let slice = if let Some(range) = request.headers.get("Content-Range") {
        let range = range
            .strip_prefix("bytes=")
            .ok_or("Invalid Content-Range header")?;
        let mut split = range.split('-');
        let start = split
            .next()
            .ok_or("Invalid Content-Range header")?
            .parse::<usize>()
            .map_err(|_| "Invalid Content-Range header")?;
        let end = split
            .next()
            .ok_or("Invalid Content-Range header")?
            .parse::<usize>()
            .map_err(|_| "Invalid Content-Range header")?;
        Some((start, end))
    } else {
        None
    };

    match cache.get(&request.path) {
        Some(bytes) => {
            let bytes = if let Some((start, end)) = slice {
                if start >= bytes.len() || end >= bytes.len() || start > end {
                    return Ok(Response::bad_request(request.stream_id));
                }
                bytes[start..=end].to_vec()
            } else {
                bytes.clone()
            };

            Ok(ResponseBuilder::new()
                .status_code(StatusCode::Ok)
                .header("Content-Type".to_string(), content_type.into())
                .header("Content-Length".to_string(), bytes.len().to_string())
                .stream_id(request.stream_id)
                .build())
        }
        None => Ok(Response::not_found(request.stream_id)),
    }
}
