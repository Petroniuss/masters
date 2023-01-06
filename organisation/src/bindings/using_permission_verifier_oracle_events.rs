pub use using_permission_verifier_oracle_events::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod using_permission_verifier_oracle_events {
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
    #[doc = "UsingPermissionVerifierOracleEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeValidated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphValidationRequested\",\"outputs\":[],\"anonymous\":false}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static USINGPERMISSIONVERIFIERORACLEEVENTS_ABI:
        ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI)
                .expect("invalid abi")
        });
    pub struct UsingPermissionVerifierOracleEvents<M>(
        ethers::contract::Contract<M>,
    );
    impl<M> Clone for UsingPermissionVerifierOracleEvents<M> {
        fn clone(&self) -> Self {
            UsingPermissionVerifierOracleEvents(self.0.clone())
        }
    }
    impl<M> std::ops::Deref
        for UsingPermissionVerifierOracleEvents<M>
    {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug
        for UsingPermissionVerifierOracleEvents<M>
    {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter,
        ) -> std::fmt::Result {
            f.debug_tuple(stringify!(
                UsingPermissionVerifierOracleEvents
            ))
            .field(&self.address())
            .finish()
        }
    }
    impl<M: ethers::providers::Middleware>
        UsingPermissionVerifierOracleEvents<M>
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
                USINGPERMISSIONVERIFIERORACLEEVENTS_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Gets the contract's `PermissionGraphChangeValidated` event"]
        pub fn permission_graph_change_validated_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PermissionGraphChangeValidatedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `PermissionGraphValidationRequested` event"]
        pub fn permission_graph_validation_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            PermissionGraphValidationRequestedFilter,
        > {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            UsingPermissionVerifierOracleEventsEvents,
        > {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware>
        From<ethers::contract::Contract<M>>
        for UsingPermissionVerifierOracleEvents<M>
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
        pub peer_set_smart_contract:
            ethers::core::types::Address,
        pub proposed_graph_ipfs_pointer: String,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
    )]
    pub enum UsingPermissionVerifierOracleEventsEvents {
        PermissionGraphChangeValidatedFilter(
            PermissionGraphChangeValidatedFilter,
        ),
        PermissionGraphValidationRequestedFilter(
            PermissionGraphValidationRequestedFilter,
        ),
    }
    impl ethers::contract::EthLogDecode
        for UsingPermissionVerifierOracleEventsEvents
    {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) =
                PermissionGraphChangeValidatedFilter::decode_log(
                    log,
                )
            {
                return Ok(
                    UsingPermissionVerifierOracleEventsEvents::PermissionGraphChangeValidatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                PermissionGraphValidationRequestedFilter::decode_log(log)
            {
                return Ok (UsingPermissionVerifierOracleEventsEvents :: PermissionGraphValidationRequestedFilter (decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display
        for UsingPermissionVerifierOracleEventsEvents
    {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> ::std::fmt::Result {
            match self { UsingPermissionVerifierOracleEventsEvents :: PermissionGraphChangeValidatedFilter (element) => element . fmt (f) , UsingPermissionVerifierOracleEventsEvents :: PermissionGraphValidationRequestedFilter (element) => element . fmt (f) }
        }
    }
}
