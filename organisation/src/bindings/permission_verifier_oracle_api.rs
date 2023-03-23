pub use permission_verifier_oracle_api::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod permission_verifier_oracle_api {
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
    #[doc = "PermissionVerifierOracleAPI was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeValidated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphValidationRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"result\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitPeerValidation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validatePermissionGraphChange\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PERMISSIONVERIFIERORACLEAPI_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PermissionVerifierOracleAPI<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PermissionVerifierOracleAPI<M> {
        fn clone(&self) -> Self {
            PermissionVerifierOracleAPI(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PermissionVerifierOracleAPI<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PermissionVerifierOracleAPI<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PermissionVerifierOracleAPI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PermissionVerifierOracleAPI<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                PERMISSIONVERIFIERORACLEAPI_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `submitPeerValidation` (0x0eb93125) function"]
        pub fn submit_peer_validation(
            &self,
            request_id: [u8; 32],
            result: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 185, 49, 37], (request_id, result))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validatePermissionGraphChange` (0x925b7ca5) function"]
        pub fn validate_permission_graph_change(
            &self,
            proposed_graph_ipfs_pointer: String,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([146, 91, 124, 165], proposed_graph_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PermissionGraphChangeValidated` event"]
        pub fn permission_graph_change_validated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PermissionGraphChangeValidatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PermissionGraphValidationRequested` event"]
        pub fn permission_graph_validation_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PermissionGraphValidationRequestedFilter>
        {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, PermissionVerifierOracleAPIEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PermissionVerifierOracleAPI<M>
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
    #[ethevent(
        name = "PermissionGraphChangeValidated",
        abi = "PermissionGraphChangeValidated(bytes32,bool)"
    )]
    pub struct PermissionGraphChangeValidatedFilter {
        pub request_id: [u8; 32],
        pub valid: bool,
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
        name = "PermissionGraphValidationRequested",
        abi = "PermissionGraphValidationRequested(bytes32,address,string)"
    )]
    pub struct PermissionGraphValidationRequestedFilter {
        pub request_id: [u8; 32],
        pub peer_set_smart_contract: ethers::core::types::Address,
        pub proposed_graph_ipfs_pointer: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PermissionVerifierOracleAPIEvents {
        PermissionGraphChangeValidatedFilter(PermissionGraphChangeValidatedFilter),
        PermissionGraphValidationRequestedFilter(PermissionGraphValidationRequestedFilter),
    }
    impl ethers::contract::EthLogDecode for PermissionVerifierOracleAPIEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PermissionGraphChangeValidatedFilter::decode_log(log) {
                return Ok(
                    PermissionVerifierOracleAPIEvents::PermissionGraphChangeValidatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PermissionGraphValidationRequestedFilter::decode_log(log) {
                return Ok(
                    PermissionVerifierOracleAPIEvents::PermissionGraphValidationRequestedFilter(
                        decoded,
                    ),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PermissionVerifierOracleAPIEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PermissionVerifierOracleAPIEvents::PermissionGraphChangeValidatedFilter(
                    element,
                ) => element.fmt(f),
                PermissionVerifierOracleAPIEvents::PermissionGraphValidationRequestedFilter(
                    element,
                ) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `submitPeerValidation` function with signature `submitPeerValidation(bytes32,bool)` and selector `0x0eb93125`"]
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
        name = "submitPeerValidation",
        abi = "submitPeerValidation(bytes32,bool)"
    )]
    pub struct SubmitPeerValidationCall {
        pub request_id: [u8; 32],
        pub result: bool,
    }
    #[doc = "Container type for all input parameters for the `validatePermissionGraphChange` function with signature `validatePermissionGraphChange(string)` and selector `0x925b7ca5`"]
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
        name = "validatePermissionGraphChange",
        abi = "validatePermissionGraphChange(string)"
    )]
    pub struct ValidatePermissionGraphChangeCall {
        pub proposed_graph_ipfs_pointer: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PermissionVerifierOracleAPICalls {
        SubmitPeerValidation(SubmitPeerValidationCall),
        ValidatePermissionGraphChange(ValidatePermissionGraphChangeCall),
    }
    impl ethers::core::abi::AbiDecode for PermissionVerifierOracleAPICalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SubmitPeerValidationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PermissionVerifierOracleAPICalls::SubmitPeerValidation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ValidatePermissionGraphChangeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    PermissionVerifierOracleAPICalls::ValidatePermissionGraphChange(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PermissionVerifierOracleAPICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PermissionVerifierOracleAPICalls::SubmitPeerValidation(element) => element.encode(),
                PermissionVerifierOracleAPICalls::ValidatePermissionGraphChange(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for PermissionVerifierOracleAPICalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PermissionVerifierOracleAPICalls::SubmitPeerValidation(element) => element.fmt(f),
                PermissionVerifierOracleAPICalls::ValidatePermissionGraphChange(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<SubmitPeerValidationCall> for PermissionVerifierOracleAPICalls {
        fn from(var: SubmitPeerValidationCall) -> Self {
            PermissionVerifierOracleAPICalls::SubmitPeerValidation(var)
        }
    }
    impl ::std::convert::From<ValidatePermissionGraphChangeCall> for PermissionVerifierOracleAPICalls {
        fn from(var: ValidatePermissionGraphChangeCall) -> Self {
            PermissionVerifierOracleAPICalls::ValidatePermissionGraphChange(var)
        }
    }
    #[doc = "Container type for all return fields from the `validatePermissionGraphChange` function with signature `validatePermissionGraphChange(string)` and selector `0x925b7ca5`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ValidatePermissionGraphChangeReturn(pub [u8; 32]);
}
