pub use peer_broadcast::*;
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
pub mod peer_broadcast {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"peerMetadataIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContractAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"peerIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract PeerSetSmartContractAPI\",\"name\":\"peerSetSmartContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerPeerSet\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PEERBROADCAST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        1,
        249,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        54,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        31,
        11,
        170,
        107,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        51,
        231,
        251,
        69,
        20,
        97,
        0,
        80,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        78,
        97,
        0,
        73,
        54,
        96,
        4,
        97,
        0,
        225,
        86,
        91,
        97,
        0,
        99,
        86,
        91,
        0,
        91,
        97,
        0,
        78,
        97,
        0,
        94,
        54,
        96,
        4,
        97,
        1,
        17,
        86,
        91,
        97,
        0,
        162,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        130,
        22,
        129,
        82,
        127,
        30,
        180,
        29,
        164,
        139,
        95,
        60,
        107,
        20,
        200,
        239,
        183,
        205,
        167,
        81,
        215,
        234,
        159,
        24,
        102,
        178,
        98,
        251,
        225,
        74,
        46,
        129,
        214,
        209,
        245,
        69,
        111,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        86,
        91,
        127,
        19,
        255,
        133,
        101,
        153,
        209,
        201,
        62,
        135,
        111,
        52,
        229,
        7,
        41,
        60,
        100,
        100,
        112,
        67,
        204,
        1,
        113,
        202,
        164,
        45,
        53,
        248,
        1,
        92,
        86,
        69,
        92,
        51,
        131,
        131,
        96,
        64,
        81,
        97,
        0,
        213,
        147,
        146,
        145,
        144,
        97,
        1,
        131,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        0,
        243,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        1,
        10,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        32,
        131,
        133,
        3,
        18,
        21,
        97,
        1,
        36,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        60,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        133,
        1,
        145,
        80,
        133,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        80,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        1,
        95,
        87,
        96,
        0,
        128,
        253,
        91,
        134,
        96,
        32,
        130,
        133,
        1,
        1,
        17,
        21,
        97,
        1,
        113,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        146,
        144,
        146,
        1,
        150,
        145,
        149,
        80,
        144,
        147,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        132,
        22,
        129,
        82,
        96,
        64,
        96,
        32,
        130,
        1,
        129,
        144,
        82,
        129,
        1,
        130,
        144,
        82,
        129,
        131,
        96,
        96,
        131,
        1,
        55,
        96,
        0,
        129,
        131,
        1,
        96,
        96,
        144,
        129,
        1,
        145,
        144,
        145,
        82,
        96,
        31,
        144,
        146,
        1,
        96,
        31,
        25,
        22,
        1,
        1,
        146,
        145,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        138,
        157,
        75,
        75,
        219,
        206,
        34,
        155,
        38,
        204,
        239,
        52,
        183,
        33,
        97,
        161,
        165,
        174,
        105,
        95,
        227,
        25,
        175,
        32,
        23,
        125,
        76,
        110,
        113,
        138,
        46,
        236,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static PEERBROADCAST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        54,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        31,
        11,
        170,
        107,
        20,
        97,
        0,
        59,
        87,
        128,
        99,
        51,
        231,
        251,
        69,
        20,
        97,
        0,
        80,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        78,
        97,
        0,
        73,
        54,
        96,
        4,
        97,
        0,
        225,
        86,
        91,
        97,
        0,
        99,
        86,
        91,
        0,
        91,
        97,
        0,
        78,
        97,
        0,
        94,
        54,
        96,
        4,
        97,
        1,
        17,
        86,
        91,
        97,
        0,
        162,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        130,
        22,
        129,
        82,
        127,
        30,
        180,
        29,
        164,
        139,
        95,
        60,
        107,
        20,
        200,
        239,
        183,
        205,
        167,
        81,
        215,
        234,
        159,
        24,
        102,
        178,
        98,
        251,
        225,
        74,
        46,
        129,
        214,
        209,
        245,
        69,
        111,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        86,
        91,
        127,
        19,
        255,
        133,
        101,
        153,
        209,
        201,
        62,
        135,
        111,
        52,
        229,
        7,
        41,
        60,
        100,
        100,
        112,
        67,
        204,
        1,
        113,
        202,
        164,
        45,
        53,
        248,
        1,
        92,
        86,
        69,
        92,
        51,
        131,
        131,
        96,
        64,
        81,
        97,
        0,
        213,
        147,
        146,
        145,
        144,
        97,
        1,
        131,
        86,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        0,
        243,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        1,
        10,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        32,
        131,
        133,
        3,
        18,
        21,
        97,
        1,
        36,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        1,
        60,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        133,
        1,
        145,
        80,
        133,
        96,
        31,
        131,
        1,
        18,
        97,
        1,
        80,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        1,
        95,
        87,
        96,
        0,
        128,
        253,
        91,
        134,
        96,
        32,
        130,
        133,
        1,
        1,
        17,
        21,
        97,
        1,
        113,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        32,
        146,
        144,
        146,
        1,
        150,
        145,
        149,
        80,
        144,
        147,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        132,
        22,
        129,
        82,
        96,
        64,
        96,
        32,
        130,
        1,
        129,
        144,
        82,
        129,
        1,
        130,
        144,
        82,
        129,
        131,
        96,
        96,
        131,
        1,
        55,
        96,
        0,
        129,
        131,
        1,
        96,
        96,
        144,
        129,
        1,
        145,
        144,
        145,
        82,
        96,
        31,
        144,
        146,
        1,
        96,
        31,
        25,
        22,
        1,
        1,
        146,
        145,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        138,
        157,
        75,
        75,
        219,
        206,
        34,
        155,
        38,
        204,
        239,
        52,
        183,
        33,
        97,
        161,
        165,
        174,
        105,
        95,
        227,
        25,
        175,
        32,
        23,
        125,
        76,
        110,
        113,
        138,
        46,
        236,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static PEERBROADCAST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PeerBroadcast<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PeerBroadcast<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PeerBroadcast<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PeerBroadcast<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PeerBroadcast<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PeerBroadcast))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PeerBroadcast<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PEERBROADCAST_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                PEERBROADCAST_ABI.clone(),
                PEERBROADCAST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `registerPeer` (0x33e7fb45) function
        pub fn register_peer(
            &self,
            peer_ipfs_pointer: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 231, 251, 69], peer_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPeerSet` (0x1f0baa6b) function
        pub fn register_peer_set(
            &self,
            peer_set_smart_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 11, 170, 107], peer_set_smart_contract)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PeerRegistered` event
        pub fn peer_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PeerRegisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PeerSetRegistered` event
        pub fn peer_set_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PeerSetRegisteredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PeerBroadcastEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for PeerBroadcast<M>
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
    #[ethevent(name = "PeerRegistered", abi = "PeerRegistered(address,string)")]
    pub struct PeerRegisteredFilter {
        pub peer_address: ::ethers::core::types::Address,
        pub peer_metadata_ipfs_pointer: ::std::string::String,
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
    #[ethevent(name = "PeerSetRegistered", abi = "PeerSetRegistered(address)")]
    pub struct PeerSetRegisteredFilter {
        pub peer_set_smart_contract_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PeerBroadcastEvents {
        PeerRegisteredFilter(PeerRegisteredFilter),
        PeerSetRegisteredFilter(PeerSetRegisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for PeerBroadcastEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PeerRegisteredFilter::decode_log(log) {
                return Ok(PeerBroadcastEvents::PeerRegisteredFilter(decoded));
            }
            if let Ok(decoded) = PeerSetRegisteredFilter::decode_log(log) {
                return Ok(PeerBroadcastEvents::PeerSetRegisteredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PeerBroadcastEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PeerRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PeerSetRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PeerRegisteredFilter> for PeerBroadcastEvents {
        fn from(value: PeerRegisteredFilter) -> Self {
            Self::PeerRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<PeerSetRegisteredFilter> for PeerBroadcastEvents {
        fn from(value: PeerSetRegisteredFilter) -> Self {
            Self::PeerSetRegisteredFilter(value)
        }
    }
    ///Container type for all input parameters for the `registerPeer` function with signature `registerPeer(string)` and selector `0x33e7fb45`
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
    #[ethcall(name = "registerPeer", abi = "registerPeer(string)")]
    pub struct RegisterPeerCall {
        pub peer_ipfs_pointer: ::std::string::String,
    }
    ///Container type for all input parameters for the `registerPeerSet` function with signature `registerPeerSet(address)` and selector `0x1f0baa6b`
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
    #[ethcall(name = "registerPeerSet", abi = "registerPeerSet(address)")]
    pub struct RegisterPeerSetCall {
        pub peer_set_smart_contract: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PeerBroadcastCalls {
        RegisterPeer(RegisterPeerCall),
        RegisterPeerSet(RegisterPeerSetCall),
    }
    impl ::ethers::core::abi::AbiDecode for PeerBroadcastCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RegisterPeerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterPeer(decoded));
            }
            if let Ok(decoded) =
                <RegisterPeerSetCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterPeerSet(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PeerBroadcastCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RegisterPeer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterPeerSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PeerBroadcastCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RegisterPeer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterPeerSet(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RegisterPeerCall> for PeerBroadcastCalls {
        fn from(value: RegisterPeerCall) -> Self {
            Self::RegisterPeer(value)
        }
    }
    impl ::core::convert::From<RegisterPeerSetCall> for PeerBroadcastCalls {
        fn from(value: RegisterPeerSetCall) -> Self {
            Self::RegisterPeerSet(value)
        }
    }
}
