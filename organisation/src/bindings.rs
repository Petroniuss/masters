pub use permission_graph::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod permission_graph {
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
    #[doc = "PermissionGraph was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"organisationName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"_PermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"organisationName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"PermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"organisationName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_PermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePermissionGraphChange\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestPermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PERMISSIONGRAPH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PermissionGraph<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PermissionGraph<M> {
        fn clone(&self) -> Self {
            PermissionGraph(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PermissionGraph<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PermissionGraph<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PermissionGraph))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PermissionGraph<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PERMISSIONGRAPH_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `PermissionGraphIPFSPointer` (0xecaa133f) function"]
        pub fn permission_graph_ipfs_pointer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([236, 170, 19, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestPermissionGraphIPFSPointer` (0x20755771) function"]
        pub fn get_latest_permission_graph_ipfs_pointer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([32, 117, 87, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposePermissionGraphChange` (0x9f9fe1ba) function"]
        pub fn propose_permission_graph_change(
            &self,
            organisation_name: String,
            permission_graph_ipfs_pointer: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [159, 159, 225, 186],
                    (organisation_name, permission_graph_ipfs_pointer),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PermissionGraphChangeRequest` event"]
        pub fn permission_graph_change_request_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PermissionGraphChangeRequestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PermissionGraphUpdated` event"]
        pub fn permission_graph_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PermissionGraphUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PermissionGraphEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PermissionGraph<M> {
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
    #[ethevent(
        name = "PermissionGraphChangeRequest",
        abi = "PermissionGraphChangeRequest(string,string)"
    )]
    pub struct PermissionGraphChangeRequestFilter {
        pub organisation_name: String,
        pub permission_graph_ipfs_pointer: String,
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
        name = "PermissionGraphUpdated",
        abi = "PermissionGraphUpdated(string,string)"
    )]
    pub struct PermissionGraphUpdatedFilter {
        pub organisation_name: String,
        pub permission_graph_ipfs_pointer: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PermissionGraphEvents {
        PermissionGraphChangeRequestFilter(PermissionGraphChangeRequestFilter),
        PermissionGraphUpdatedFilter(PermissionGraphUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for PermissionGraphEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PermissionGraphChangeRequestFilter::decode_log(log) {
                return Ok(PermissionGraphEvents::PermissionGraphChangeRequestFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PermissionGraphUpdatedFilter::decode_log(log) {
                return Ok(PermissionGraphEvents::PermissionGraphUpdatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PermissionGraphEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PermissionGraphEvents::PermissionGraphChangeRequestFilter(element) => {
                    element.fmt(f)
                }
                PermissionGraphEvents::PermissionGraphUpdatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `PermissionGraphIPFSPointer` function with signature `PermissionGraphIPFSPointer()` and selector `[236, 170, 19, 63]`"]
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
        name = "PermissionGraphIPFSPointer",
        abi = "PermissionGraphIPFSPointer()"
    )]
    pub struct PermissionGraphIPFSPointerCall;
    #[doc = "Container type for all input parameters for the `getLatestPermissionGraphIPFSPointer` function with signature `getLatestPermissionGraphIPFSPointer()` and selector `[32, 117, 87, 113]`"]
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
        name = "getLatestPermissionGraphIPFSPointer",
        abi = "getLatestPermissionGraphIPFSPointer()"
    )]
    pub struct GetLatestPermissionGraphIPFSPointerCall;
    #[doc = "Container type for all input parameters for the `proposePermissionGraphChange` function with signature `proposePermissionGraphChange(string,string)` and selector `[159, 159, 225, 186]`"]
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
        abi = "proposePermissionGraphChange(string,string)"
    )]
    pub struct ProposePermissionGraphChangeCall {
        pub organisation_name: String,
        pub permission_graph_ipfs_pointer: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PermissionGraphCalls {
        PermissionGraphIPFSPointer(PermissionGraphIPFSPointerCall),
        GetLatestPermissionGraphIPFSPointer(GetLatestPermissionGraphIPFSPointerCall),
        ProposePermissionGraphChange(ProposePermissionGraphChangeCall),
    }
    impl ethers::core::abi::AbiDecode for PermissionGraphCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PermissionGraphIPFSPointerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PermissionGraphCalls::PermissionGraphIPFSPointer(decoded));
            }
            if let Ok(decoded) =
                <GetLatestPermissionGraphIPFSPointerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PermissionGraphCalls::GetLatestPermissionGraphIPFSPointer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ProposePermissionGraphChangeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PermissionGraphCalls::ProposePermissionGraphChange(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PermissionGraphCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PermissionGraphCalls::PermissionGraphIPFSPointer(element) => element.encode(),
                PermissionGraphCalls::GetLatestPermissionGraphIPFSPointer(element) => {
                    element.encode()
                }
                PermissionGraphCalls::ProposePermissionGraphChange(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PermissionGraphCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PermissionGraphCalls::PermissionGraphIPFSPointer(element) => element.fmt(f),
                PermissionGraphCalls::GetLatestPermissionGraphIPFSPointer(element) => {
                    element.fmt(f)
                }
                PermissionGraphCalls::ProposePermissionGraphChange(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PermissionGraphIPFSPointerCall> for PermissionGraphCalls {
        fn from(var: PermissionGraphIPFSPointerCall) -> Self {
            PermissionGraphCalls::PermissionGraphIPFSPointer(var)
        }
    }
    impl ::std::convert::From<GetLatestPermissionGraphIPFSPointerCall> for PermissionGraphCalls {
        fn from(var: GetLatestPermissionGraphIPFSPointerCall) -> Self {
            PermissionGraphCalls::GetLatestPermissionGraphIPFSPointer(var)
        }
    }
    impl ::std::convert::From<ProposePermissionGraphChangeCall> for PermissionGraphCalls {
        fn from(var: ProposePermissionGraphChangeCall) -> Self {
            PermissionGraphCalls::ProposePermissionGraphChange(var)
        }
    }
    #[doc = "Container type for all return fields from the `PermissionGraphIPFSPointer` function with signature `PermissionGraphIPFSPointer()` and selector `[236, 170, 19, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PermissionGraphIPFSPointerReturn(pub String);
    #[doc = "Container type for all return fields from the `getLatestPermissionGraphIPFSPointer` function with signature `getLatestPermissionGraphIPFSPointer()` and selector `[32, 117, 87, 113]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLatestPermissionGraphIPFSPointerReturn(pub String);
}
