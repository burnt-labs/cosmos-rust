// @generated
/// AuthzAllowance creates allowance only authz message for a specific grantee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthzAllowance {
    /// allowance can be any of basic and periodic fee allowance.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::pbjson_types::Any>,
    /// The address that can use this authorization-based allowance
    #[prost(string, tag = "2")]
    pub authz_grantee: ::prost::alloc::string::String,
}
/// ContractsAllowance creates allowance only for specific contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractsAllowance {
    /// allowance can be any allowance interface type.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::pbjson_types::Any>,
    /// List of contract addresses that this allowance applies to
    #[prost(string, repeated, tag = "2")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MultiAnyAllowance creates an allowance that pays if any of the internal
/// allowances are met
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiAnyAllowance {
    /// allowance can be any allowance interface type.
    #[prost(message, repeated, tag = "1")]
    pub allowances: ::prost::alloc::vec::Vec<::pbjson_types::Any>,
}
/// GenesisState defines the xion module's genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// The percentage fee taken by the platform
    #[prost(uint32, tag = "1")]
    pub platform_percentage: u32,
    /// Minimum amounts required for platform operations
    #[prost(message, repeated, tag = "2")]
    pub platform_minimums: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryWebAuthNVerifyRegisterRequest is the request type for WebAuthN
/// registration verification
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyRegisterRequest {
    /// The account address
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// The challenge string for registration
    #[prost(string, tag = "2")]
    pub challenge: ::prost::alloc::string::String,
    /// The relying party identifier
    #[prost(string, tag = "3")]
    pub rp: ::prost::alloc::string::String,
    /// The registration data
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryWebAuthNVerifyRegisterResponse is the response type for WebAuthN
/// registration verification
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyRegisterResponse {
    /// The generated credential
    #[prost(bytes = "vec", tag = "1")]
    pub credential: ::prost::alloc::vec::Vec<u8>,
}
/// QueryWebAuthNVerifyAuthenticateRequest is the request type for WebAuthN
/// authentication verification
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyAuthenticateRequest {
    /// The account address
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    /// The challenge string for authentication
    #[prost(string, tag = "2")]
    pub challenge: ::prost::alloc::string::String,
    /// The relying party identifier
    #[prost(string, tag = "3")]
    pub rp: ::prost::alloc::string::String,
    /// The credential to verify
    #[prost(bytes = "vec", tag = "4")]
    pub credential: ::prost::alloc::vec::Vec<u8>,
    /// The authentication data
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryWebAuthNVerifyAuthenticateResponse is the response type for WebAuthN
/// authentication verification
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWebAuthNVerifyAuthenticateResponse {}
/// QueryPlatformPercentageRequest is the request type for querying platform
/// percentage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformPercentageRequest {}
/// QueryPlatformPercentageResponse is the response type for querying platform
/// percentage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformPercentageResponse {
    /// The platform percentage fee
    #[prost(uint64, tag = "1")]
    pub platform_percentage: u64,
}
/// QueryPlatformMinimumRequest is the request type for querying platform minimum
/// fees
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformMinimumRequest {}
/// QueryPlatformMinimumResponse is the response type for querying platform
/// minimum fees
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlatformMinimumResponse {
    /// The minimum fees required by the platform
    #[prost(message, repeated, tag = "3")]
    pub minimums: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSend represents a message to send coins from one account to another.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    /// The address sending the coins
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    /// The address receiving the coins
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    /// The amount of coins to send
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSendResponse defines the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
/// MsgMultiSend represents an arbitrary multi-in, multi-out send message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSend {
    /// Inputs, despite being `repeated`, only allows one sender input. This is
    /// checked in MsgMultiSend's ValidateBasic.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<super::super::cosmos::bank::v1beta1::Input>,
    /// The outputs specifying recipient addresses and amounts
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<super::super::cosmos::bank::v1beta1::Output>,
}
/// MsgMultiSendResponse defines the Msg/MultiSend response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMultiSendResponse {}
/// MsgSetPlatformPercentage defines the message for setting platform percentage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformPercentage {
    /// The authority address that can set the platform percentage
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// platform_percentage is the platform fee percentage to multiplied by 10000
    #[prost(uint32, tag = "2")]
    pub platform_percentage: u32,
}
/// MsgSetPlatformPercentageResponse defines the response for setting platform
/// percentage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformPercentageResponse {}
/// MsgSetPlatformMinimum defines the message for setting platform minimum fees
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformMinimum {
    /// The authority address that can set the platform minimums
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The minimum fees required by the platform
    #[prost(message, repeated, tag = "3")]
    pub minimums: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetPlatformMinimumResponse defines the response for setting platform
/// minimum fees
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPlatformMinimumResponse {}
include!("xion.v1.serde.rs");
include!("xion.v1.tonic.rs");
// @@protoc_insertion_point(module)
