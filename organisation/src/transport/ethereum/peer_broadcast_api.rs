pub use peer_broadcast_api::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod peer_broadcast_api {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"peerMetadataIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"peerIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeerSet\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PEERBROADCASTAPI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct PeerBroadcastAPI<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PeerBroadcastAPI<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PeerBroadcastAPI<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PeerBroadcastAPI<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PeerBroadcastAPI<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PeerBroadcastAPI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PeerBroadcastAPI<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PEERBROADCASTAPI_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `registerPeer` (0x33e7fb45) function
        pub fn register_peer(
            &self,
            peer_ipfs_pointer: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 231, 251, 69], peer_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPeerSet` (0x1f0baa6b) function
        pub fn register_peer_set(
            &self,
            peer_set_smart_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 11, 170, 107], peer_set_smart_contract)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PeerRegistered` event
        pub fn peer_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PeerRegisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PeerSetRegistered` event
        pub fn peer_set_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PeerSetRegisteredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PeerBroadcastAPIEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PeerBroadcastAPI<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PeerRegistered", abi = "PeerRegistered(address,string)")]
    pub struct PeerRegisteredFilter {
        pub peer_address: ::ethers::core::types::Address,
        pub peer_metadata_ipfs_pointer: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PeerSetRegistered", abi = "PeerSetRegistered(address)")]
    pub struct PeerSetRegisteredFilter {
        pub peer_set_smart_contract_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PeerBroadcastAPIEvents {
        PeerRegisteredFilter(PeerRegisteredFilter),
        PeerSetRegisteredFilter(PeerSetRegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for PeerBroadcastAPIEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PeerRegisteredFilter::decode_log(log) {
                return Ok(PeerBroadcastAPIEvents::PeerRegisteredFilter(decoded));
            }
            if let Ok(decoded) = PeerSetRegisteredFilter::decode_log(log) {
                return Ok(PeerBroadcastAPIEvents::PeerSetRegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PeerBroadcastAPIEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PeerRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerSetRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PeerRegisteredFilter> for PeerBroadcastAPIEvents {
        fn from(value: PeerRegisteredFilter) -> Self {
            Self::PeerRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetRegisteredFilter> for PeerBroadcastAPIEvents {
        fn from(value: PeerSetRegisteredFilter) -> Self {
            Self::PeerSetRegisteredFilter(value)
        }
    }
    ///Container type for all input parameters for the `registerPeer` function with signature `registerPeer(string)` and selector `0x33e7fb45`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "registerPeer", abi = "registerPeer(string)")]
    pub struct RegisterPeerCall {
        pub peer_ipfs_pointer: ::std::string::String,
    }
    ///Container type for all input parameters for the `registerPeerSet` function with signature `registerPeerSet(address)` and selector `0x1f0baa6b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "registerPeerSet", abi = "registerPeerSet(address)")]
    pub struct RegisterPeerSetCall {
        pub peer_set_smart_contract: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PeerBroadcastAPICalls {
        RegisterPeer(RegisterPeerCall),
        RegisterPeerSet(RegisterPeerSetCall),
    }
    impl ::ethers::core::abi::AbiDecode for PeerBroadcastAPICalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RegisterPeerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterPeer(decoded));
            }
            if let Ok(decoded) =
                <RegisterPeerSetCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterPeerSet(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PeerBroadcastAPICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RegisterPeer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterPeerSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PeerBroadcastAPICalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RegisterPeer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterPeerSet(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RegisterPeerCall> for PeerBroadcastAPICalls {
        fn from(value: RegisterPeerCall) -> Self {
            Self::RegisterPeer(value)
        }
    }
    impl ::core::convert::From<RegisterPeerSetCall> for PeerBroadcastAPICalls {
        fn from(value: RegisterPeerSetCall) -> Self {
            Self::RegisterPeerSet(value)
        }
    }
}
