pub enum HTTP2Error {
    Connection(HTTP2ErrorCode),
    Stream(StreamError),
}

#[repr(u32)]
pub enum HTTP2ErrorCode {
    ProtocolError = 1,
}

pub struct StreamError {
    pub stream_id: u32,
    pub error_code: HTTP2ErrorCode,
}
