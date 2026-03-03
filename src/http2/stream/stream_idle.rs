use crate::http2::{
    error::{HTTP2Error, HTTP2ErrorCode},
    frames::frame::Frame,
};

pub struct HTTP2StreamIdle {
    pub identifier: u32,
}

impl HTTP2StreamIdle {
    pub fn handle_frame(&self, frame: Frame) -> Result<Vec<u8>, HTTP2Error> {
        match frame {
            Frame::Headers(headers_frame) => todo!(),
            Frame::Priority(priority_frame) => todo!(),
            _ => Err(HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError)),
        }
    }
}
