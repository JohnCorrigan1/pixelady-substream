// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub token_id: u64,
    #[prost(string, tag="4")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mints {
    #[prost(message, repeated, tag="1")]
    pub mints: ::prost::alloc::vec::Vec<Mint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag="1")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub token_id: u64,
}
// @@protoc_insertion_point(module)
