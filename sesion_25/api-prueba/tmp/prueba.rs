#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaludoRequest {
    /// tipo nombre-campo = posicion
    ///
    /// [ header | nombre]
    #[prost(string, tag = "1")]
    pub nombre: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaludoReply {
    #[prost(string, tag = "1")]
    pub saludo: ::prost::alloc::string::String,
}
