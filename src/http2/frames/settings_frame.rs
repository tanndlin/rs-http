use crate::http2::{
    error::{HTTP2Error, HTTP2ErrorCode},
    frames::frame::{FrameHeader, FrameType},
};

use crate::encode_to::EncodeTo;

#[derive(Debug, Default)]
pub struct SettingsFrameFlags {
    pub ack: bool,
}

impl From<u8> for SettingsFrameFlags {
    fn from(bits: u8) -> Self {
        Self {
            ack: bits & 1 > 0, // bit 0
        }
    }
}

impl From<SettingsFrameFlags> for u8 {
    fn from(flags: SettingsFrameFlags) -> Self {
        u8::from(flags.ack)
    }
}

#[repr(u16)]
pub enum SettingsIdentifier {
    HeaderTableSize = 1,
    EnablePush = 2,
    MaxConcurrentStreams = 3,
    InitialWindowSize = 4,
    MaxFrameSize = 5,
    MaxHeaderListSize = 6,
}

impl TryFrom<u16> for SettingsIdentifier {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::HeaderTableSize),
            2 => Ok(Self::EnablePush),
            3 => Ok(Self::MaxConcurrentStreams),
            4 => Ok(Self::InitialWindowSize),
            5 => Ok(Self::MaxFrameSize),
            6 => Ok(Self::MaxHeaderListSize),
            _ => Err(format!("Invalid settings identifier: {value}")),
        }
    }
}

#[derive(Debug, Default)]
pub struct SettingsFrame {
    pub header: FrameHeader<SettingsFrameFlags>,
    header_table_size: Option<u32>,
    enable_push: Option<bool>,
    max_concurrent_streams: Option<u32>,
    initial_window_size: Option<u32>,
    max_frame_size: Option<u32>,
    max_header_list_size: Option<u32>,
}

impl SettingsFrame {
    pub fn new_ack(stream_identifier: u32) -> Self {
        Self {
            header: FrameHeader {
                length: 0,
                frame_type: FrameType::Settings,
                flags: SettingsFrameFlags { ack: true },
                stream_id: stream_identifier,
            },
            ..Default::default()
        }
    }
}

impl TryFrom<&[u8]> for SettingsFrame {
    type Error = HTTP2Error;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let header = FrameHeader::try_from(buf)
            .map_err(|_| HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError))?; // TODO: This is not a protocol error in the spec; but we'll say that for now
        let length = header.length as usize;
        let buf = &buf[9..];
        assert!(
            buf.len() >= length,
            "Settings frame length does not match the length in the frame header. {} out of {length} bytes left",
            buf.len()
        );

        if !length.is_multiple_of(6) {
            return Err(HTTP2Error::Connection(HTTP2ErrorCode::FrameSizeError));
        }

        let mut ret = Self {
            header,
            ..Default::default()
        };
        let mut offset = 0;
        while offset < length {
            let ident = u16::from_be_bytes(buf[offset..offset + 2].try_into().unwrap());
            if let Ok(ident) = SettingsIdentifier::try_from(ident) {
                let value = u32::from_be_bytes(buf[offset + 2..offset + 6].try_into().unwrap());
                match ident {
                    SettingsIdentifier::HeaderTableSize => ret.header_table_size = Some(value),
                    SettingsIdentifier::EnablePush => {
                        if !(value == 0 || value == 1) {
                            return Err(HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError));
                        }
                        ret.enable_push = Some(value > 0);
                    }
                    SettingsIdentifier::MaxConcurrentStreams => {
                        ret.max_concurrent_streams = Some(value);
                    }
                    SettingsIdentifier::InitialWindowSize => {
                        if value > 2u32.pow(31) - 1 {
                            return Err(HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError));
                        }
                        ret.initial_window_size = Some(value);
                    }
                    SettingsIdentifier::MaxFrameSize => {
                        if value < 2u32.pow(14) || value > 2u32.pow(24) - 1 {
                            return Err(HTTP2Error::Connection(HTTP2ErrorCode::ProtocolError));
                        }
                        ret.max_frame_size = Some(value);
                    }
                    SettingsIdentifier::MaxHeaderListSize => ret.max_header_list_size = Some(value),
                }
            }

            offset += 6;
        }

        Ok(ret)
    }
}

