use std::{
    fmt::{Debug, Display, Pointer},
    str::FromStr,
};

/// An AS number, either an AS16 or AS32
///
/// This enum can contain either an [`ASN16`] or an [`ASN32`], see their
/// respective documentation for more details.
///
/// # Examples
///
/// ```
/// use asn::ASN;
/// let as_16_number = ASN::AS_16(ASN16::new());
/// let as_32_number = ASN::AS_32(ASN32::new());
///
/// assert_eq!("AS13345".parse(), Ok(as_16_number));
/// assert_eq!("AS78322".parse(), Ok(as_32_number));
///
/// assert_eq!(as_16_number.is_as16(), false);
/// assert_eq!(as_16_number.is_as32(), true);
/// ```
//#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub enum ASN {
    AS_16(ASN16),
    AS_32(ASN32),
}

impl Display for ASN {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASN::AS_16(asn) => asn.fmt(fmt),
            ASN::AS_32(asn) => asn.fmt(fmt),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ASN16 {
    n: u16,
}
impl Display for ASN16 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "AS{}", self.n)
    }
}
impl ASN16 {
    pub const fn new(n: u16) -> Self {
        ASN16 { n }
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ASN32 {
    n: u32,
}
impl ASN32 {
    pub const fn new(n: u32) -> Self {
        ASN32 { n }
    }
}
impl Display for ASN32 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "AS{}", self.n)
    }
}
