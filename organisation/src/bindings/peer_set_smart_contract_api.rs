pub use peer_set_smart_contract_api::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod peer_set_smart_contract_api {
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
    #[doc = "PeerSetSmartContractAPI was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"rejectedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"updatedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"result\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__callback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPeer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestPeerSetPermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePermissionGraphChange\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PEERSETSMARTCONTRACTAPI_ABI:
        ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI)
                .expect("invalid abi")
        });
    pub struct PeerSetSmartContractAPI<M>(
        ethers::contract::Contract<M>,
    );
    impl<M> Clone for PeerSetSmartContractAPI<M> {
        fn clone(&self) -> Self {
            PeerSetSmartContractAPI(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PeerSetSmartContractAPI<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PeerSetSmartContractAPI<M> {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            f.debug_tuple(stringify!(PeerSetSmartContractAPI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware>
        PeerSetSmartContractAPI<M>
    {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                PEERSETSMARTCONTRACTAPI_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `__callback` (0x240cede1) function"]
        pub fn callback(
            &self,
            request_id: [u8; 32],
            result: bool,
            peer_validating_change: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()>
        {
            self.0
                .method_hash(
                    [36, 12, 237, 225],
                    (request_id, result, peer_validating_change),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPeer` (0x3e44cf78) function"]
        pub fn is_peer(
            &self,
            peer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool>
        {
            self.0.method_hash([62, 68, 207, 120], peer).expect(
                "method not found (this should never happen)",
            )
        }
        #[doc = "Calls the contract's `latestPeerSetPermissionGraphIPFSPointer` (0xfeceb2cb) function"]
        pub fn latest_peer_set_permission_graph_ipfs_pointer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String>
        {
            self.0.method_hash([254, 206, 178, 203], ()).expect(
                "method not found (this should never happen)",
            )
        }
        #[doc = "Calls the contract's `proposePermissionGraphChange` (0x6c986d7c) function"]
        pub fn propose_permission_graph_change(
            &self,
            proposed_graph_ipfs_pointer: String,
        ) -> ethers::contract::builders::ContractCall<M, ()>
        {
            self.0
                .method_hash([108, 152, 109, 124], proposed_graph_ipfs_pointer)
                .expect("method not found (this should never happen)")
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
            PeerSetSmartContractAPIEvents,
        > {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware>
        From<ethers::contract::Contract<M>>
        for PeerSetSmartContractAPI<M>
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
    pub enum PeerSetSmartContractAPIEvents {
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
        for PeerSetSmartContractAPIEvents
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
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphChangeRejectedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                PeerSetPermissionGraphChangeRequestFilter::decode_log(log)
            {
                return Ok(
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphChangeRequestFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                PeerSetPermissionGraphUpdatedFilter::decode_log(
                    log,
                )
            {
                return Ok(
                    PeerSetSmartContractAPIEvents::PeerSetPermissionGraphUpdatedFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PeerSetSmartContractAPIEvents {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            match self {
                PeerSetSmartContractAPIEvents::PeerSetPermissionGraphChangeRejectedFilter(
                    element,
                ) => element.fmt(f),
                PeerSetSmartContractAPIEvents::PeerSetPermissionGraphChangeRequestFilter(
                    element,
                ) => element.fmt(f),
                PeerSetSmartContractAPIEvents::PeerSetPermissionGraphUpdatedFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `__callback` function with signature `__callback(bytes32,bool,address)` and selector `0x240cede1`"]
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
        name = "__callback",
        abi = "__callback(bytes32,bool,address)"
    )]
    pub struct CallbackCall {
        pub request_id: [u8; 32],
        pub result: bool,
        pub peer_validating_change:
            ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isPeer` function with signature `isPeer(address)` and selector `0x3e44cf78`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isPeer", abi = "isPeer(address)")]
    pub struct IsPeerCall {
        pub peer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `latestPeerSetPermissionGraphIPFSPointer` function with signature `latestPeerSetPermissionGraphIPFSPointer()` and selector `0xfeceb2cb`"]
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
        name = "latestPeerSetPermissionGraphIPFSPointer",
        abi = "latestPeerSetPermissionGraphIPFSPointer()"
    )]
    pub struct LatestPeerSetPermissionGraphIPFSPointerCall;
    #[doc = "Container type for all input parameters for the `proposePermissionGraphChange` function with signature `proposePermissionGraphChange(string)` and selector `0x6c986d7c`"]
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
        name = "proposePermissionGraphChange",
        abi = "proposePermissionGraphChange(string)"
    )]
    pub struct ProposePermissionGraphChangeCall {
        pub proposed_graph_ipfs_pointer: String,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
    )]
    pub enum PeerSetSmartContractAPICalls {
        Callback(CallbackCall),
        IsPeer(IsPeerCall),
        LatestPeerSetPermissionGraphIPFSPointer(
            LatestPeerSetPermissionGraphIPFSPointerCall,
        ),
        ProposePermissionGraphChange(
            ProposePermissionGraphChangeCall,
        ),
    }
    impl ethers::core::abi::AbiDecode
        for PeerSetSmartContractAPICalls
    {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<
            Self,
            ethers::core::abi::AbiError,
        > {
            if let Ok(decoded) =
                <CallbackCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeerSetSmartContractAPICalls::Callback(decoded));
            }
            if let Ok(decoded) =
                <IsPeerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeerSetSmartContractAPICalls::IsPeer(decoded));
            }
            if let Ok (decoded) = < LatestPeerSetPermissionGraphIPFSPointerCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PeerSetSmartContractAPICalls :: LatestPeerSetPermissionGraphIPFSPointer (decoded)) }
            if let Ok(decoded) =
                <ProposePermissionGraphChangeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeerSetSmartContractAPICalls::ProposePermissionGraphChange(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode
        for PeerSetSmartContractAPICalls
    {
        fn encode(self) -> Vec<u8> {
            match self {
                PeerSetSmartContractAPICalls::Callback(element) => element.encode(),
                PeerSetSmartContractAPICalls::IsPeer(element) => element.encode(),
                PeerSetSmartContractAPICalls::LatestPeerSetPermissionGraphIPFSPointer(element) => {
                    element.encode()
                }
                PeerSetSmartContractAPICalls::ProposePermissionGraphChange(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for PeerSetSmartContractAPICalls {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            match self {
                PeerSetSmartContractAPICalls::Callback(element) => element.fmt(f),
                PeerSetSmartContractAPICalls::IsPeer(element) => element.fmt(f),
                PeerSetSmartContractAPICalls::LatestPeerSetPermissionGraphIPFSPointer(element) => {
                    element.fmt(f)
                }
                PeerSetSmartContractAPICalls::ProposePermissionGraphChange(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<CallbackCall>
        for PeerSetSmartContractAPICalls
    {
        fn from(var: CallbackCall) -> Self {
            PeerSetSmartContractAPICalls::Callback(var)
        }
    }
    impl ::std::convert::From<IsPeerCall>
        for PeerSetSmartContractAPICalls
    {
        fn from(var: IsPeerCall) -> Self {
            PeerSetSmartContractAPICalls::IsPeer(var)
        }
    }
    impl
        ::std::convert::From<
            LatestPeerSetPermissionGraphIPFSPointerCall,
        > for PeerSetSmartContractAPICalls
    {
        fn from(
            var: LatestPeerSetPermissionGraphIPFSPointerCall,
        ) -> Self {
            PeerSetSmartContractAPICalls::LatestPeerSetPermissionGraphIPFSPointer(var)
        }
    }
    impl ::std::convert::From<ProposePermissionGraphChangeCall>
        for PeerSetSmartContractAPICalls
    {
        fn from(var: ProposePermissionGraphChangeCall) -> Self {
            PeerSetSmartContractAPICalls::ProposePermissionGraphChange(var)
        }
    }
    #[doc = "Container type for all return fields from the `isPeer` function with signature `isPeer(address)` and selector `0x3e44cf78`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsPeerReturn(pub bool);
    #[doc = "Container type for all return fields from the `latestPeerSetPermissionGraphIPFSPointer` function with signature `latestPeerSetPermissionGraphIPFSPointer()` and selector `0xfeceb2cb`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LatestPeerSetPermissionGraphIPFSPointerReturn(
        pub String,
    );
}
