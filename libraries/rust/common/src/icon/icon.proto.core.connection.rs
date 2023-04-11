// @generated
// ICS03 - Connection Data Structures as defined in
// <https://github.com/cosmos/ibc/blob/master/spec/core/ics-003-connection-semantics#data-structures>

/// ConnectionEnd defines a stateful object on a chain connected to another
/// separate one.
/// NOTE: there must only be 2 defined ConnectionEnds to establish
/// a connection between two chains.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionEnd {
    /// client associated with this connection.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection.
    #[prost(message, repeated, tag="2")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration="connection_end::State", tag="3")]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag="4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period that must pass before a consensus state can be used for
    /// packet-verification NOTE: delay period logic is only implemented by some
    /// clients.
    #[prost(uint64, tag="5")]
    pub delay_period: u64,
}
/// Nested message and enum types in `ConnectionEnd`.
pub mod connection_end {
    /// State defines if a connection is in one of the following states:
    /// INIT, TRYOPEN, OPEN or UNINITIALIZED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default State
        StateUninitializedUnspecified = 0,
        /// A connection end has just started the opening handshake.
        StateInit = 1,
        /// A connection end has acknowledged the handshake step on the counterparty
        /// chain.
        StateTryopen = 2,
        /// A connection end has completed the handshake.
        StateOpen = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::StateUninitializedUnspecified => "STATE_UNINITIALIZED_UNSPECIFIED",
                State::StateInit => "STATE_INIT",
                State::StateTryopen => "STATE_TRYOPEN",
                State::StateOpen => "STATE_OPEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNINITIALIZED_UNSPECIFIED" => Some(Self::StateUninitializedUnspecified),
                "STATE_INIT" => Some(Self::StateInit),
                "STATE_TRYOPEN" => Some(Self::StateTryopen),
                "STATE_OPEN" => Some(Self::StateOpen),
                _ => None,
            }
        }
    }
}
/// Counterparty defines the counterparty chain associated with a connection end.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counterparty {
    /// identifies the client on the counterparty chain associated with a given
    /// connection.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// identifies the connection end on the counterparty chain associated with a
    /// given connection.
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// commitment merkle prefix of the counterparty chain.
    #[prost(message, optional, tag="3")]
    pub prefix: ::core::option::Option<MerklePrefix>,
}
/// MerklePrefix is merkle path prefixed to the key.
/// The constructed key from the Path and the key will be append(Path.KeyPath,
/// append(Path.KeyPrefix, key...))
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePrefix {
    #[prost(bytes="vec", tag="1")]
    pub key_prefix: ::prost::alloc::vec::Vec<u8>,
}
/// Version defines the versioning scheme used to negotiate the IBC verison in
/// the connection handshake.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// unique version identifier
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
    /// list of features compatible with the specified identifier
    #[prost(string, repeated, tag="2")]
    pub features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `icon.proto.core.connection` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xff, 0x1c, 0x0a, 0x23, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x30, 0x33, 0x2d, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x22, 0x86, 0x03, 0x0a, 0x0d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x64, 0x12, 0x3f, 0x0a, 0x08, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x08, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x12, 0x45, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x2f, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x2e, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x4c, 0x0a, 0x0c, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x28, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x63,
    0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x52, 0x0c, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x65, 0x6c,
    0x61, 0x79, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0b, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x22, 0x5f, 0x0a, 0x05,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x23, 0x0a, 0x1f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x55,
    0x4e, 0x49, 0x4e, 0x49, 0x54, 0x49, 0x41, 0x4c, 0x49, 0x5a, 0x45, 0x44, 0x5f, 0x55, 0x4e, 0x53,
    0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x53, 0x54,
    0x41, 0x54, 0x45, 0x5f, 0x49, 0x4e, 0x49, 0x54, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x0d, 0x53, 0x54,
    0x41, 0x54, 0x45, 0x5f, 0x54, 0x52, 0x59, 0x4f, 0x50, 0x45, 0x4e, 0x10, 0x02, 0x12, 0x0e, 0x0a,
    0x0a, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x50, 0x45, 0x4e, 0x10, 0x03, 0x22, 0x92, 0x01,
    0x0a, 0x0c, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x12, 0x1b,
    0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64,
    0x12, 0x40, 0x0a, 0x06, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x28, 0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x63, 0x6f,
    0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x4d, 0x65,
    0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x52, 0x06, 0x70, 0x72, 0x65, 0x66,
    0x69, 0x78, 0x22, 0x2d, 0x0a, 0x0c, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x65, 0x66,
    0x69, 0x78, 0x12, 0x1d, 0x0a, 0x0a, 0x6b, 0x65, 0x79, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x6b, 0x65, 0x79, 0x50, 0x72, 0x65, 0x66, 0x69,
    0x78, 0x22, 0x45, 0x0a, 0x07, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x0a,
    0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x1a, 0x0a, 0x08,
    0x66, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08,
    0x66, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x42, 0xdc, 0x01, 0x0a, 0x1e, 0x63, 0x6f, 0x6d,
    0x2e, 0x69, 0x63, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x63, 0x6f, 0x72, 0x65,
    0x2e, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x0f, 0x43, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x1d,
    0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x69, 0x65, 0x73, 0x2f, 0x67, 0x6f, 0x2f, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2f, 0x69, 0x63, 0x6f, 0x6e, 0x3b, 0x69, 0x63, 0x6f, 0x6e, 0xa2, 0x02, 0x04,
    0x49, 0x50, 0x43, 0x43, 0xaa, 0x02, 0x1a, 0x49, 0x63, 0x6f, 0x6e, 0x2e, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x43, 0x6f, 0x72, 0x65, 0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0xca, 0x02, 0x1a, 0x49, 0x63, 0x6f, 0x6e, 0x5c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x5c, 0x43,
    0x6f, 0x72, 0x65, 0x5c, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0xe2, 0x02,
    0x26, 0x49, 0x63, 0x6f, 0x6e, 0x5c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x5c, 0x43, 0x6f, 0x72, 0x65,
    0x5c, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5c, 0x47, 0x50, 0x42, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1d, 0x49, 0x63, 0x6f, 0x6e, 0x3a, 0x3a,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x43, 0x6f, 0x72, 0x65, 0x3a, 0x3a, 0x43, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4a, 0xc0, 0x15, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x4c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x01, 0x00, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x34,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x04, 0x00, 0x34, 0x0a, 0xde, 0x02, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x0d, 0x00, 0x2b, 0x01, 0x1a, 0xba, 0x01, 0x20, 0x43, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x6f, 0x74,
    0x68, 0x65, 0x72, 0x0a, 0x20, 0x73, 0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x6e,
    0x65, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x6d, 0x75, 0x73, 0x74, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x32, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x45, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x69,
    0x73, 0x68, 0x0a, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x63, 0x68, 0x61,
    0x69, 0x6e, 0x73, 0x2e, 0x0a, 0x32, 0x94, 0x01, 0x20, 0x49, 0x43, 0x53, 0x30, 0x33, 0x20, 0x2d,
    0x20, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x44, 0x61, 0x74, 0x61,
    0x20, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x73, 0x20, 0x61, 0x73, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73,
    0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f,
    0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63, 0x2f, 0x62, 0x6c, 0x6f, 0x62, 0x2f, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2f, 0x73, 0x70, 0x65, 0x63, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x69,
    0x63, 0x73, 0x2d, 0x30, 0x30, 0x33, 0x2d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2d, 0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x73, 0x23, 0x64, 0x61, 0x74, 0x61,
    0x2d, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x15, 0x0a, 0x77, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00,
    0x12, 0x04, 0x10, 0x04, 0x1c, 0x05, 0x1a, 0x69, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x6f, 0x6e, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e,
    0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x73, 0x3a, 0x0a, 0x20, 0x49, 0x4e, 0x49, 0x54, 0x2c,
    0x20, 0x54, 0x52, 0x59, 0x4f, 0x50, 0x45, 0x4e, 0x2c, 0x20, 0x4f, 0x50, 0x45, 0x4e, 0x20, 0x6f,
    0x72, 0x20, 0x55, 0x4e, 0x49, 0x4e, 0x49, 0x54, 0x49, 0x41, 0x4c, 0x49, 0x5a, 0x45, 0x44, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x10, 0x09, 0x0e, 0x0a,
    0x1e, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x08, 0x2c, 0x1a, 0x0f,
    0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x27, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x14, 0x2a, 0x2b, 0x0a,
    0x49, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x08, 0x17, 0x1a, 0x3a,
    0x20, 0x41, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x6e,
    0x64, 0x20, 0x68, 0x61, 0x73, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x68,
    0x61, 0x6e, 0x64, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x15, 0x16, 0x0a, 0x61, 0x0a, 0x06, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x19, 0x08, 0x1a, 0x1a, 0x52, 0x20, 0x41, 0x20, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x68, 0x61, 0x73,
    0x20, 0x61, 0x63, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65, 0x64, 0x67, 0x65, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x20, 0x73, 0x74, 0x65, 0x70,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x0a, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x15, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x19, 0x18, 0x19, 0x0a, 0x3e, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x08, 0x17, 0x1a, 0x2f, 0x20, 0x41,
    0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x20,
    0x68, 0x61, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x12, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1b, 0x15, 0x16, 0x0a, 0x36, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x04, 0x19, 0x1a, 0x29, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x1f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x0b,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x17, 0x18, 0x0a,
    0x88, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x04, 0x22, 0x1a, 0x7b, 0x20,
    0x49, 0x42, 0x43, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x73, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x65,
    0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x75,
    0x74, 0x69, 0x6c, 0x69, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x22, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x22, 0x15, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22,
    0x20, 0x21, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x14, 0x1a,
    0x26, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x65, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x24, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x24, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x12,
    0x13, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x26, 0x04, 0x22, 0x1a, 0x35,
    0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x20, 0x63, 0x68,
    0x61, 0x69, 0x6e, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x26, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x26, 0x11,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x26, 0x20, 0x21, 0x0a,
    0xa8, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x2a, 0x04, 0x1c, 0x1a, 0x9a, 0x01,
    0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x62, 0x65, 0x66,
    0x6f, 0x72, 0x65, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x2d, 0x76, 0x65,
    0x72, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a,
    0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x20, 0x6c, 0x6f,
    0x67, 0x69, 0x63, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x6d, 0x70, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x0a,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x2a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x2a, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x2a, 0x1a, 0x1b, 0x0a, 0x5b, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2e, 0x00, 0x39, 0x01,
    0x1a, 0x4f, 0x20, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x61,
    0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61,
    0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x14, 0x0a, 0x63, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x33, 0x04, 0x19, 0x1a, 0x56, 0x20, 0x69, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x73, 0x73,
    0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x67,
    0x69, 0x76, 0x65, 0x6e, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x33, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x0b, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x17, 0x18, 0x0a, 0x6b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x36, 0x04, 0x1d, 0x1a, 0x5e, 0x20, 0x69, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x20, 0x63, 0x68,
    0x61, 0x69, 0x6e, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x61, 0x0a, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x36, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x36, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x36, 0x1b, 0x1c, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x38, 0x04, 0x1c,
    0x1a, 0x35, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65,
    0x72, 0x6b, 0x6c, 0x65, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x79, 0x20,
    0x63, 0x68, 0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x38, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x38, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x1a,
    0x1b, 0x0a, 0xac, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x3e, 0x00, 0x40, 0x01, 0x1a, 0x9f,
    0x01, 0x20, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20, 0x69,
    0x73, 0x20, 0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x70, 0x72,
    0x65, 0x66, 0x69, 0x78, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65,
    0x79, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x50, 0x61, 0x74, 0x68, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65,
    0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x64,
    0x28, 0x50, 0x61, 0x74, 0x68, 0x2e, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x74, 0x68, 0x2c, 0x0a, 0x20,
    0x61, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x28, 0x50, 0x61, 0x74, 0x68, 0x2e, 0x4b, 0x65, 0x79, 0x50,
    0x72, 0x65, 0x66, 0x69, 0x78, 0x2c, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x2e, 0x2e, 0x29, 0x29, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x3f, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x3f, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x3f, 0x15, 0x16, 0x0a, 0x73, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x45, 0x00, 0x4c, 0x01, 0x1a,
    0x67, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x6e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x42,
    0x43, 0x20, 0x76, 0x65, 0x72, 0x69, 0x73, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x68, 0x61, 0x6e,
    0x64, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x45, 0x08, 0x0f, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x49, 0x04,
    0x1a, 0x1a, 0x1b, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x49, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x18, 0x19, 0x0a, 0x48, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x4b, 0x04, 0x21, 0x1a, 0x3b, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x66, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74, 0x69,
    0x62, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4b, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b, 0x1f, 0x20, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];
include!("icon.proto.core.connection.serde.rs");
// @@protoc_insertion_point(module)