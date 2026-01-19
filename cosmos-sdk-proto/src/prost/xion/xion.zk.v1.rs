// @generated
/// Params defines the zk module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// max_vkey_size_bytes caps the size of a verification key JSON payload.
    #[prost(uint64, tag = "1")]
    pub max_vkey_size_bytes: u64,
    /// upload_chunk_size defines the byte-size of each gas tier.
    #[prost(uint64, tag = "2")]
    pub upload_chunk_size: u64,
    /// upload_chunk_gas defines the gas cost per upload chunk.
    #[prost(uint64, tag = "3")]
    pub upload_chunk_gas: u64,
}
/// SnarkJsProof represents a ZK-SNARK proof in SnarkJS format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnarkJsProof {
    /// pi_a defines the first component of the proof.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub pi_a: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// pi_b defines the second component of the proof.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub pi_b: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// pi_c defines the third component of the proof.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub pi_c: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// QueryVerifyRequest defines the request structure for proof verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyRequest {
    /// proof is the serialized ZK proof bytes.
    #[prost(bytes = "vec", tag = "1")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// public_inputs are the public inputs for the ZK circuit.
    #[prost(string, repeated, tag = "2")]
    pub public_inputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// vkey_name is the name of the verification key to use.
    #[prost(string, tag = "3")]
    pub vkey_name: ::prost::alloc::string::String,
    /// vkey_id is the unique identifier of the verification key to use.
    #[prost(uint64, tag = "4")]
    pub vkey_id: u64,
}
/// ProofVerifyResponse defines the response structure for proof verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProofVerifyResponse {
    /// verified indicates whether the proof verification was successful.
    #[prost(bool, tag = "1")]
    pub verified: bool,
}
/// VKey represents a verification key for ZK proof verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VKey {
    /// key_bytes is the serialized verification key data.
    #[prost(bytes = "vec", tag = "1")]
    pub key_bytes: ::prost::alloc::vec::Vec<u8>,
    /// name is the unique name identifier for the verification key.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// description provides a human-readable description of the verification key.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// circuit_hash is the hash of the circuit this key verifies.
    #[prost(string, tag = "4")]
    pub circuit_hash: ::prost::alloc::string::String,
    /// authority is the uploader of the verification key.
    #[prost(string, tag = "5")]
    pub authority: ::prost::alloc::string::String,
}
/// QueryVKeyRequest is the request type for the Query/VKey RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVKeyRequest {
    /// id is the unique identifier of the verification key
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// QueryVKeyResponse is the response type for the Query/VKey RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVKeyResponse {
    /// vkey is the verification key
    #[prost(message, optional, tag = "1")]
    pub vkey: ::core::option::Option<VKey>,
}
/// QueryVKeyByNameRequest is the request type for the Query/VKeyByName RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVKeyByNameRequest {
    /// name is the unique name of the verification key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryVKeyByNameResponse is the response type for the Query/VKeyByName RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVKeyByNameResponse {
    /// vkey is the verification key
    #[prost(message, optional, tag = "1")]
    pub vkey: ::core::option::Option<VKey>,
    /// id is the numeric identifier of the verification key
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
/// QueryVKeysRequest is the request type for the Query/VKeys RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVKeysRequest {
    /// pagination defines an optional pagination for the request
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVKeysResponse is the response type for the Query/VKeys RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVKeysResponse {
    /// vkeys is the list of all verification keys with their IDs
    #[prost(message, repeated, tag = "1")]
    pub vkeys: ::prost::alloc::vec::Vec<VKeyWithId>,
    /// pagination defines the pagination in the response
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// VKeyWithID combines a verification key with its ID
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VKeyWithId {
    /// id is the unique identifier
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// vkey is the verification key
    #[prost(message, optional, tag = "2")]
    pub vkey: ::core::option::Option<VKey>,
}
/// QueryHasVKeyRequest is the request type for the Query/HasVKey RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHasVKeyRequest {
    /// name is the name of the verification key to check
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryHasVKeyResponse is the response type for the Query/HasVKey RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHasVKeyResponse {
    /// exists indicates whether the verification key exists
    #[prost(bool, tag = "1")]
    pub exists: bool,
    /// id is the numeric identifier if the key exists (0 if not found)
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
/// QueryNextVKeyIDRequest is the request type for the Query/NextVKeyID RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextVKeyIdRequest {}
/// QueryNextVKeyIDResponse is the response type for the Query/NextVKeyID RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextVKeyIdResponse {
    /// next_id is the next available verification key identifier.
    #[prost(uint64, tag = "1")]
    pub next_id: u64,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params is the current zk module parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// GenesisState defines the zk module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// vkeys is the list of all verification keys with their IDs
    #[prost(message, repeated, tag = "1")]
    pub vkeys: ::prost::alloc::vec::Vec<VKeyWithId>,
    /// params defines the module parameters.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgAddVKey is the message for adding a verification key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddVKey {
    /// authority is the address that controls the module (governance)
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// name is the unique identifier for this verification key
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// description provides context about this verification key
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// vkey_json is the JSON representation of the verification key
    #[prost(bytes = "vec", tag = "4")]
    pub vkey_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// MsgAddVKeyResponse is the response for MsgAddVKey
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddVKeyResponse {
    /// id is the unique numeric identifier assigned to the vkey
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// MsgUpdateVKey is the message for updating a verification key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateVKey {
    /// authority is the address that controls the module (governance)
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// name is the identifier of the verification key to update
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// description provides updated context
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// vkey_json is the new JSON representation of the verification key
    #[prost(bytes = "vec", tag = "4")]
    pub vkey_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// MsgUpdateVKeyResponse is the response for MsgUpdateVKey
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateVKeyResponse {}
/// MsgRemoveVKey is the message for removing a verification key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveVKey {
    /// authority is the address that controls the module (governance)
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// name is the identifier of the verification key to remove
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// MsgRemoveVKeyResponse is the response for MsgRemoveVKey
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveVKeyResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("xion.zk.v1.serde.rs");
include!("xion.zk.v1.tonic.rs");
// @@protoc_insertion_point(module)
