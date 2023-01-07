pub use peer_broadcast_api::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod peer_broadcast_api {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{
            Abi, Detokenize, InvalidOutputType, Token,
            Tokenizable,
        },
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "PeerBroadcastAPI was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"peerMetadataIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"peerIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeerSet\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PEERBROADCASTAPI_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("invalid abi")
    });
    pub struct PeerBroadcastAPI<M>(
        ethers::contract::Contract<M>,
    );
    impl<M> Clone for PeerBroadcastAPI<M> {
        fn clone(&self) -> Self {
            PeerBroadcastAPI(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PeerBroadcastAPI<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PeerBroadcastAPI<M> {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            f.debug_tuple(stringify!(PeerBroadcastAPI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PeerBroadcastAPI<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                PEERBROADCASTAPI_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `registerPeer` (0x33e7fb45) function"]
        pub fn register_peer(
            &self,
            peer_ipfs_pointer: String,
        ) -> ethers::contract::builders::ContractCall<M, ()>
        {
            self.0
                .method_hash([51, 231, 251, 69], peer_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerPeerSet` (0x1f0baa6b) function"]
        pub fn register_peer_set(
            &self,
            peer_set_smart_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()>
        {
            self.0
                .method_hash([31, 11, 170, 107], peer_set_smart_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PeerRegistered` event"]
        pub fn peer_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PeerRegisteredFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetRegistered` event"]
        pub fn peer_set_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PeerSetRegisteredFilter,
        > {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PeerBroadcastAPIEvents,
        > {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware>
        From<ethers::contract::Contract<M>>
        for PeerBroadcastAPI<M>
    {
        fn from(
            contract: ethers::contract::Contract<M>,
        ) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "PeerRegistered",
        abi = "PeerRegistered(address,string)"
    )]
    pub struct PeerRegisteredFilter {
        pub peer_address: ethers::core::types::Address,
        pub peer_metadata_ipfs_pointer: String,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "PeerSetRegistered",
        abi = "PeerSetRegistered(address)"
    )]
    pub struct PeerSetRegisteredFilter {
        pub peer_set_smart_contract_address:
            ethers::core::types::Address,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
    )]
    pub enum PeerBroadcastAPIEvents {
        PeerRegisteredFilter(PeerRegisteredFilter),
        PeerSetRegisteredFilter(PeerSetRegisteredFilter),
    }
    impl ethers::contract::EthLogDecode for PeerBroadcastAPIEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) =
                PeerRegisteredFilter::decode_log(log)
            {
                return Ok(PeerBroadcastAPIEvents::PeerRegisteredFilter(decoded));
            }
            if let Ok(decoded) =
                PeerSetRegisteredFilter::decode_log(log)
            {
                return Ok(PeerBroadcastAPIEvents::PeerSetRegisteredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PeerBroadcastAPIEvents {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            match self {
                PeerBroadcastAPIEvents::PeerRegisteredFilter(element) => element.fmt(f),
                PeerBroadcastAPIEvents::PeerSetRegisteredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `registerPeer` function with signature `registerPeer(string)` and selector `0x33e7fb45`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "registerPeer",
        abi = "registerPeer(string)"
    )]
    pub struct RegisterPeerCall {
        pub peer_ipfs_pointer: String,
    }
    #[doc = "Container type for all input parameters for the `registerPeerSet` function with signature `registerPeerSet(address)` and selector `0x1f0baa6b`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "registerPeerSet",
        abi = "registerPeerSet(address)"
    )]
    pub struct RegisterPeerSetCall {
        pub peer_set_smart_contract:
            ethers::core::types::Address,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
    )]
    pub enum PeerBroadcastAPICalls {
        RegisterPeer(RegisterPeerCall),
        RegisterPeerSet(RegisterPeerSetCall),
    }
    impl ethers::core::abi::AbiDecode for PeerBroadcastAPICalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<
            Self,
            ethers::core::abi::AbiError,
        > {
            if let Ok(decoded) =
                <RegisterPeerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerBroadcastAPICalls::RegisterPeer(decoded));
            }
            if let Ok(decoded) =
                <RegisterPeerSetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerBroadcastAPICalls::RegisterPeerSet(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PeerBroadcastAPICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PeerBroadcastAPICalls::RegisterPeer(
                    element,
                ) => element.encode(),
                PeerBroadcastAPICalls::RegisterPeerSet(
                    element,
                ) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PeerBroadcastAPICalls {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            match self {
                PeerBroadcastAPICalls::RegisterPeer(
                    element,
                ) => element.fmt(f),
                PeerBroadcastAPICalls::RegisterPeerSet(
                    element,
                ) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<RegisterPeerCall>
        for PeerBroadcastAPICalls
    {
        fn from(var: RegisterPeerCall) -> Self {
            PeerBroadcastAPICalls::RegisterPeer(var)
        }
    }
    impl ::std::convert::From<RegisterPeerSetCall>
        for PeerBroadcastAPICalls
    {
        fn from(var: RegisterPeerSetCall) -> Self {
            PeerBroadcastAPICalls::RegisterPeerSet(var)
        }
    }
}
