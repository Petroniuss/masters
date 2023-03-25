pub use permission_verifier_oracle_api::*;
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
pub mod permission_verifier_oracle_api {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeValidated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphValidationRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"result\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitPeerValidation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validatePermissionGraphChange\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static PERMISSIONVERIFIERORACLEAPI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct PermissionVerifierOracleAPI<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PermissionVerifierOracleAPI<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PermissionVerifierOracleAPI<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PermissionVerifierOracleAPI<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PermissionVerifierOracleAPI<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PermissionVerifierOracleAPI))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PermissionVerifierOracleAPI<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PERMISSIONVERIFIERORACLEAPI_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `submitPeerValidation` (0x0eb93125) function
        pub fn submit_peer_validation(
            &self,
            request_id: [u8; 32],
            result: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 185, 49, 37], (request_id, result))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatePermissionGraphChange` (0x925b7ca5) function
        pub fn validate_permission_graph_change(
            &self,
            proposed_graph_ipfs_pointer: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([146, 91, 124, 165], proposed_graph_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PermissionGraphChangeValidated` event
        pub fn permission_graph_change_validated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermissionGraphChangeValidatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PermissionGraphValidationRequested` event
        pub fn permission_graph_validation_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermissionGraphValidationRequestedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PermissionVerifierOracleAPIEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PermissionVerifierOracleAPI<M>
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
        name = "PermissionGraphChangeValidated",
        abi = "PermissionGraphChangeValidated(bytes32,bool)"
    )]
    pub struct PermissionGraphChangeValidatedFilter {
        pub request_id: [u8; 32],
        pub valid: bool,
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
        name = "PermissionGraphValidationRequested",
        abi = "PermissionGraphValidationRequested(bytes32,address,string)"
    )]
    pub struct PermissionGraphValidationRequestedFilter {
        pub request_id: [u8; 32],
        pub peer_set_smart_contract: ::ethers::core::types::Address,
        pub proposed_graph_ipfs_pointer: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PermissionVerifierOracleAPIEvents {
        PermissionGraphChangeValidatedFilter(PermissionGraphChangeValidatedFilter),
        PermissionGraphValidationRequestedFilter(PermissionGraphValidationRequestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PermissionVerifierOracleAPIEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
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
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PermissionVerifierOracleAPIEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PermissionGraphChangeValidatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermissionGraphValidationRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PermissionGraphChangeValidatedFilter>
        for PermissionVerifierOracleAPIEvents
    {
        fn from(value: PermissionGraphChangeValidatedFilter) -> Self {
            Self::PermissionGraphChangeValidatedFilter(value)
        }
    }
    impl ::core::convert::From<PermissionGraphValidationRequestedFilter>
        for PermissionVerifierOracleAPIEvents
    {
        fn from(value: PermissionGraphValidationRequestedFilter) -> Self {
            Self::PermissionGraphValidationRequestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `submitPeerValidation` function with signature `submitPeerValidation(bytes32,bool)` and selector `0x0eb93125`
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
        name = "submitPeerValidation",
        abi = "submitPeerValidation(bytes32,bool)"
    )]
    pub struct SubmitPeerValidationCall {
        pub request_id: [u8; 32],
        pub result: bool,
    }
    ///Container type for all input parameters for the `validatePermissionGraphChange` function with signature `validatePermissionGraphChange(string)` and selector `0x925b7ca5`
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
        name = "validatePermissionGraphChange",
        abi = "validatePermissionGraphChange(string)"
    )]
    pub struct ValidatePermissionGraphChangeCall {
        pub proposed_graph_ipfs_pointer: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PermissionVerifierOracleAPICalls {
        SubmitPeerValidation(SubmitPeerValidationCall),
        ValidatePermissionGraphChange(ValidatePermissionGraphChangeCall),
    }
    impl ::ethers::core::abi::AbiDecode for PermissionVerifierOracleAPICalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <SubmitPeerValidationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitPeerValidation(decoded));
            }
            if let Ok(decoded) =
                <ValidatePermissionGraphChangeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidatePermissionGraphChange(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PermissionVerifierOracleAPICalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SubmitPeerValidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatePermissionGraphChange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PermissionVerifierOracleAPICalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SubmitPeerValidation(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatePermissionGraphChange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SubmitPeerValidationCall> for PermissionVerifierOracleAPICalls {
        fn from(value: SubmitPeerValidationCall) -> Self {
            Self::SubmitPeerValidation(value)
        }
    }
    impl ::core::convert::From<ValidatePermissionGraphChangeCall> for PermissionVerifierOracleAPICalls {
        fn from(value: ValidatePermissionGraphChangeCall) -> Self {
            Self::ValidatePermissionGraphChange(value)
        }
    }
    ///Container type for all return fields from the `validatePermissionGraphChange` function with signature `validatePermissionGraphChange(string)` and selector `0x925b7ca5`
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
    pub struct ValidatePermissionGraphChangeReturn(pub [u8; 32]);
}
