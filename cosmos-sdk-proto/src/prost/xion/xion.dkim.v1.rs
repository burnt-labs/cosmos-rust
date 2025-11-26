// @generated
// <https://github.com/cosmos/cosmos-sdk/blob/main/orm/README.md>

/// DkimPubKey represents a DKIM public key with associated metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkimPubKey {
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// base64 encoded public key
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    /// hash of the public key
    #[prost(bytes = "vec", tag = "3")]
    pub poseidon_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub selector: ::prost::alloc::string::String,
    #[prost(enumeration = "Version", tag = "5")]
    pub version: i32,
    #[prost(enumeration = "KeyType", tag = "6")]
    pub key_type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Version {
    Dkim1 = 0,
}
impl Version {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Version::Dkim1 => "DKIM1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DKIM1" => Some(Self::Dkim1),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Rsa = 0,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::Rsa => "RSA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RSA" => Some(Self::Rsa),
            _ => None,
        }
    }
}
/// GenesisState defines the module genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub dkim_pubkeys: ::prost::alloc::vec::Vec<DkimPubKey>,
}
/// Params defines the set of module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(bytes = "vec", tag = "1")]
    pub vkey: ::prost::alloc::vec::Vec<u8>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDkimPubKeyRequest is the request type for the Query/DkimPubKey RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeyRequest {
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
}
/// QueryDkimPubKeyResponse is the response type for the Query/DkimPubKey RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub dkim_pub_key: ::core::option::Option<DkimPubKey>,
}
/// QueryDkimPubKeysRequest is the request type for the Query/DkimPubKeys RPC
/// method. All fields are optional, and will filter down results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeysRequest {
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub poseidon_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDkimPubKeysResponse is the response type for the Query/DkimPubKeys RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeysResponse {
    #[prost(message, repeated, tag = "1")]
    pub dkim_pub_keys: ::prost::alloc::vec::Vec<DkimPubKey>,
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnarkJsProof {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub pi_a: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub pi_b: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub pi_c: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthenticateRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub email_hash: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "4")]
    pub public_inputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub allowed_email_hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    #[prost(bool, tag = "1")]
    pub verified: bool,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// MsgAddDkimPubKey is the Msg/AddDkimPubKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddDkimPubKeys {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub dkim_pubkeys: ::prost::alloc::vec::Vec<DkimPubKey>,
}
/// MsgAddDkimPubKeyResponse defines the response structure for executing a
/// MsgAddDkimPubKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddDkimPubKeysResponse {}
/// MsgRemoveDkimPubKey is the Msg/RemoveDkimPubKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveDkimPubKey {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub selector: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub domain: ::prost::alloc::string::String,
}
/// MsgRemoveDkimPubKeyResponse defines the response structure for executing a
/// MsgRemoveDkimPubKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveDkimPubKeyResponse {}
/// MsgRevokeDkimPubKey is the Msg/RevokeDkimPubKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeDkimPubKey {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub priv_key: ::prost::alloc::vec::Vec<u8>,
}
/// MsgRevokeDkimPubKeyResponse defines the response structure for executing a
/// MsgRevokeDkimPubKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeDkimPubKeyResponse {}
include!("xion.dkim.v1.serde.rs");
include!("xion.dkim.v1.tonic.rs");
// @@protoc_insertion_point(module)
