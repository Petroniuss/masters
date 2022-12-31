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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"organisationName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"_PermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"organisationName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"PermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestPermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"organisationName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_PermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePermissionGraphChange\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PERMISSIONGRAPH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PERMISSIONGRAPH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50604080516020810190915260008082529061002c90826100d1565b50610190565b634e487b7160e01b600052604160045260246000fd5b600181811c9082168061005c57607f821691505b60208210810361007c57634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156100cc57600081815260208120601f850160051c810160208610156100a95750805b601f850160051c820191505b818110156100c8578281556001016100b5565b5050505b505050565b81516001600160401b038111156100ea576100ea610032565b6100fe816100f88454610048565b84610082565b602080601f831160018114610133576000841561011b5750858301515b600019600386901b1c1916600185901b1785556100c8565b600085815260208120601f198616915b8281101561016257888601518255948401946001909101908401610143565b50858210156101805787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b6105bf8061019f6000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806320755771146100465780639f9fe1ba14610064578063ecaa133f14610079575b600080fd5b61004e610081565b60405161005b919061022e565b60405180910390f35b6100776100723660046102c5565b610113565b005b61004e6101a0565b60606000805461009090610331565b80601f01602080910402602001604051908101604052809291908181526020018280546100bc90610331565b80156101095780601f106100de57610100808354040283529160200191610109565b820191906000526020600020905b8154815290600101906020018083116100ec57829003601f168201915b5050505050905090565b7fb60cf0cc2b5f35bc877cc6c604ec9e596cc4b9efa20d534c8b4fd6e27c468c58848484846040516101489493929190610394565b60405180910390a1600061015d82848361042b565b507f25247b053c9e7919cd8239f3df6912fca441f50ecee836f4827503f034436a3f84846000604051610192939291906104ec565b60405180910390a150505050565b600080546101ad90610331565b80601f01602080910402602001604051908101604052809291908181526020018280546101d990610331565b80156102265780601f106101fb57610100808354040283529160200191610226565b820191906000526020600020905b81548152906001019060200180831161020957829003601f168201915b505050505081565b600060208083528351808285015260005b8181101561025b5785810183015185820160400152820161023f565b506000604082860101526040601f19601f8301168501019250505092915050565b60008083601f84011261028e57600080fd5b50813567ffffffffffffffff8111156102a657600080fd5b6020830191508360208285010111156102be57600080fd5b9250929050565b600080600080604085870312156102db57600080fd5b843567ffffffffffffffff808211156102f357600080fd5b6102ff8883890161027c565b9096509450602087013591508082111561031857600080fd5b506103258782880161027c565b95989497509550505050565b600181811c9082168061034557607f821691505b60208210810361036557634e487b7160e01b600052602260045260246000fd5b50919050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6040815260006103a860408301868861036b565b82810360208401526103bb81858761036b565b979650505050505050565b634e487b7160e01b600052604160045260246000fd5b601f82111561042657600081815260208120601f850160051c810160208610156104035750805b601f850160051c820191505b818110156104225782815560010161040f565b5050505b505050565b67ffffffffffffffff831115610443576104436103c6565b610457836104518354610331565b836103dc565b6000601f84116001811461048b57600085156104735750838201355b600019600387901b1c1916600186901b1783556104e5565b600083815260209020601f19861690835b828110156104bc578685013582556020948501946001909201910161049c565b50868210156104d95760001960f88860031b161c19848701351681555b505060018560011b0183555b5050505050565b60408152600061050060408301858761036b565b6020838203818501526000855461051681610331565b80855260018281168015610531576001811461054b57610579565b60ff1984168787015282151560051b870186019450610579565b896000528560002060005b84811015610571578154898201890152908301908701610556565b880187019550505b50929a995050505050505050505056fea2646970667358221220429b389825dab71152498901e78e6dcc221a34ecd65d968b41a19105e70b3ec964736f6c63430008110033" . parse () . expect ("invalid bytecode")
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
                PERMISSIONGRAPH_ABI.clone(),
                PERMISSIONGRAPH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    #[doc = "Container type for all input parameters for the `PermissionGraphIPFSPointer` function with signature `PermissionGraphIPFSPointer()` and selector `0xecaa133f`"]
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
    #[doc = "Container type for all input parameters for the `getLatestPermissionGraphIPFSPointer` function with signature `getLatestPermissionGraphIPFSPointer()` and selector `0x20755771`"]
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
    #[doc = "Container type for all input parameters for the `proposePermissionGraphChange` function with signature `proposePermissionGraphChange(string,string)` and selector `0x9f9fe1ba`"]
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
    #[doc = "Container type for all return fields from the `PermissionGraphIPFSPointer` function with signature `PermissionGraphIPFSPointer()` and selector `0xecaa133f`"]
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
    #[doc = "Container type for all return fields from the `getLatestPermissionGraphIPFSPointer` function with signature `getLatestPermissionGraphIPFSPointer()` and selector `0x20755771`"]
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
