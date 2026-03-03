use crate::http2::{
    error::{HTTP2Error, HTTP2ErrorCode},
    frames::frame::Frame,
};

pub struct HTTP2StreamClosed {
    pub identifier: u32,
}

impl HTTP2StreamClosed {
    pub fn handle_frame(&self, frame: Frame) -> Result<Vec<u8>, HTTP2Error> {
        match frame {
            _ => Err(HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError)),
        }
    }
}
