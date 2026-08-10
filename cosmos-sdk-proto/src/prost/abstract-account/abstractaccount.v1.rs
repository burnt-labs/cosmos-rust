// @generated
/// AbstractAccount is a smart contract that is capable of initiating txs.
///
/// This account type is similar to BaseAccount except for it doesn't have a
/// pubkey. If a pubkey is needed, it creates and returns a new NilPubKey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbstractAccount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub account_number: u64,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
}
/// NilPubKey is the pubkey type of the AbstractAccount. Basically, it represents
/// a pubkey that doesn't exist.
///
/// The actual pubkey of an AbstractAccount (if it has one) is to be stored
/// inside the contract, not at the SDK level. Signature verification is also
/// done inside the contract, typically in the BeforeTx hook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NilPubKey {
    #[prost(bytes = "vec", tag = "1")]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAccountRegistered {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub code_id: u64,
    #[prost(string, tag = "3")]
    pub contract_addr: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// AllowAllCodeIDs determines whether a Wasm code ID can be used to register
    /// AbstractAccounts:
    /// - if set to true, any code ID can be used;
    /// - if set to false, only code IDs whitelisted in the AllowedCodeIDs list can
    /// be used.
    #[prost(bool, tag = "1")]
    pub allow_all_code_ids: bool,
    /// AllowedCodeIDs is the whitelist of Wasm code IDs that can be used to
    /// regiseter AbstractAccounts.
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub allowed_code_ids: ::prost::alloc::vec::Vec<u64>,
    /// MaxGasBefore is the maximum amount of gas that can be consumed by the
    /// contract call in the before_tx decorator.
    ///
    /// Must be greater than zero.
    #[prost(uint64, tag = "3")]
    pub max_gas_before: u64,
    /// MaxGasAfter is the maximum amount of gas that can be consumed by the
    /// contract call in the after_tx decorator.
    ///
    /// Must be greater than zero.
    #[prost(uint64, tag = "4")]
    pub max_gas_after: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(uint64, tag = "2")]
    pub next_account_id: u64,
}
// ---------------------------------- Params -----------------------------------

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
// ------------------------------- UpdateParams --------------------------------

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
// ------------------------------ RegisterAccount ------------------------------

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccount {
    /// Sender is the actor who signs the message
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeID indicates which wasm binary code is to be used for this contract
    #[prost(uint64, tag = "2")]
    pub code_id: u64,
    /// Msg is the JSON-encoded instantiate message for the contract
    #[prost(bytes = "vec", tag = "3")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds are coins to be deposited to the contract on instantiattion
    #[prost(message, repeated, tag = "4")]
    pub funds: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// Salt is an arbinary value to be used in deriving the account address.
    /// Max 64 bytes.
    #[prost(bytes = "vec", tag = "5")]
    pub salt: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccountResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
include!("abstractaccount.v1.serde.rs");
include!("abstractaccount.v1.tonic.rs");
// @@protoc_insertion_point(module)
