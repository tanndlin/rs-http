use std::fmt;
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

impl fmt::Display for PsuedoHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Method => ":method",
            Self::Scheme => ":scheme",
            Self::Authority => ":authority",
            Self::Path => ":path",
            Self::Status => ":status",
        };

        f.write_str(value)
    }
}
