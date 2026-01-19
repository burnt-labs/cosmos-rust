// @generated
/// DkimPubKey represents a DKIM public key with associated metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkimPubKey {
    /// domain defines the email domain associated with this DKIM key.
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// pub_key defines the base64 encoded public key.
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    /// poseidon_hash defines the Poseidon hash of the public key.
    #[prost(bytes = "vec", tag = "3")]
    pub poseidon_hash: ::prost::alloc::vec::Vec<u8>,
    /// selector defines the DKIM selector for this key.
    #[prost(string, tag = "4")]
    pub selector: ::prost::alloc::string::String,
    /// version defines the DKIM version.
    #[prost(enumeration = "Version", tag = "5")]
    pub version: i32,
    /// key_type defines the cryptographic key type.
    #[prost(enumeration = "KeyType", tag = "6")]
    pub key_type: i32,
}
/// Version defines the supported DKIM versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Version {
    /// VERSION_DKIM1_UNSPECIFIED represents DKIM version 1 (unspecified default).
    Dkim1Unspecified = 0,
}
impl Version {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Version::Dkim1Unspecified => "VERSION_DKIM1_UNSPECIFIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VERSION_DKIM1_UNSPECIFIED" => Some(Self::Dkim1Unspecified),
            _ => None,
        }
    }
}
/// KeyType defines the supported cryptographic key types for DKIM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    /// KEY_TYPE_RSA_UNSPECIFIED represents RSA key type (unspecified default).
    RsaUnspecified = 0,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::RsaUnspecified => "KEY_TYPE_RSA_UNSPECIFIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KEY_TYPE_RSA_UNSPECIFIED" => Some(Self::RsaUnspecified),
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
    /// dkim_pubkeys stores the list of active DKIM public keys.
    #[prost(message, repeated, tag = "2")]
    pub dkim_pubkeys: ::prost::alloc::vec::Vec<DkimPubKey>,
    /// revoked_pubkeys stores the list of revoked DKIM public keys.
    #[prost(string, repeated, tag = "3")]
    pub revoked_pubkeys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// vkey defines the verification key used by the module.
    #[prost(uint64, tag = "1")]
    pub vkey_identifier: u64,
    /// max_pubkey_size_bytes caps the allowed DKIM public key size (base64
    /// decoded).
    #[prost(uint64, tag = "2")]
    pub max_pubkey_size_bytes: u64,
}
/// QueryDkimPubKeyRequest is the request type for the Query/DkimPubKey RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeyRequest {
    /// selector defines the DKIM selector to query.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// domain defines the domain to query.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
}
/// QueryDkimPubKeyResponse is the response type for the Query/DkimPubKey RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeyResponse {
    /// dkim_pub_key defines the DKIM public key.
    #[prost(message, optional, tag = "1")]
    pub dkim_pub_key: ::core::option::Option<DkimPubKey>,
}
/// QueryDkimPubKeysRequest is the request type for the Query/DkimPubKeys RPC
/// method. All fields are optional, and will filter down results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeysRequest {
    /// selector defines the DKIM selector to filter by.
    #[prost(string, tag = "1")]
    pub selector: ::prost::alloc::string::String,
    /// domain defines the domain to filter by.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    /// poseidon_hash defines the Poseidon hash to filter by.
    #[prost(bytes = "vec", tag = "3")]
    pub poseidon_hash: ::prost::alloc::vec::Vec<u8>,
    /// pagination defines the pagination parameters.
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDkimPubKeysResponse is the response type for the Query/DkimPubKeys RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkimPubKeysResponse {
    /// dkim_pub_keys defines the list of DKIM public keys.
    #[prost(message, repeated, tag = "1")]
    pub dkim_pub_keys: ::prost::alloc::vec::Vec<DkimPubKey>,
    /// pagination defines the pagination response.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAuthenticateRequest defines the request structure for proof
/// verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthenticateRequest {
    /// tx_bytes defines the serialized transaction bytes.
    #[prost(bytes = "vec", tag = "1")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    /// email_hash defines the hash of the email being authenticated.
    #[prost(string, tag = "2")]
    pub email_hash: ::prost::alloc::string::String,
    /// proof defines the zk proof bytes.
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// public_inputs defines the public inputs for the zk proof.
    #[prost(string, repeated, tag = "4")]
    pub public_inputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allowed_email_hosts defines the list of allowed email hosts.
    #[prost(string, repeated, tag = "5")]
    pub allowed_email_hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AuthenticateResponse defines the response structure for proof verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    /// verified indicates whether the proof verification was successful.
    #[prost(bool, tag = "1")]
    pub verified: bool,
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
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
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
/// MsgAddDkimPubKey is the Msg/AddDkimPubKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddDkimPubKeys {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// dkim_pubkeys defines the DKIM public keys to add.
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
    /// selector defines the DKIM selector to remove.
    #[prost(string, tag = "2")]
    pub selector: ::prost::alloc::string::String,
    /// domain defines the domain for the DKIM key to remove.
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
    /// signer defines the address of the signer revoking the key.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// domain defines the domain for the DKIM key to revoke.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    /// priv_key defines the private key used to prove ownership for revocation.
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
