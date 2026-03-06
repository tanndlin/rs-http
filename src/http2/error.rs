#[derive(Debug)]
pub enum HTTP2Error {
    Connection(HTTP2ErrorCode),
    Stream(StreamError),
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u32)]
pub enum HTTP2ErrorCode {
    NoError = 0,
    ProtocolError = 1,
    InternalError = 2,
    FlowControlError = 3,
    SettingsTimeout = 4,
    StreamClosed = 5,
    FrameSizeError = 6,
    RefusedStream = 7,
    Cancel = 8,
    CompressionError = 9,
    ConnectError = 10,
    EnhanceYourCalm = 11,
    InadequateSecurity = 12,
    HTTP11Required = 13,
}

impl TryFrom<u32> for HTTP2ErrorCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HTTP2ErrorCode::NoError),
            1 => Ok(HTTP2ErrorCode::ProtocolError),
            2 => Ok(HTTP2ErrorCode::InternalError),
            3 => Ok(HTTP2ErrorCode::FlowControlError),
            4 => Ok(HTTP2ErrorCode::SettingsTimeout),
            5 => Ok(HTTP2ErrorCode::StreamClosed),
            6 => Ok(HTTP2ErrorCode::FrameSizeError),
            7 => Ok(HTTP2ErrorCode::RefusedStream),
            8 => Ok(HTTP2ErrorCode::Cancel),
            9 => Ok(HTTP2ErrorCode::CompressionError),
            10 => Ok(HTTP2ErrorCode::ConnectError),
            11 => Ok(HTTP2ErrorCode::EnhanceYourCalm),
            12 => Ok(HTTP2ErrorCode::InadequateSecurity),
            13 => Ok(HTTP2ErrorCode::HTTP11Required),
            _ => Err(()),
        }
    }
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