impl EncodeTo for SettingsFrame {
    fn encode_to(self, buf: &mut Vec<u8>) {
        let mut header = self.header;
        header.length = 0;

        let mut bytes = vec![];

        if let Some(size) = self.header_table_size {
            header.length += 6;
            bytes.extend((SettingsIdentifier::HeaderTableSize as u16).to_be_bytes());
            bytes.extend(size.to_be_bytes());
        }
        if let Some(enable) = self.enable_push {
            header.length += 6;
            bytes.extend((SettingsIdentifier::EnablePush as u16).to_be_bytes());
            bytes.extend(u32::from(enable).to_be_bytes());
        }
        if let Some(max) = self.max_concurrent_streams {
            header.length += 6;
            bytes.extend((SettingsIdentifier::MaxConcurrentStreams as u16).to_be_bytes());
            bytes.extend(max.to_be_bytes());
        }
        if let Some(size) = self.initial_window_size {
            header.length += 6;
            bytes.extend((SettingsIdentifier::InitialWindowSize as u16).to_be_bytes());
            bytes.extend(size.to_be_bytes());
        }
        if let Some(size) = self.max_frame_size {
            header.length += 6;
            bytes.extend((SettingsIdentifier::MaxFrameSize as u16).to_be_bytes());
            bytes.extend(size.to_be_bytes());
        }
        if let Some(size) = self.max_header_list_size {
            header.length += 6;
            bytes.extend((SettingsIdentifier::MaxHeaderListSize as u16).to_be_bytes());
            bytes.extend(size.to_be_bytes());
        }

        header.encode_to(buf);
        buf.extend(bytes);
    }
}

pub struct SettingsFrameBuilder {
    header: Option<FrameHeader<SettingsFrameFlags>>,
    header_table_size: Option<u32>,
    enable_push: Option<bool>,
    max_concurrent_streams: Option<u32>,
    initial_window_size: Option<u32>,
    max_frame_size: Option<u32>,
    max_header_list_size: Option<u32>,
}

impl SettingsFrameBuilder {
    pub fn new() -> Self {
        SettingsFrameBuilder {
            header: None,
            header_table_size: None,
            enable_push: None,
            max_concurrent_streams: None,
            initial_window_size: None,
            max_frame_size: None,
            max_header_list_size: None,
        }
    }

    pub fn header_table_size(mut self, size: u32) -> Self {
        self.header_table_size = Some(size);
        self
    }

    pub fn enable_push(mut self, enable: bool) -> Self {
        self.enable_push = Some(enable);
        self
    }

    pub fn max_concurrent_streams(mut self, max: u32) -> Self {
        self.max_concurrent_streams = Some(max);
        self
    }

    pub fn initial_window_size(mut self, size: u32) -> Self {
        self.initial_window_size = Some(size);
        self
    }

    pub fn max_frame_size(mut self, size: u32) -> Self {
        self.max_frame_size = Some(size);
        self
    }

    pub fn max_header_list_size(mut self, size: u32) -> Self {
        self.max_header_list_size = Some(size);
        self
    }

    pub fn build(self) -> SettingsFrame {
        let length = self.calc_length();
        let SettingsFrameBuilder {
            header,
            header_table_size,
            enable_push,
            max_concurrent_streams,
            initial_window_size,
            max_frame_size,
            max_header_list_size,
        } = self;

        let header = match header {
            Some(mut h) => {
                h.length = length;
                h
            }
            None => FrameHeader::<SettingsFrameFlags> {
                length,
                frame_type: FrameType::Settings,
                stream_id: 0,
                flags: SettingsFrameFlags { ack: false },
            },
        };

        SettingsFrame {
            header,
            header_table_size,
            enable_push,
            max_concurrent_streams,
            initial_window_size,
            max_frame_size,
            max_header_list_size,
        }
    }

    fn calc_length(&self) -> u32 {
        let mut specified_parameters = 0u32;

        if self.header_table_size.is_some() {
            specified_parameters += 1;
        }
        if self.enable_push.is_some() {
            specified_parameters += 1;
        }
        if self.max_concurrent_streams.is_some() {
            specified_parameters += 1;
        }
        if self.initial_window_size.is_some() {
            specified_parameters += 1;
        }
        if self.max_frame_size.is_some() {
            specified_parameters += 1;
        }
        if self.max_header_list_size.is_some() {
            specified_parameters += 1;
        }

        specified_parameters * 6
    }
}
