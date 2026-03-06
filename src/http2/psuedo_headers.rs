use std::str::FromStr;

pub enum PsuedoHeader {
    Method,
    Scheme,
    Authority,
    Path,
    Status,
}

impl FromStr for PsuedoHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ":method" => Ok(Self::Method),
            ":scheme" => Ok(Self::Scheme),
            ":authority" => Ok(Self::Authority),
            ":path" => Ok(Self::Path),
            ":status" => Ok(Self::Status),
            _ => Err(()),
        }
    }
}

impl ToString for PsuedoHeader {
    fn to_string(&self) -> String {
        match self {
            Self::Method => ":method".to_string(),
            Self::Scheme => ":scheme".to_string(),
            Self::Authority => ":authority".to_string(),
            Self::Path => ":path".to_string(),
            Self::Status => ":status".to_string(),
        }
    }
}
