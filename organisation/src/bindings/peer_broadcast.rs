pub use peer_broadcast::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod peer_broadcast {
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
    #[doc = "PeerBroadcast was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"peerMetadataIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"peerIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeerSet\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PEERBROADCAST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PEERBROADCAST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506101f9806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80631f0baa6b1461003b57806333e7fb4514610050575b600080fd5b61004e6100493660046100e1565b610063565b005b61004e61005e366004610111565b6100a2565b6040516001600160a01b03821681527f1eb41da48b5f3c6b14c8efb7cda751d7ea9f1866b262fbe14a2e81d6d1f5456f9060200160405180910390a150565b7f13ff856599d1c93e876f34e507293c64647043cc0171caa42d35f8015c56455c3383836040516100d593929190610183565b60405180910390a15050565b6000602082840312156100f357600080fd5b81356001600160a01b038116811461010a57600080fd5b9392505050565b6000806020838503121561012457600080fd5b823567ffffffffffffffff8082111561013c57600080fd5b818501915085601f83011261015057600080fd5b81358181111561015f57600080fd5b86602082850101111561017157600080fd5b60209290920196919550909350505050565b6001600160a01b03841681526040602082018190528101829052818360608301376000818301606090810191909152601f909201601f191601019291505056fea2646970667358221220c519dbba734de36c5dd2ab1ac4a30fbed1997a00764ffced005de6e08f1a41b164736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct PeerBroadcast<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PeerBroadcast<M> {
        fn clone(&self) -> Self {
            PeerBroadcast(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PeerBroadcast<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PeerBroadcast<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PeerBroadcast))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PeerBroadcast<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PEERBROADCAST_ABI.clone(), client)
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
                PEERBROADCAST_ABI.clone(),
                PEERBROADCAST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `registerPeer` (0x33e7fb45) function"]
        pub fn register_peer(
            &self,
            peer_ipfs_pointer: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 231, 251, 69], peer_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerPeerSet` (0x1f0baa6b) function"]
        pub fn register_peer_set(
            &self,
            peer_set_smart_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 11, 170, 107], peer_set_smart_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PeerRegistered` event"]
        pub fn peer_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetRegistered` event"]
        pub fn peer_set_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerSetRegisteredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PeerBroadcastEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PeerBroadcast<M> {
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
    #[ethevent(name = "PeerRegistered", abi = "PeerRegistered(address,string)")]
    pub struct PeerRegisteredFilter {
        pub peer_address: ethers::core::types::Address,
        pub peer_metadata_ipfs_pointer: String,
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
    #[ethevent(name = "PeerSetRegistered", abi = "PeerSetRegistered(address)")]
    pub struct PeerSetRegisteredFilter {
        pub peer_set_smart_contract_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PeerBroadcastEvents {
        PeerRegisteredFilter(PeerRegisteredFilter),
        PeerSetRegisteredFilter(PeerSetRegisteredFilter),
    }
    impl ethers::contract::EthLogDecode for PeerBroadcastEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PeerRegisteredFilter::decode_log(log) {
                return Ok(PeerBroadcastEvents::PeerRegisteredFilter(decoded));
            }
            if let Ok(decoded) = PeerSetRegisteredFilter::decode_log(log) {
                return Ok(PeerBroadcastEvents::PeerSetRegisteredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PeerBroadcastEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeerBroadcastEvents::PeerRegisteredFilter(element) => element.fmt(f),
                PeerBroadcastEvents::PeerSetRegisteredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `registerPeer` function with signature `registerPeer(string)` and selector `0x33e7fb45`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registerPeer", abi = "registerPeer(string)")]
    pub struct RegisterPeerCall {
        pub peer_ipfs_pointer: String,
    }
    #[doc = "Container type for all input parameters for the `registerPeerSet` function with signature `registerPeerSet(address)` and selector `0x1f0baa6b`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registerPeerSet", abi = "registerPeerSet(address)")]
    pub struct RegisterPeerSetCall {
        pub peer_set_smart_contract: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PeerBroadcastCalls {
        RegisterPeer(RegisterPeerCall),
        RegisterPeerSet(RegisterPeerSetCall),
    }
    impl ethers::core::abi::AbiDecode for PeerBroadcastCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <RegisterPeerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerBroadcastCalls::RegisterPeer(decoded));
            }
            if let Ok(decoded) =
                <RegisterPeerSetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerBroadcastCalls::RegisterPeerSet(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PeerBroadcastCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PeerBroadcastCalls::RegisterPeer(element) => element.encode(),
                PeerBroadcastCalls::RegisterPeerSet(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PeerBroadcastCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeerBroadcastCalls::RegisterPeer(element) => element.fmt(f),
                PeerBroadcastCalls::RegisterPeerSet(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<RegisterPeerCall> for PeerBroadcastCalls {
        fn from(var: RegisterPeerCall) -> Self {
            PeerBroadcastCalls::RegisterPeer(var)
        }
    }
    impl ::std::convert::From<RegisterPeerSetCall> for PeerBroadcastCalls {
        fn from(var: RegisterPeerSetCall) -> Self {
            PeerBroadcastCalls::RegisterPeerSet(var)
        }
    }
}
