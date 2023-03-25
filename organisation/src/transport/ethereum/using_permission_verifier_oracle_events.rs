pub use using_permission_verifier_oracle_events::*;
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
pub mod using_permission_verifier_oracle_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeValidated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphValidationRequested\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static USINGPERMISSIONVERIFIERORACLEEVENTS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct UsingPermissionVerifierOracleEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UsingPermissionVerifierOracleEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UsingPermissionVerifierOracleEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UsingPermissionVerifierOracleEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UsingPermissionVerifierOracleEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(UsingPermissionVerifierOracleEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UsingPermissionVerifierOracleEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                USINGPERMISSIONVERIFIERORACLEEVENTS_ABI.clone(),
                client,
            ))
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
            UsingPermissionVerifierOracleEventsEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UsingPermissionVerifierOracleEvents<M>
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
    pub enum UsingPermissionVerifierOracleEventsEvents {
        PermissionGraphChangeValidatedFilter(PermissionGraphChangeValidatedFilter),
        PermissionGraphValidationRequestedFilter(PermissionGraphValidationRequestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for UsingPermissionVerifierOracleEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PermissionGraphChangeValidatedFilter::decode_log(log) {
                return Ok(
                    UsingPermissionVerifierOracleEventsEvents::PermissionGraphChangeValidatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PermissionGraphValidationRequestedFilter::decode_log(log) {
                return Ok(
                    UsingPermissionVerifierOracleEventsEvents::PermissionGraphValidationRequestedFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UsingPermissionVerifierOracleEventsEvents {
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
        for UsingPermissionVerifierOracleEventsEvents
    {
        fn from(value: PermissionGraphChangeValidatedFilter) -> Self {
            Self::PermissionGraphChangeValidatedFilter(value)
        }
    }
    impl ::core::convert::From<PermissionGraphValidationRequestedFilter>
        for UsingPermissionVerifierOracleEventsEvents
    {
        fn from(value: PermissionGraphValidationRequestedFilter) -> Self {
            Self::PermissionGraphValidationRequestedFilter(value)
        }
    }
}
