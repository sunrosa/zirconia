macro_rules! string_newtypes {
  [
    $(#[$meta:meta])*
    $name:ident
  ] => {
    $(#[$meta])*
    #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default, ::std::hash::Hash, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::cmp::PartialOrd, ::std::cmp::Ord, ::serde::Serialize, ::serde::Deserialize, ::derive_more::Mul, ::derive_more::MulAssign, ::derive_more::Div, ::derive_more::DivAssign, ::derive_more::Display)]
    pub struct $name(pub String);
  };
  [
    $($(#[$meta:meta])*
    $name:ident,)+
  ] => {
    $($(#[$meta])*
    #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default, ::std::hash::Hash, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::cmp::PartialOrd, ::std::cmp::Ord, ::serde::Serialize, ::serde::Deserialize, ::derive_more::Mul, ::derive_more::MulAssign, ::derive_more::Div, ::derive_more::DivAssign, ::derive_more::Display)]
    pub struct $name(pub String);)*
  };
}
pub(crate) use string_newtypes;

macro_rules! integer_newtypes {
  [
    $(#[$meta:meta])*
    $name:ident($type:ty)
  ] => {
    $(#[$meta])*
    #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::default::Default, ::std::hash::Hash, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::cmp::PartialOrd, ::std::cmp::Ord, ::serde::Serialize, ::serde::Deserialize, ::derive_more::Add, ::derive_more::AddAssign, ::derive_more::Sub, ::derive_more::SubAssign, ::derive_more::Mul, ::derive_more::MulAssign, ::derive_more::Div, ::derive_more::DivAssign, ::derive_more::Display)]
    pub struct $name(pub $type);
  };
  [
    $($(#[$meta:meta])*
    $name:ident($type:ty),)+
  ] => {
    $($(#[$meta])*
    #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::default::Default, ::std::hash::Hash, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::cmp::PartialOrd, ::std::cmp::Ord, ::serde::Serialize, ::serde::Deserialize, ::derive_more::Add, ::derive_more::AddAssign, ::derive_more::Sub, ::derive_more::SubAssign, ::derive_more::Mul, ::derive_more::MulAssign, ::derive_more::Div, ::derive_more::DivAssign, ::derive_more::Display)]
    pub struct $name(pub $type);)*
  };
}
pub(crate) use integer_newtypes;

macro_rules! float_newtypes {
  [
    $(#[$meta:meta])*
    $name:ident($type:ty)
  ] => {
    $(#[$meta])*
    #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::default::Default, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::serde::Serialize, ::serde::Deserialize, ::derive_more::Add, ::derive_more::AddAssign, ::derive_more::Sub, ::derive_more::SubAssign, ::derive_more::Mul, ::derive_more::MulAssign, ::derive_more::Div, ::derive_more::DivAssign, ::derive_more::Display)]
    pub struct $name(pub $type);
  };
  [
    $($(#[$meta:meta])*
    $name:ident($type:ty),)+
  ] => {
    $($(#[$meta])*
    #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::marker::Copy, ::std::default::Default, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::serde::Serialize, ::serde::Deserialize, ::derive_more::Add, ::derive_more::AddAssign, ::derive_more::Sub, ::derive_more::SubAssign, ::derive_more::Mul, ::derive_more::MulAssign, ::derive_more::Div, ::derive_more::DivAssign, ::derive_more::Display)]
    pub struct $name(pub $type);)*
  };
}
pub(crate) use float_newtypes;
