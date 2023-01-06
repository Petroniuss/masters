pub use using_peer_set_events::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod using_peer_set_events {
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
    #[doc = "UsingPeerSetEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"rejectedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"updatedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static USINGPEERSETEVENTS_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("invalid abi")
    });
    pub struct UsingPeerSetEvents<M>(
        ethers::contract::Contract<M>,
    );
    impl<M> Clone for UsingPeerSetEvents<M> {
        fn clone(&self) -> Self {
            UsingPeerSetEvents(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for UsingPeerSetEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for UsingPeerSetEvents<M> {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            f.debug_tuple(stringify!(UsingPeerSetEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> UsingPeerSetEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                USINGPEERSETEVENTS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Gets the contract's `PeerSetPermissionGraphChangeRejected` event"]
        pub fn peer_set_permission_graph_change_rejected_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PeerSetPermissionGraphChangeRejectedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetPermissionGraphChangeRequest` event"]
        pub fn peer_set_permission_graph_change_request_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PeerSetPermissionGraphChangeRequestFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetPermissionGraphUpdated` event"]
        pub fn peer_set_permission_graph_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PeerSetPermissionGraphUpdatedFilter,
        > {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            UsingPeerSetEventsEvents,
        > {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware>
        From<ethers::contract::Contract<M>>
        for UsingPeerSetEvents<M>
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
        name = "PeerSetPermissionGraphChangeRejected",
        abi = "PeerSetPermissionGraphChangeRejected(address,address,string)"
    )]
    pub struct PeerSetPermissionGraphChangeRejectedFilter {
        pub peer_requesting_change:
            ethers::core::types::Address,
        pub peer_validating_change:
            ethers::core::types::Address,
        pub rejected_peer_set_permission_graph_ipfs_pointer:
            String,
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
        name = "PeerSetPermissionGraphChangeRequest",
        abi = "PeerSetPermissionGraphChangeRequest(address,string)"
    )]
    pub struct PeerSetPermissionGraphChangeRequestFilter {
        pub peer_requesting_change:
            ethers::core::types::Address,
        pub proposed_peer_set_permission_graph_ipfs_pointer:
            String,
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
        name = "PeerSetPermissionGraphUpdated",
        abi = "PeerSetPermissionGraphUpdated(address,address,string)"
    )]
    pub struct PeerSetPermissionGraphUpdatedFilter {
        pub peer_requesting_change:
            ethers::core::types::Address,
        pub peer_validating_change:
            ethers::core::types::Address,
        pub updated_peer_set_permission_graph_ipfs_pointer:
            String,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
    )]
    pub enum UsingPeerSetEventsEvents {
        PeerSetPermissionGraphChangeRejectedFilter(
            PeerSetPermissionGraphChangeRejectedFilter,
        ),
        PeerSetPermissionGraphChangeRequestFilter(
            PeerSetPermissionGraphChangeRequestFilter,
        ),
        PeerSetPermissionGraphUpdatedFilter(
            PeerSetPermissionGraphUpdatedFilter,
        ),
    }
    impl ethers::contract::EthLogDecode
        for UsingPeerSetEventsEvents
    {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) =
                PeerSetPermissionGraphChangeRejectedFilter::decode_log(log)
            {
                return Ok(
                    UsingPeerSetEventsEvents::PeerSetPermissionGraphChangeRejectedFilter(decoded),
                );
            }
            if let Ok(decoded) =
                PeerSetPermissionGraphChangeRequestFilter::decode_log(log)
            {
                return Ok(
                    UsingPeerSetEventsEvents::PeerSetPermissionGraphChangeRequestFilter(decoded),
                );
            }
            if let Ok(decoded) =
                PeerSetPermissionGraphUpdatedFilter::decode_log(
                    log,
                )
            {
                return Ok(UsingPeerSetEventsEvents::PeerSetPermissionGraphUpdatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UsingPeerSetEventsEvents {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            match self {
                UsingPeerSetEventsEvents::PeerSetPermissionGraphChangeRejectedFilter(element) => {
                    element.fmt(f)
                }
                UsingPeerSetEventsEvents::PeerSetPermissionGraphChangeRequestFilter(element) => {
                    element.fmt(f)
                }
                UsingPeerSetEventsEvents::PeerSetPermissionGraphUpdatedFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
}
