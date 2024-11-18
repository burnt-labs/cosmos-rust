// @generated
/// AuthzAllowance creates allowance only authz message for a specific grantee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthzAllowance {
    /// allowance can be any of basic and periodic fee allowance.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
    #[prost(string, tag = "2")]
    pub authz_grantee: ::prost::alloc::string::String,
}
impl ::prost::Name for AuthzAllowance {
    const NAME: &'static str = "AuthzAllowance";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
/// ContractsAllowance creates allowance only for specific contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractsAllowance {
    /// allowance can be any allowance interface type.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
    #[prost(string, repeated, tag = "2")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for ContractsAllowance {
    const NAME: &'static str = "ContractsAllowance";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
/// MultiAnyAllowance creates an allowance that pays if any of the internal
/// allowances are met
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiAnyAllowance {
    /// allowance can be any allowance interface type.
    #[prost(message, repeated, tag = "1")]
    pub allowances: ::prost::alloc::vec::Vec<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for MultiAnyAllowance {
    const NAME: &'static str = "MultiAnyAllowance";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(uint32, tag = "1")]
    pub platform_percentage: u32,
    #[prost(message, repeated, tag = "2")]
    pub platform_minimums: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyRegisterRequest {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub challenge: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub rp: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for QueryWebAuthNVerifyRegisterRequest {
    const NAME: &'static str = "QueryWebAuthNVerifyRegisterRequest";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyRegisterResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub credential: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for QueryWebAuthNVerifyRegisterResponse {
    const NAME: &'static str = "QueryWebAuthNVerifyRegisterResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyAuthenticateRequest {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub challenge: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub rp: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub credential: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for QueryWebAuthNVerifyAuthenticateRequest {
    const NAME: &'static str = "QueryWebAuthNVerifyAuthenticateRequest";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyAuthenticateResponse {}
impl ::prost::Name for QueryWebAuthNVerifyAuthenticateResponse {
    const NAME: &'static str = "QueryWebAuthNVerifyAuthenticateResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformPercentageRequest {}
impl ::prost::Name for QueryPlatformPercentageRequest {
    const NAME: &'static str = "QueryPlatformPercentageRequest";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformPercentageResponse {
    #[prost(uint64, tag = "1")]
    pub platform_percentage: u64,
}
impl ::prost::Name for QueryPlatformPercentageResponse {
    const NAME: &'static str = "QueryPlatformPercentageResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformMinimumRequest {}
impl ::prost::Name for QueryPlatformMinimumRequest {
    const NAME: &'static str = "QueryPlatformMinimumRequest";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformMinimumResponse {
    #[prost(message, repeated, tag = "3")]
    pub minimums: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryPlatformMinimumResponse {
    const NAME: &'static str = "QueryPlatformMinimumResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
/// MsgSend represents a message to send coins from one account to another.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgSend {
    const NAME: &'static str = "MsgSend";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
/// MsgSendResponse defines the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
impl ::prost::Name for MsgSendResponse {
    const NAME: &'static str = "MsgSendResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
/// MsgMultiSend represents an arbitrary multi-in, multi-out send message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSend {
    /// Inputs, despite being `repeated`, only allows one sender input. This is
    /// checked in MsgMultiSend's ValidateBasic.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<super::super::cosmos::bank::v1beta1::Input>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<super::super::cosmos::bank::v1beta1::Output>,
}
impl ::prost::Name for MsgMultiSend {
    const NAME: &'static str = "MsgMultiSend";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
/// MsgMultiSendResponse defines the Msg/MultiSend response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSendResponse {}
impl ::prost::Name for MsgMultiSendResponse {
    const NAME: &'static str = "MsgMultiSendResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformPercentage {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// platform_percentage is the platform fee percentage to multiplied by 10000
    #[prost(uint32, tag = "2")]
    pub platform_percentage: u32,
}
impl ::prost::Name for MsgSetPlatformPercentage {
    const NAME: &'static str = "MsgSetPlatformPercentage";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformPercentageResponse {}
impl ::prost::Name for MsgSetPlatformPercentageResponse {
    const NAME: &'static str = "MsgSetPlatformPercentageResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformMinimum {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub minimums: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgSetPlatformMinimum {
    const NAME: &'static str = "MsgSetPlatformMinimum";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformMinimumResponse {}
impl ::prost::Name for MsgSetPlatformMinimumResponse {
    const NAME: &'static str = "MsgSetPlatformMinimumResponse";
    const PACKAGE: &'static str = "xion.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("xion.v1.{}", Self::NAME)
    }
}
include!("xion.v1.serde.rs");
include!("xion.v1.tonic.rs");
// @@protoc_insertion_point(module)
