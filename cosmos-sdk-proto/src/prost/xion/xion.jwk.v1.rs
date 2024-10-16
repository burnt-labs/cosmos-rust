// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceClaim {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// in nanoseconds
    #[prost(uint64, tag = "1")]
    pub time_offset: u64,
    /// gas to deploy a new project/audience
    #[prost(uint64, tag = "2")]
    pub deployment_gas: u64,
}
/// GenesisState defines the jwk module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub audience_list: ::prost::alloc::vec::Vec<Audience>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceClaimRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceClaimResponse {
    #[prost(message, optional, tag = "1")]
    pub claim: ::core::option::Option<AudienceClaim>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceRequest {
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceResponse {
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAudienceRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAudienceResponse {
    #[prost(message, repeated, tag = "1")]
    pub audience: ::prost::alloc::vec::Vec<Audience>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidateJwtRequest {
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sub: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sig_bytes: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateClaim {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidateJwtResponse {
    #[prost(message, repeated, tag = "1")]
    pub private_claims: ::prost::alloc::vec::Vec<PrivateClaim>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudienceClaim {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub aud_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudienceClaimResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudienceClaim {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub aud_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudienceClaimResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudience {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub aud: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudienceResponse {
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAudience {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_admin: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub aud: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAudienceResponse {
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudience {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub aud: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudienceResponse {}
include!("xion.jwk.v1.serde.rs");
include!("xion.jwk.v1.tonic.rs");
// @@protoc_insertion_point(module)
