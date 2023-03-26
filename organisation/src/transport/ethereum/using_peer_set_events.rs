pub use using_peer_set_events::*;
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
pub mod using_peer_set_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"rejectedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"updatedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"cid\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"vote\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphVoteReceived\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static USINGPEERSETEVENTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct UsingPeerSetEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UsingPeerSetEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UsingPeerSetEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UsingPeerSetEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UsingPeerSetEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(UsingPeerSetEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UsingPeerSetEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                USINGPEERSETEVENTS_ABI.clone(),
                client,
            ))
        }
        ///Gets the contract's `PeerSetPermissionGraphChangeRejected` event
        pub fn peer_set_permission_graph_change_rejected_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PeerSetPermissionGraphChangeRejectedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PeerSetPermissionGraphChangeRequest` event
        pub fn peer_set_permission_graph_change_request_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PeerSetPermissionGraphChangeRequestFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PeerSetPermissionGraphUpdated` event
        pub fn peer_set_permission_graph_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PeerSetPermissionGraphUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PeerSetPermissionGraphVoteReceived` event
        pub fn peer_set_permission_graph_vote_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PeerSetPermissionGraphVoteReceivedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UsingPeerSetEventsEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UsingPeerSetEvents<M>
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
    #[ethevent(
        name = "PeerSetPermissionGraphChangeRejected",
        abi = "PeerSetPermissionGraphChangeRejected(address,string)"
    )]
    pub struct PeerSetPermissionGraphChangeRejectedFilter {
        pub peer_requesting_change: ::ethers::core::types::Address,
        pub rejected_peer_set_permission_graph_ipfs_pointer: ::std::string::String,
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
    #[ethevent(
        name = "PeerSetPermissionGraphChangeRequest",
        abi = "PeerSetPermissionGraphChangeRequest(address,string)"
    )]
    pub struct PeerSetPermissionGraphChangeRequestFilter {
        pub peer_requesting_change: ::ethers::core::types::Address,
        pub proposed_peer_set_permission_graph_ipfs_pointer: ::std::string::String,
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
    #[ethevent(
        name = "PeerSetPermissionGraphUpdated",
        abi = "PeerSetPermissionGraphUpdated(address,string)"
    )]
    pub struct PeerSetPermissionGraphUpdatedFilter {
        pub peer_requesting_change: ::ethers::core::types::Address,
        pub updated_peer_set_permission_graph_ipfs_pointer: ::std::string::String,
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
    #[ethevent(
        name = "PeerSetPermissionGraphVoteReceived",
        abi = "PeerSetPermissionGraphVoteReceived(string,bool)"
    )]
    pub struct PeerSetPermissionGraphVoteReceivedFilter {
        pub cid: ::std::string::String,
        pub vote: bool,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UsingPeerSetEventsEvents {
        PeerSetPermissionGraphChangeRejectedFilter(PeerSetPermissionGraphChangeRejectedFilter),
        PeerSetPermissionGraphChangeRequestFilter(PeerSetPermissionGraphChangeRequestFilter),
        PeerSetPermissionGraphUpdatedFilter(PeerSetPermissionGraphUpdatedFilter),
        PeerSetPermissionGraphVoteReceivedFilter(PeerSetPermissionGraphVoteReceivedFilter),
    }
    impl ::ethers::contract::EthLogDecode for UsingPeerSetEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PeerSetPermissionGraphChangeRejectedFilter::decode_log(log) {
                return Ok(
                    UsingPeerSetEventsEvents::PeerSetPermissionGraphChangeRejectedFilter(decoded),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphChangeRequestFilter::decode_log(log) {
                return Ok(
                    UsingPeerSetEventsEvents::PeerSetPermissionGraphChangeRequestFilter(decoded),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphUpdatedFilter::decode_log(log) {
                return Ok(UsingPeerSetEventsEvents::PeerSetPermissionGraphUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PeerSetPermissionGraphVoteReceivedFilter::decode_log(log) {
                return Ok(
                    UsingPeerSetEventsEvents::PeerSetPermissionGraphVoteReceivedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UsingPeerSetEventsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PeerSetPermissionGraphChangeRejectedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PeerSetPermissionGraphChangeRequestFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PeerSetPermissionGraphUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PeerSetPermissionGraphVoteReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphChangeRejectedFilter>
        for UsingPeerSetEventsEvents
    {
        fn from(value: PeerSetPermissionGraphChangeRejectedFilter) -> Self {
            Self::PeerSetPermissionGraphChangeRejectedFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphChangeRequestFilter> for UsingPeerSetEventsEvents {
        fn from(value: PeerSetPermissionGraphChangeRequestFilter) -> Self {
            Self::PeerSetPermissionGraphChangeRequestFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphUpdatedFilter> for UsingPeerSetEventsEvents {
        fn from(value: PeerSetPermissionGraphUpdatedFilter) -> Self {
            Self::PeerSetPermissionGraphUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphVoteReceivedFilter> for UsingPeerSetEventsEvents {
        fn from(value: PeerSetPermissionGraphVoteReceivedFilter) -> Self {
            Self::PeerSetPermissionGraphVoteReceivedFilter(value)
        }
    }
}
