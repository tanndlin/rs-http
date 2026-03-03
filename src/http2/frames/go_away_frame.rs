use crate::http2::{
    error::HTTP2ErrorCode,
    frames::frame::{FrameHeader, FrameType},
};

#[derive(Debug)]
pub struct GoAwayFrame {
    pub header: FrameHeader<u8>,
    last_stream_id: u32, // 31 bits
    error_code: u32,
    data: Vec<u8>,
}

impl TryFrom<&[u8]> for GoAwayFrame {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let header = FrameHeader::try_from(buf)?;
        let n = u32::from_be_bytes(buf[9..13].try_into().map_err(|_| "Not enough bytes")?);
        let last_stream_id = n & !(1 << 31);
        let error_code =
            u32::from_be_bytes(buf[13..17].try_into().map_err(|_| "Not enough bytes")?);

        let data = buf[17..17 + header.length as usize].to_vec();

        Ok(Self {
            header,
            last_stream_id,
            error_code,
            data,
        })
    }
}

impl From<HTTP2ErrorCode> for GoAwayFrame {
    fn from(e: HTTP2ErrorCode) -> Self {
        GoAwayFrame::new(e as u32)
    }
}

impl GoAwayFrame {
    pub fn new(error_code: u32) -> Self {
        Self {
            header: FrameHeader {
                length: 8,
                frame_type: FrameType::GoAway,
                flags: 0,
                stream_id: 0,
            },
            last_stream_id: 0, // TODO
            error_code,
            data: vec![], // TODO
        }
    }
}

impl From<GoAwayFrame> for Vec<u8> {
    fn from(frame: GoAwayFrame) -> Self {
        let mut ret = vec![];
        let header_bytes: Vec<u8> = frame.header.into();
        ret.extend_from_slice(&header_bytes);
        ret.extend_from_slice(&frame.last_stream_id.to_be_bytes());
        ret.extend_from_slice(&frame.error_code.to_be_bytes());
        ret.extend_from_slice(&frame.data);
        ret
    }
}
