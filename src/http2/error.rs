#[derive(Debug)]
pub enum HTTP2Error {
    Connection(HTTP2ErrorCode),
    Stream(StreamError),
}

#[derive(Debug)]
#[repr(u32)]
pub enum HTTP2ErrorCode {
    ProtocolError = 1,
    StreamClosed = 5,
    FrameSizeError = 6,
}

#[derive(Debug)]
pub struct StreamError {
    pub stream_id: u32,
    pub error_code: HTTP2ErrorCode,
}

impl StreamError {
    pub fn new(stream_id: u32, error_code: HTTP2ErrorCode) -> Self {
        Self {
            stream_id,
            error_code,
        }
    }
}
