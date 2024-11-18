// @generated
/// Module is the app config object of the module.
/// Learn more: <https://docs.cosmos.network/main/building-modules/depinject>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "dkim.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("dkim.module.v1.{}", Self::NAME)
    }
}
include!("dkim.module.v1.serde.rs");
// @@protoc_insertion_point(module)
