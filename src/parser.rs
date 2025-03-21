use crate::asn::{ASN, ASN16, ASN32};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ASNKind {
    ASN,
    ASN16,
    ASN32,
}
struct Parser<'a> {
    state: &'a [u8],
}
impl<'a> Parser<'a> {}
pub struct ASNParseError(ASNKind);
impl ASN {
    pub fn parse_ascii(b: &[u8]) -> Result<Self, ASNParseError> {
        if (b.starts_with("as".as_bytes())
            || b.starts_with("AS".as_bytes())
            || b.starts_with("As".as_bytes())
            || b.starts_with("aS".as_bytes()))
            && (b.len() < 13)
        {
            match std::str::from_utf8(&b[2..]) {
                Ok(o) => {
                    if let Ok(n) = o.parse::<u16>() {
                        Ok(ASN::AS_16(ASN16::new(n)))
                    } else if let Ok(n) = o.parse::<u32>() {
                        Ok(ASN::AS_32(ASN32::new(n)))
                    } else {
                        Err(ASNParseError(ASNKind::ASN))
                    }
                }
                Err(_) => Err(ASNParseError(ASNKind::ASN)),
            }
        } else if b.len() < 11 {
            match std::str::from_utf8(&b) {
                Ok(o) => {
                    if let Ok(n) = o.parse::<u16>() {
                        Ok(ASN::AS_16(ASN16::new(n)))
                    } else if let Ok(n) = o.parse::<u32>() {
                        Ok(ASN::AS_32(ASN32::new(n)))
                    } else {
                        Err(ASNParseError(ASNKind::ASN))
                    }
                }
                Err(_) => Err(ASNParseError(ASNKind::ASN)),
            }
        } else {
            Err(ASNParseError(ASNKind::ASN))
        }
    }
}
impl ASN16 {
    pub fn parse_ascii(b: &[u8]) -> Result<Self, ASNParseError> {
        if (b.starts_with("as".as_bytes())
            || b.starts_with("AS".as_bytes())
            || b.starts_with("As".as_bytes())
            || b.starts_with("aS".as_bytes()))
            && (b.len() < 7)
        {
            match std::str::from_utf8(&b[2..]) {
                Ok(o) => {
                    if let Ok(n) = o.parse::<u16>() {
                        Ok(ASN16::new(n))
                    } else {
                        Err(ASNParseError(ASNKind::ASN))
                    }
                }
                Err(_) => Err(ASNParseError(ASNKind::ASN)),
            }
        } else if b.len() < 5 {
            match std::str::from_utf8(&b) {
                Ok(o) => {
                    if let Ok(n) = o.parse::<u16>() {
                        Ok(ASN16::new(n))
                    } else {
                        Err(ASNParseError(ASNKind::ASN))
                    }
                }
                Err(_) => Err(ASNParseError(ASNKind::ASN)),
            }
        } else {
            Err(ASNParseError(ASNKind::ASN))
        }
    }
}
impl ASN32 {
    pub fn parse_ascii(b: &[u8]) -> Result<Self, ASNParseError> {
        if (b.starts_with("as".as_bytes())
            || b.starts_with("AS".as_bytes())
            || b.starts_with("As".as_bytes())
            || b.starts_with("aS".as_bytes()))
            && (b.len() < 13)
        {
            match std::str::from_utf8(&b[2..]) {
                Ok(o) => {
                    if let Ok(n) = o.parse::<u32>() {
                        Ok(ASN32::new(n))
                    } else {
                        Err(ASNParseError(ASNKind::ASN))
                    }
                }
                Err(_) => Err(ASNParseError(ASNKind::ASN)),
            }
        } else if b.len() < 11 {
            match std::str::from_utf8(&b) {
                Ok(o) => {
                    if let Ok(n) = o.parse::<u32>() {
                        Ok(ASN32::new(n))
                    } else {
                        Err(ASNParseError(ASNKind::ASN))
                    }
                }
                Err(_) => Err(ASNParseError(ASNKind::ASN)),
            }
        } else {
            Err(ASNParseError(ASNKind::ASN))
        }
    }
}
impl FromStr for ASN {
    type Err = ASNParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_ascii(s.as_bytes())
    }
}
impl From<ASN16> for ASN {
    fn from(value: ASN16) -> Self {
        ASN::AS_16(value)
    }
}
impl From<ASN32> for ASN {
    fn from(value: ASN32) -> Self {
        ASN::AS_32(value)
    }
}
impl From<u8> for ASN {
    fn from(value: u8) -> Self {
        ASN::AS_16(ASN16::new(value.into()))
    }
}
impl From<u16> for ASN {
    fn from(value: u16) -> Self {
        ASN::AS_32(ASN32::new(value.into()))
    }
}
