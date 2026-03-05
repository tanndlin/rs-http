use crate::{
    encode_to::EncodeTo,
    http2::{error::HTTP2Error, frames::frame::FrameHeader},
};

#[derive(Debug)]
pub struct ContinuationFrameFlags {
    pub end_headers: bool, // bit 2
}

impl From<u8> for ContinuationFrameFlags {
    fn from(value: u8) -> Self {
        let end_headers = (value & 4) > 0;

        Self { end_headers }
    }
}

impl From<ContinuationFrameFlags> for u8 {
    fn from(flags: ContinuationFrameFlags) -> Self {
        let mut bits = 0u8;
        bits |= u8::from(flags.end_headers) << 2;
        bits
    }
}

#[derive(Debug)]
pub struct ContinuationFrame {
    pub header: FrameHeader<ContinuationFrameFlags>,
    pub header_block_fragment: Vec<u8>,
}

impl TryFrom<&[u8]> for ContinuationFrame {
    type Error = HTTP2Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let header = FrameHeader::<ContinuationFrameFlags>::try_from(buf)?;
        let header_block_fragment = buf[9..9 + header.length as usize].to_vec();

        Ok(Self {
            header,
            header_block_fragment,
        })
    }
}

impl EncodeTo for ContinuationFrame {
    fn encode_to(self, buf: &mut Vec<u8>) {
        self.header.encode_to(buf);
        buf.extend(self.header_block_fragment);
    }
}
