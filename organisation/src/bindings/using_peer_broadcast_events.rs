pub use using_peer_broadcast_events::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod using_peer_broadcast_events {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "UsingPeerBroadcastEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"peerMetadataIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetRegistered\",\"outputs\":[],\"anonymous\":false}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static USINGPEERBROADCASTEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct UsingPeerBroadcastEvents<M>(ethers::contract::Contract<M>);
    impl<M> Clone for UsingPeerBroadcastEvents<M> {
        fn clone(&self) -> Self {
            UsingPeerBroadcastEvents(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for UsingPeerBroadcastEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for UsingPeerBroadcastEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UsingPeerBroadcastEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> UsingPeerBroadcastEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                USINGPEERBROADCASTEVENTS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Gets the contract's `PeerRegistered` event"]
        pub fn peer_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetRegistered` event"]
        pub fn peer_set_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerSetRegisteredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, UsingPeerBroadcastEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for UsingPeerBroadcastEvents<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
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
    #[ethevent(name = "PeerRegistered", abi = "PeerRegistered(address,string)")]
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
    #[ethevent(name = "PeerSetRegistered", abi = "PeerSetRegistered(address)")]
    pub struct PeerSetRegisteredFilter {
        pub peer_set_smart_contract_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UsingPeerBroadcastEventsEvents {
        PeerRegisteredFilter(PeerRegisteredFilter),
        PeerSetRegisteredFilter(PeerSetRegisteredFilter),
    }
    impl ethers::contract::EthLogDecode for UsingPeerBroadcastEventsEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PeerRegisteredFilter::decode_log(log) {
                return Ok(UsingPeerBroadcastEventsEvents::PeerRegisteredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PeerSetRegisteredFilter::decode_log(log) {
                return Ok(UsingPeerBroadcastEventsEvents::PeerSetRegisteredFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UsingPeerBroadcastEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UsingPeerBroadcastEventsEvents::PeerRegisteredFilter(element) => element.fmt(f),
                UsingPeerBroadcastEventsEvents::PeerSetRegisteredFilter(element) => element.fmt(f),
            }
        }
    }
}
