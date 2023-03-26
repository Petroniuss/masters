pub use peer_set_smart_contract_api::*;
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
pub mod peer_set_smart_contract_api {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"rejectedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"updatedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"cid\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"vote\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphVoteReceived\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentPeerSetPermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPeer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePermissionGraphChange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"cid\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"vote\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitPeerVote\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PEERSETSMARTCONTRACTAPI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct PeerSetSmartContractAPI<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PeerSetSmartContractAPI<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PeerSetSmartContractAPI<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PeerSetSmartContractAPI<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PeerSetSmartContractAPI<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PeerSetSmartContractAPI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PeerSetSmartContractAPI<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PEERSETSMARTCONTRACTAPI_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `currentPeerSetPermissionGraphIPFSPointer` (0xb4dc5765) function
        pub fn current_peer_set_permission_graph_ipfs_pointer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([180, 220, 87, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPeer` (0x3e44cf78) function
        pub fn is_peer(
            &self,
            peer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 68, 207, 120], peer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposePermissionGraphChange` (0x6c986d7c) function
        pub fn propose_permission_graph_change(
            &self,
            proposed_graph_ipfs_pointer: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 152, 109, 124], proposed_graph_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitPeerVote` (0x281e8729) function
        pub fn submit_peer_vote(
            &self,
            cid: ::std::string::String,
            vote: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 30, 135, 41], (cid, vote))
                .expect("method not found (this should never happen)")
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PeerSetSmartContractAPIEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PeerSetSmartContractAPI<M>
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
    pub enum PeerSetSmartContractAPIEvents {
        PeerSetPermissionGraphChangeRejectedFilter(PeerSetPermissionGraphChangeRejectedFilter),
        PeerSetPermissionGraphChangeRequestFilter(PeerSetPermissionGraphChangeRequestFilter),
        PeerSetPermissionGraphUpdatedFilter(PeerSetPermissionGraphUpdatedFilter),
        PeerSetPermissionGraphVoteReceivedFilter(PeerSetPermissionGraphVoteReceivedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PeerSetSmartContractAPIEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PeerSetPermissionGraphChangeRejectedFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphChangeRejectedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphChangeRequestFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphChangeRequestFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphUpdatedFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphVoteReceivedFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphVoteReceivedFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PeerSetSmartContractAPIEvents {
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
        for PeerSetSmartContractAPIEvents
    {
        fn from(value: PeerSetPermissionGraphChangeRejectedFilter) -> Self {
            Self::PeerSetPermissionGraphChangeRejectedFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphChangeRequestFilter>
        for PeerSetSmartContractAPIEvents
    {
        fn from(value: PeerSetPermissionGraphChangeRequestFilter) -> Self {
            Self::PeerSetPermissionGraphChangeRequestFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphUpdatedFilter> for PeerSetSmartContractAPIEvents {
        fn from(value: PeerSetPermissionGraphUpdatedFilter) -> Self {
            Self::PeerSetPermissionGraphUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetPermissionGraphVoteReceivedFilter>
        for PeerSetSmartContractAPIEvents
    {
        fn from(value: PeerSetPermissionGraphVoteReceivedFilter) -> Self {
            Self::PeerSetPermissionGraphVoteReceivedFilter(value)
        }
    }
    ///Container type for all input parameters for the `currentPeerSetPermissionGraphIPFSPointer` function with signature `currentPeerSetPermissionGraphIPFSPointer()` and selector `0xb4dc5765`
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
    #[ethcall(
        name = "currentPeerSetPermissionGraphIPFSPointer",
        abi = "currentPeerSetPermissionGraphIPFSPointer()"
    )]
    pub struct CurrentPeerSetPermissionGraphIPFSPointerCall;
    ///Container type for all input parameters for the `isPeer` function with signature `isPeer(address)` and selector `0x3e44cf78`
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
    #[ethcall(name = "isPeer", abi = "isPeer(address)")]
    pub struct IsPeerCall {
        pub peer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `proposePermissionGraphChange` function with signature `proposePermissionGraphChange(string)` and selector `0x6c986d7c`
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
    #[ethcall(
        name = "proposePermissionGraphChange",
        abi = "proposePermissionGraphChange(string)"
    )]
    pub struct ProposePermissionGraphChangeCall {
        pub proposed_graph_ipfs_pointer: ::std::string::String,
    }
    ///Container type for all input parameters for the `submitPeerVote` function with signature `submitPeerVote(string,bool)` and selector `0x281e8729`
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
    #[ethcall(name = "submitPeerVote", abi = "submitPeerVote(string,bool)")]
    pub struct SubmitPeerVoteCall {
        pub cid: ::std::string::String,
        pub vote: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PeerSetSmartContractAPICalls {
        CurrentPeerSetPermissionGraphIPFSPointer(CurrentPeerSetPermissionGraphIPFSPointerCall),
        IsPeer(IsPeerCall),
        ProposePermissionGraphChange(ProposePermissionGraphChangeCall),
        SubmitPeerVote(SubmitPeerVoteCall),
    }
    impl ::ethers::core::abi::AbiDecode for PeerSetSmartContractAPICalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CurrentPeerSetPermissionGraphIPFSPointerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentPeerSetPermissionGraphIPFSPointer(decoded));
            }
            if let Ok(decoded) = <IsPeerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPeer(decoded));
            }
            if let Ok(decoded) =
                <ProposePermissionGraphChangeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProposePermissionGraphChange(decoded));
            }
            if let Ok(decoded) =
                <SubmitPeerVoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitPeerVote(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PeerSetSmartContractAPICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CurrentPeerSetPermissionGraphIPFSPointer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPeer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProposePermissionGraphChange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitPeerVote(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PeerSetSmartContractAPICalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CurrentPeerSetPermissionGraphIPFSPointer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsPeer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposePermissionGraphChange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitPeerVote(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CurrentPeerSetPermissionGraphIPFSPointerCall>
        for PeerSetSmartContractAPICalls
    {
        fn from(value: CurrentPeerSetPermissionGraphIPFSPointerCall) -> Self {
            Self::CurrentPeerSetPermissionGraphIPFSPointer(value)
        }
    }
    impl ::core::convert::From<IsPeerCall> for PeerSetSmartContractAPICalls {
        fn from(value: IsPeerCall) -> Self {
            Self::IsPeer(value)
        }
    }
    impl ::core::convert::From<ProposePermissionGraphChangeCall> for PeerSetSmartContractAPICalls {
        fn from(value: ProposePermissionGraphChangeCall) -> Self {
            Self::ProposePermissionGraphChange(value)
        }
    }
    impl ::core::convert::From<SubmitPeerVoteCall> for PeerSetSmartContractAPICalls {
        fn from(value: SubmitPeerVoteCall) -> Self {
            Self::SubmitPeerVote(value)
        }
    }
    ///Container type for all return fields from the `currentPeerSetPermissionGraphIPFSPointer` function with signature `currentPeerSetPermissionGraphIPFSPointer()` and selector `0xb4dc5765`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CurrentPeerSetPermissionGraphIPFSPointerReturn(pub ::std::string::String);
    ///Container type for all return fields from the `isPeer` function with signature `isPeer(address)` and selector `0x3e44cf78`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsPeerReturn(pub bool);
}
