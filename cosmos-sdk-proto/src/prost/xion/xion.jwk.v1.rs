// @generated
/// Audience represents a JWT audience configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    /// The audience identifier
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
    /// The public key associated with this audience
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// The admin address for this audience
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
}
/// AudienceClaim represents a claim for an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceClaim {
    /// The signer of the audience claim
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Time offset in nanoseconds for JWT validation
    #[prost(uint64, tag = "1")]
    pub time_offset: u64,
    /// Gas required to deploy a new project/audience
    #[prost(uint64, tag = "2")]
    pub deployment_gas: u64,
}
/// GenesisState defines the jwk module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// The module parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// List of all audiences
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
/// QueryAudienceClaimRequest is the request type for querying an audience claim
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceClaimRequest {
    /// The hash of the audience claim to query
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAudienceClaimResponse is the response type for querying an audience
/// claim
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceClaimResponse {
    /// The audience claim
    #[prost(message, optional, tag = "1")]
    pub claim: ::core::option::Option<AudienceClaim>,
}
/// QueryGetAudienceClaimRequest is the legacy request type for querying an
/// audience claim (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceClaimRequest {
    /// The hash of the audience claim to query
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryGetAudienceClaimResponse is the legacy response type for querying an
/// audience claim (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceClaimResponse {
    /// The audience claim
    #[prost(message, optional, tag = "1")]
    pub claim: ::core::option::Option<AudienceClaim>,
}
/// QueryAudienceRequest is the request type for querying an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceRequest {
    /// The audience identifier to query
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
}
/// QueryAudienceResponse is the response type for querying an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceResponse {
    /// The audience information
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
/// QueryGetAudienceRequest is the legacy request type for querying an audience
/// (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceRequest {
    /// The audience identifier to query
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
}
/// QueryGetAudienceResponse is the legacy response type for querying an audience
/// (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAudienceResponse {
    /// The audience information
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
/// QueryAudienceAllRequest is the request type for querying all audiences
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceAllRequest {
    /// Pagination parameters
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAudienceAllResponse is the response type for querying all audiences
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceAllResponse {
    /// List of all audiences
    #[prost(message, repeated, tag = "1")]
    pub audience: ::prost::alloc::vec::Vec<Audience>,
    /// Pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAllAudienceRequest is the legacy request type for querying all audiences
/// (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAudienceRequest {
    /// Pagination parameters
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllAudienceResponse is the legacy response type for querying all
/// audiences (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAudienceResponse {
    /// List of all audiences
    #[prost(message, repeated, tag = "1")]
    pub audience: ::prost::alloc::vec::Vec<Audience>,
    /// Pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryValidateJWTRequest is the request type for validating a JWT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidateJwtRequest {
    /// The audience identifier
    #[prost(string, tag = "1")]
    pub aud: ::prost::alloc::string::String,
    /// The subject
    #[prost(string, tag = "2")]
    pub sub: ::prost::alloc::string::String,
    /// The signature bytes
    #[prost(string, tag = "3")]
    pub sig_bytes: ::prost::alloc::string::String,
}
/// PrivateClaim represents a private claim in a JWT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateClaim {
    /// The claim key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The claim value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// QueryValidateJWTResponse is the response type for validating a JWT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidateJwtResponse {
    /// The private claims from the JWT
    #[prost(message, repeated, tag = "1")]
    pub private_claims: ::prost::alloc::vec::Vec<PrivateClaim>,
}
/// MsgCreateAudienceClaim defines the message for creating an audience claim
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudienceClaim {
    /// The admin address creating the claim
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The hash of the audience for this claim
    #[prost(bytes = "vec", tag = "2")]
    pub aud_hash: ::prost::alloc::vec::Vec<u8>,
}
/// MsgCreateAudienceClaimResponse defines the response for creating an audience
/// claim
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudienceClaimResponse {}
/// MsgDeleteAudienceClaim defines the message for deleting an audience claim
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudienceClaim {
    /// The admin address deleting the claim
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The hash of the audience for this claim
    #[prost(bytes = "vec", tag = "2")]
    pub aud_hash: ::prost::alloc::vec::Vec<u8>,
}
/// MsgDeleteAudienceClaimResponse defines the response for deleting an audience
/// claim
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudienceClaimResponse {}
/// MsgCreateAudience defines the message for creating an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudience {
    /// The admin address creating the audience
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The audience identifier
    #[prost(string, tag = "2")]
    pub aud: ::prost::alloc::string::String,
    /// The public key for this audience
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
/// MsgCreateAudienceResponse defines the response for creating an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAudienceResponse {
    /// The created audience
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
/// MsgUpdateAudience defines the message for updating an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAudience {
    /// The current admin address
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The new admin address
    #[prost(string, tag = "2")]
    pub new_admin: ::prost::alloc::string::String,
    /// The current audience identifier
    #[prost(string, tag = "3")]
    pub aud: ::prost::alloc::string::String,
    /// The current public key
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
    /// The new audience identifier
    #[prost(string, tag = "5")]
    pub new_aud: ::prost::alloc::string::String,
}
/// MsgUpdateAudienceResponse defines the response for updating an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAudienceResponse {
    /// The updated audience
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
}
/// MsgDeleteAudience defines the message for deleting an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudience {
    /// The admin address deleting the audience
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The audience identifier to delete
    #[prost(string, tag = "2")]
    pub aud: ::prost::alloc::string::String,
}
/// MsgDeleteAudienceResponse defines the response for deleting an audience
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAudienceResponse {}
include!("xion.jwk.v1.serde.rs");
include!("xion.jwk.v1.tonic.rs");
// @@protoc_insertion_point(module)
