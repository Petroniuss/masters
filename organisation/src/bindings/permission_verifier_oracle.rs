pub use permission_verifier_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod permission_verifier_oracle {
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
    #[doc = "PermissionVerifierOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeValidated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphValidationRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"result\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitPeerValidation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validatePermissionGraphChange\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PERMISSIONVERIFIERORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PERMISSIONVERIFIERORACLE_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b506104db806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80630eb9312514610046578063925b7ca51461005b5780639d86698514610081575b600080fd5b61005961005436600461036f565b6100c2565b005b61006e61006936600461039f565b6102ba565b6040519081526020015b60405180910390f35b6100aa61008f366004610411565b6000602081905290815260409020546001600160a01b031681565b6040516001600160a01b039091168152602001610078565b6000828152602081905260409020546001600160a01b0316806101255760405162461bcd60e51b815260206004820152601660248201527514995c5d595cdd1259081a5cc81b9bdd081d985b1a5960521b60448201526064015b60405180910390fd5b6040516307c899ef60e31b81523360048201819052906001600160a01b03831690633e44cf7890602401602060405180830381865afa15801561016c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610190919061042a565b6101f55760405162461bcd60e51b815260206004820152603060248201527f6f6e6c79206120706565722063616e2076616c6964617465207065726d69737360448201526f696f6e206772617068206368616e676560801b606482015260840161011c565b6040805185815284151560208201527f6292a5fb4defdbf22b90ec9d9577e3b16e8c8039884cfc24998a330754aedd73910160405180910390a160405163240cede160e01b81526004810185905283151560248201526001600160a01b03828116604483015283169063240cede190606401600060405180830381600087803b15801561028157600080fd5b505af1158015610295573d6000803e3d6000fd5b50505060009485525050506020829052506040902080546001600160a01b0319169055565b600080339050600084846040516020016102d592919061044e565b6040516020818303038152906040528051906020012090507f7d24d0351ea8375a8319a8a6680987d327a66b99f8b05e56c8dc0c52cf33651c81838787604051610322949392919061045e565b60405180910390a1600081815260208190526040902080546001600160a01b0319166001600160a01b0393909316929092179091559392505050565b801515811461036c57600080fd5b50565b6000806040838503121561038257600080fd5b8235915060208301356103948161035e565b809150509250929050565b600080602083850312156103b257600080fd5b823567ffffffffffffffff808211156103ca57600080fd5b818501915085601f8301126103de57600080fd5b8135818111156103ed57600080fd5b8660208285010111156103ff57600080fd5b60209290920196919550909350505050565b60006020828403121561042357600080fd5b5035919050565b60006020828403121561043c57600080fd5b81516104478161035e565b9392505050565b8183823760009101908152919050565b8481526001600160a01b03841660208201526060604082018190528101829052818360808301376000818301608090810191909152601f909201601f19160101939250505056fea26469706673582212204631c9df1963eb82e7e2d83ea14efa2942ea4386df5cf618bff4a4bf616f229c64736f6c63430008110033" . parse () . expect ("invalid bytecode")
    });
    pub struct PermissionVerifierOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PermissionVerifierOracle<M> {
        fn clone(&self) -> Self {
            PermissionVerifierOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PermissionVerifierOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PermissionVerifierOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PermissionVerifierOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PermissionVerifierOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                PERMISSIONVERIFIERORACLE_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                PERMISSIONVERIFIERORACLE_ABI.clone(),
                PERMISSIONVERIFIERORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `requests` (0x9d866985) function"]
        pub fn requests(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([157, 134, 105, 133], p0)
                .expect("method not found (this should never happen)")
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
        ) -> ethers::contract::builders::Event<M, PermissionVerifierOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PermissionVerifierOracle<M>
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
    pub enum PermissionVerifierOracleEvents {
        PermissionGraphChangeValidatedFilter(PermissionGraphChangeValidatedFilter),
        PermissionGraphValidationRequestedFilter(PermissionGraphValidationRequestedFilter),
    }
    impl ethers::contract::EthLogDecode for PermissionVerifierOracleEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PermissionGraphChangeValidatedFilter::decode_log(log) {
                return Ok(
                    PermissionVerifierOracleEvents::PermissionGraphChangeValidatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PermissionGraphValidationRequestedFilter::decode_log(log) {
                return Ok(
                    PermissionVerifierOracleEvents::PermissionGraphValidationRequestedFilter(
                        decoded,
                    ),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PermissionVerifierOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PermissionVerifierOracleEvents::PermissionGraphChangeValidatedFilter(element) => {
                    element.fmt(f)
                }
                PermissionVerifierOracleEvents::PermissionGraphValidationRequestedFilter(
                    element,
                ) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `requests` function with signature `requests(bytes32)` and selector `0x9d866985`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "requests", abi = "requests(bytes32)")]
    pub struct RequestsCall(pub [u8; 32]);
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
    pub enum PermissionVerifierOracleCalls {
        Requests(RequestsCall),
        SubmitPeerValidation(SubmitPeerValidationCall),
        ValidatePermissionGraphChange(ValidatePermissionGraphChangeCall),
    }
    impl ethers::core::abi::AbiDecode for PermissionVerifierOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <RequestsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PermissionVerifierOracleCalls::Requests(decoded));
            }
            if let Ok(decoded) =
                <SubmitPeerValidationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PermissionVerifierOracleCalls::SubmitPeerValidation(decoded));
            }
            if let Ok(decoded) =
                <ValidatePermissionGraphChangeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PermissionVerifierOracleCalls::ValidatePermissionGraphChange(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PermissionVerifierOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PermissionVerifierOracleCalls::Requests(element) => element.encode(),
                PermissionVerifierOracleCalls::SubmitPeerValidation(element) => element.encode(),
                PermissionVerifierOracleCalls::ValidatePermissionGraphChange(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for PermissionVerifierOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PermissionVerifierOracleCalls::Requests(element) => element.fmt(f),
                PermissionVerifierOracleCalls::SubmitPeerValidation(element) => element.fmt(f),
                PermissionVerifierOracleCalls::ValidatePermissionGraphChange(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<RequestsCall> for PermissionVerifierOracleCalls {
        fn from(var: RequestsCall) -> Self {
            PermissionVerifierOracleCalls::Requests(var)
        }
    }
    impl ::std::convert::From<SubmitPeerValidationCall> for PermissionVerifierOracleCalls {
        fn from(var: SubmitPeerValidationCall) -> Self {
            PermissionVerifierOracleCalls::SubmitPeerValidation(var)
        }
    }
    impl ::std::convert::From<ValidatePermissionGraphChangeCall> for PermissionVerifierOracleCalls {
        fn from(var: ValidatePermissionGraphChangeCall) -> Self {
            PermissionVerifierOracleCalls::ValidatePermissionGraphChange(var)
        }
    }
    #[doc = "Container type for all return fields from the `requests` function with signature `requests(bytes32)` and selector `0x9d866985`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RequestsReturn(pub ethers::core::types::Address);
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
