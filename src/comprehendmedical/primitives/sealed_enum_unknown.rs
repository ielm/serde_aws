use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
    Serialize,
    Deserialize,
)]
pub struct UnknownVariantValue_(pub(crate) ::std::string::String);
impl UnknownVariantValue_ {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}
impl ::std::fmt::Display for UnknownVariantValue_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
