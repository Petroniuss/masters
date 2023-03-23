pub use peer_set_smart_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod peer_set_smart_contract {
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
    #[doc = "PeerSetSmartContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_peers\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"contract PermissionVerifierOracleAPI\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_peerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"rejectedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"proposedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphChangeRequest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"peerRequestingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"updatedPeerSetPermissionGraphIPFSPointer\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PeerSetPermissionGraphUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"requestId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"result\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"peerValidatingChange\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__callback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_peer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPeer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestPeerSetPermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PermissionVerifierOracleAPI\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerRequestingChange\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peerSetPermissionGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"peers\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingGraphIPFSPointer\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingRequestId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"proposedGraphIPFSPointer\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePermissionGraphChange\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PEERSETSMARTCONTRACT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PEERSETSMARTCONTRACT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b5060405162001026380380620010268339810160408190526200003491620001d7565b600180546001600160a01b0319166001600160a01b03841617905560026200005d82826200035e565b5060005b8351811015620000cc5760016000808684815181106200008557620000856200042a565b6020908102919091018101516001600160a01b03168252810191909152604001600020805460ff191691151591909117905580620000c38162000440565b91505062000061565b5050505062000468565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b0381118282101715620001175762000117620000d6565b604052919050565b6001600160a01b03811681146200013557600080fd5b50565b805162000145816200011f565b919050565b600082601f8301126200015c57600080fd5b81516001600160401b03811115620001785762000178620000d6565b60206200018e601f8301601f19168201620000ec565b8281528582848701011115620001a357600080fd5b60005b83811015620001c3578581018301518282018401528201620001a6565b506000928101909101919091529392505050565b600080600060608486031215620001ed57600080fd5b83516001600160401b03808211156200020557600080fd5b818601915086601f8301126200021a57600080fd5b8151602082821115620002315762000231620000d6565b8160051b62000242828201620000ec565b928352848101820192828101908b8511156200025d57600080fd5b958301955b848710156200028b57865192506200027a836200011f565b828252958301959083019062000262565b98506200029d91505088820162000138565b955050506040860151915080821115620002b657600080fd5b50620002c5868287016200014a565b9150509250925092565b600181811c90821680620002e457607f821691505b6020821081036200030557634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200035957600081815260208120601f850160051c81016020861015620003345750805b601f850160051c820191505b81811015620003555782815560010162000340565b5050505b505050565b81516001600160401b038111156200037a576200037a620000d6565b62000392816200038b8454620002cf565b846200030b565b602080601f831160018114620003ca5760008415620003b15750858301515b600019600386901b1c1916600185901b17855562000355565b600085815260208120601f198616915b82811015620003fb57888601518255948401946001909101908401620003da565b50858210156200041a5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b634e487b7160e01b600052603260045260246000fd5b6000600182016200046157634e487b7160e01b600052601160045260246000fd5b5060010190565b610bae80620004786000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c80637dc0d1d0116100665780637dc0d1d014610148578063c5ff6d0e14610173578063e9ad1c0f1461017b578063f43806af1461018e578063feceb2cb146101a557600080fd5b8063191f5b03146100a35780631c8590ba146100c1578063240cede1146100f45780633e44cf78146101095780636c986d7c14610135575b600080fd5b6100ab6101ad565b6040516100b89190610605565b60405180910390f35b6100e46100cf36600461066f565b60006020819052908152604090205460ff1681565b60405190151581526020016100b8565b610107610102366004610691565b61023b565b005b6100e461011736600461066f565b6001600160a01b031660009081526020819052604090205460ff1690565b6101076101433660046106d4565b6103ca565b60015461015b906001600160a01b031681565b6040516001600160a01b0390911681526020016100b8565b6100ab610566565b60055461015b906001600160a01b031681565b61019760035481565b6040519081526020016100b8565b6100ab610573565b600480546101ba90610746565b80601f01602080910402602001604051908101604052809291908181526020018280546101e690610746565b80156102335780601f1061020857610100808354040283529160200191610233565b820191906000526020600020905b81548152906001019060200180831161021657829003601f168201915b505050505081565b6001546001600160a01b0316331461029a5760405162461bcd60e51b815260206004820152601860248201527f43616c6c6572206973206e6f7420746865206f7261636c65000000000000000060448201526064015b60405180910390fd5b82600354146102e45760405162461bcd60e51b815260206004820152601660248201527514995c5d595cdd1259081a5cc81b9bdd081d985b1a5960521b6044820152606401610291565b81156103475760026102f76004826107fa565b506005546040517f2d748f0b0f1783e02c003112a8c3100e4746e19fdc2c9b6f6515c4b4ca35e97f9161033a916001600160a01b039091169084906004906108d5565b60405180910390a1610392565b6005546040517f176088d54131b5b4dba3fe876b1adde4e6d135eeb38b0a7bd27a446c5d4cb07e91610389916001600160a01b039091169084906004906108d5565b60405180910390a15b6000600381905560408051602081019091529081526004906103b4908261097e565b5050600580546001600160a01b03191690555050565b600354156104255760405162461bcd60e51b815260206004820152602260248201527f546865726520697320616c726561647920612070656e64696e672072657175656044820152611cdd60f21b6064820152608401610291565b3360008181526020819052604090205460ff1661047b5760405162461bcd60e51b815260206004820152601460248201527321b0b63632b91034b9903737ba1030903832b2b960611b6044820152606401610291565b7fd15c5f30846c43353014be0d7108498bd0ebd7b7a4c67645dbaa0bad02b50cec3384846040516104ae93929190610a5b565b60405180910390a160015460405163925b7ca560e01b81526000916001600160a01b03169063925b7ca5906104e99087908790600401610a89565b6020604051808303816000875af1158015610508573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061052c9190610aa5565b600381905590506004610540848683610abe565b5050600580546001600160a01b0319166001600160a01b03929092169190911790555050565b600280546101ba90610746565b60606002805461058290610746565b80601f01602080910402602001604051908101604052809291908181526020018280546105ae90610746565b80156105fb5780601f106105d0576101008083540402835291602001916105fb565b820191906000526020600020905b8154815290600101906020018083116105de57829003601f168201915b5050505050905090565b600060208083528351808285015260005b8181101561063257858101830151858201604001528201610616565b506000604082860101526040601f19601f8301168501019250505092915050565b80356001600160a01b038116811461066a57600080fd5b919050565b60006020828403121561068157600080fd5b61068a82610653565b9392505050565b6000806000606084860312156106a657600080fd5b83359250602084013580151581146106bd57600080fd5b91506106cb60408501610653565b90509250925092565b600080602083850312156106e757600080fd5b823567ffffffffffffffff808211156106ff57600080fd5b818501915085601f83011261071357600080fd5b81358181111561072257600080fd5b86602082850101111561073457600080fd5b60209290920196919550909350505050565b600181811c9082168061075a57607f821691505b60208210810361077a57634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052604160045260246000fd5b601f8211156107e057600081815260208120601f850160051c810160208610156107bd5750805b601f850160051c820191505b818110156107dc578281556001016107c9565b5050505b505050565b600019600383901b1c191660019190911b1790565b818103610805575050565b61080f8254610746565b67ffffffffffffffff81111561082757610827610780565b61083b816108358454610746565b84610796565b6000601f82116001811461086957600083156108575750848201545b61086184826107e5565b8555506108ce565b600085815260209020601f19841690600086815260209020845b838110156108a35782860154825560019586019590910190602001610883565b50858310156108c15781850154600019600388901b60f8161c191681555b50505060018360011b0184555b5050505050565b6001600160a01b0384811682528316602080830191909152606060408301528254600091829161090481610746565b806060870152608060018084166000811461092657600181146109405761096e565b60ff1985168984015283151560051b89018301965061096e565b896000528560002060005b858110156109665781548b820186015290830190870161094b565b8a0184019750505b50949a9950505050505050505050565b815167ffffffffffffffff81111561099857610998610780565b6109a6816108358454610746565b602080601f8311600181146109d557600084156109c35750858301515b6109cd85826107e5565b8655506107dc565b600085815260208120601f198616915b82811015610a04578886015182559484019460019091019084016109e5565b5085821015610a225787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6001600160a01b0384168152604060208201819052600090610a809083018486610a32565b95945050505050565b602081526000610a9d602083018486610a32565b949350505050565b600060208284031215610ab757600080fd5b5051919050565b67ffffffffffffffff831115610ad657610ad6610780565b610aea83610ae48354610746565b83610796565b6000601f841160018114610b185760008515610b065750838201355b610b1086826107e5565b8455506108ce565b600083815260209020601f19861690835b82811015610b495786850135825560209485019460019092019101610b29565b5086821015610b665760001960f88860031b161c19848701351681555b505060018560011b018355505050505056fea26469706673582212200091fd6f05087071ecdb0baccc45f86ebc6ddb5a20766c3c3d5db23d6b73d2a264736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct PeerSetSmartContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PeerSetSmartContract<M> {
        fn clone(&self) -> Self {
            PeerSetSmartContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PeerSetSmartContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PeerSetSmartContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PeerSetSmartContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PeerSetSmartContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                PEERSETSMARTCONTRACT_ABI.clone(),
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
                PEERSETSMARTCONTRACT_ABI.clone(),
                PEERSETSMARTCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `__callback` (0x240cede1) function"]
        pub fn callback(
            &self,
            request_id: [u8; 32],
            result: bool,
            peer_validating_change: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 68, 207, 120], peer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestPeerSetPermissionGraphIPFSPointer` (0xfeceb2cb) function"]
        pub fn latest_peer_set_permission_graph_ipfs_pointer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([254, 206, 178, 203], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peerRequestingChange` (0xe9ad1c0f) function"]
        pub fn peer_requesting_change(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([233, 173, 28, 15], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peerSetPermissionGraphIPFSPointer` (0xc5ff6d0e) function"]
        pub fn peer_set_permission_graph_ipfs_pointer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([197, 255, 109, 14], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `peers` (0x1c8590ba) function"]
        pub fn peers(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([28, 133, 144, 186], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingGraphIPFSPointer` (0x191f5b03) function"]
        pub fn pending_graph_ipfs_pointer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([25, 31, 91, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingRequestId` (0xf43806af) function"]
        pub fn pending_request_id(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([244, 56, 6, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposePermissionGraphChange` (0x6c986d7c) function"]
        pub fn propose_permission_graph_change(
            &self,
            proposed_graph_ipfs_pointer: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 152, 109, 124], proposed_graph_ipfs_pointer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PeerSetPermissionGraphChangeRejected` event"]
        pub fn peer_set_permission_graph_change_rejected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerSetPermissionGraphChangeRejectedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetPermissionGraphChangeRequest` event"]
        pub fn peer_set_permission_graph_change_request_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerSetPermissionGraphChangeRequestFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `PeerSetPermissionGraphUpdated` event"]
        pub fn peer_set_permission_graph_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PeerSetPermissionGraphUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PeerSetSmartContractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PeerSetSmartContract<M>
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
        name = "PeerSetPermissionGraphChangeRejected",
        abi = "PeerSetPermissionGraphChangeRejected(address,address,string)"
    )]
    pub struct PeerSetPermissionGraphChangeRejectedFilter {
        pub peer_requesting_change: ethers::core::types::Address,
        pub peer_validating_change: ethers::core::types::Address,
        pub rejected_peer_set_permission_graph_ipfs_pointer: String,
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
        pub peer_requesting_change: ethers::core::types::Address,
        pub proposed_peer_set_permission_graph_ipfs_pointer: String,
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
        pub peer_requesting_change: ethers::core::types::Address,
        pub peer_validating_change: ethers::core::types::Address,
        pub updated_peer_set_permission_graph_ipfs_pointer: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PeerSetSmartContractEvents {
        PeerSetPermissionGraphChangeRejectedFilter(PeerSetPermissionGraphChangeRejectedFilter),
        PeerSetPermissionGraphChangeRequestFilter(PeerSetPermissionGraphChangeRequestFilter),
        PeerSetPermissionGraphUpdatedFilter(PeerSetPermissionGraphUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for PeerSetSmartContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PeerSetPermissionGraphChangeRejectedFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRejectedFilter(decoded),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphChangeRequestFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRequestFilter(decoded),
                );
            }
            if let Ok(decoded) = PeerSetPermissionGraphUpdatedFilter::decode_log(log) {
                return Ok(
                    PeerSetSmartContractEvents::PeerSetPermissionGraphUpdatedFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PeerSetSmartContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRejectedFilter(element) => {
                    element.fmt(f)
                }
                PeerSetSmartContractEvents::PeerSetPermissionGraphChangeRequestFilter(element) => {
                    element.fmt(f)
                }
                PeerSetSmartContractEvents::PeerSetPermissionGraphUpdatedFilter(element) => {
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
    #[ethcall(name = "__callback", abi = "__callback(bytes32,bool,address)")]
    pub struct CallbackCall {
        pub request_id: [u8; 32],
        pub result: bool,
        pub peer_validating_change: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `peerRequestingChange` function with signature `peerRequestingChange()` and selector `0xe9ad1c0f`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peerRequestingChange", abi = "peerRequestingChange()")]
    pub struct PeerRequestingChangeCall;
    #[doc = "Container type for all input parameters for the `peerSetPermissionGraphIPFSPointer` function with signature `peerSetPermissionGraphIPFSPointer()` and selector `0xc5ff6d0e`"]
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
        name = "peerSetPermissionGraphIPFSPointer",
        abi = "peerSetPermissionGraphIPFSPointer()"
    )]
    pub struct PeerSetPermissionGraphIPFSPointerCall;
    #[doc = "Container type for all input parameters for the `peers` function with signature `peers(address)` and selector `0x1c8590ba`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "peers", abi = "peers(address)")]
    pub struct PeersCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `pendingGraphIPFSPointer` function with signature `pendingGraphIPFSPointer()` and selector `0x191f5b03`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingGraphIPFSPointer", abi = "pendingGraphIPFSPointer()")]
    pub struct PendingGraphIPFSPointerCall;
    #[doc = "Container type for all input parameters for the `pendingRequestId` function with signature `pendingRequestId()` and selector `0xf43806af`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingRequestId", abi = "pendingRequestId()")]
    pub struct PendingRequestIdCall;
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PeerSetSmartContractCalls {
        Callback(CallbackCall),
        IsPeer(IsPeerCall),
        LatestPeerSetPermissionGraphIPFSPointer(LatestPeerSetPermissionGraphIPFSPointerCall),
        Oracle(OracleCall),
        PeerRequestingChange(PeerRequestingChangeCall),
        PeerSetPermissionGraphIPFSPointer(PeerSetPermissionGraphIPFSPointerCall),
        Peers(PeersCall),
        PendingGraphIPFSPointer(PendingGraphIPFSPointerCall),
        PendingRequestId(PendingRequestIdCall),
        ProposePermissionGraphChange(ProposePermissionGraphChangeCall),
    }
    impl ethers::core::abi::AbiDecode for PeerSetSmartContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CallbackCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::Callback(decoded));
            }
            if let Ok(decoded) = <IsPeerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::IsPeer(decoded));
            }
            if let Ok (decoded) = < LatestPeerSetPermissionGraphIPFSPointerCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (PeerSetSmartContractCalls :: LatestPeerSetPermissionGraphIPFSPointer (decoded)) }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PeerRequestingChangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::PeerRequestingChange(decoded));
            }
            if let Ok(decoded) =
                <PeerSetPermissionGraphIPFSPointerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeerSetSmartContractCalls::PeerSetPermissionGraphIPFSPointer(decoded));
            }
            if let Ok(decoded) = <PeersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::Peers(decoded));
            }
            if let Ok(decoded) =
                <PendingGraphIPFSPointerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::PendingGraphIPFSPointer(decoded));
            }
            if let Ok(decoded) =
                <PendingRequestIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PeerSetSmartContractCalls::PendingRequestId(decoded));
            }
            if let Ok(decoded) =
                <ProposePermissionGraphChangeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PeerSetSmartContractCalls::ProposePermissionGraphChange(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PeerSetSmartContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PeerSetSmartContractCalls::Callback(element) => element.encode(),
                PeerSetSmartContractCalls::IsPeer(element) => element.encode(),
                PeerSetSmartContractCalls::LatestPeerSetPermissionGraphIPFSPointer(element) => {
                    element.encode()
                }
                PeerSetSmartContractCalls::Oracle(element) => element.encode(),
                PeerSetSmartContractCalls::PeerRequestingChange(element) => element.encode(),
                PeerSetSmartContractCalls::PeerSetPermissionGraphIPFSPointer(element) => {
                    element.encode()
                }
                PeerSetSmartContractCalls::Peers(element) => element.encode(),
                PeerSetSmartContractCalls::PendingGraphIPFSPointer(element) => element.encode(),
                PeerSetSmartContractCalls::PendingRequestId(element) => element.encode(),
                PeerSetSmartContractCalls::ProposePermissionGraphChange(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for PeerSetSmartContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PeerSetSmartContractCalls::Callback(element) => element.fmt(f),
                PeerSetSmartContractCalls::IsPeer(element) => element.fmt(f),
                PeerSetSmartContractCalls::LatestPeerSetPermissionGraphIPFSPointer(element) => {
                    element.fmt(f)
                }
                PeerSetSmartContractCalls::Oracle(element) => element.fmt(f),
                PeerSetSmartContractCalls::PeerRequestingChange(element) => element.fmt(f),
                PeerSetSmartContractCalls::PeerSetPermissionGraphIPFSPointer(element) => {
                    element.fmt(f)
                }
                PeerSetSmartContractCalls::Peers(element) => element.fmt(f),
                PeerSetSmartContractCalls::PendingGraphIPFSPointer(element) => element.fmt(f),
                PeerSetSmartContractCalls::PendingRequestId(element) => element.fmt(f),
                PeerSetSmartContractCalls::ProposePermissionGraphChange(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CallbackCall> for PeerSetSmartContractCalls {
        fn from(var: CallbackCall) -> Self {
            PeerSetSmartContractCalls::Callback(var)
        }
    }
    impl ::std::convert::From<IsPeerCall> for PeerSetSmartContractCalls {
        fn from(var: IsPeerCall) -> Self {
            PeerSetSmartContractCalls::IsPeer(var)
        }
    }
    impl ::std::convert::From<LatestPeerSetPermissionGraphIPFSPointerCall>
        for PeerSetSmartContractCalls
    {
        fn from(var: LatestPeerSetPermissionGraphIPFSPointerCall) -> Self {
            PeerSetSmartContractCalls::LatestPeerSetPermissionGraphIPFSPointer(var)
        }
    }
    impl ::std::convert::From<OracleCall> for PeerSetSmartContractCalls {
        fn from(var: OracleCall) -> Self {
            PeerSetSmartContractCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PeerRequestingChangeCall> for PeerSetSmartContractCalls {
        fn from(var: PeerRequestingChangeCall) -> Self {
            PeerSetSmartContractCalls::PeerRequestingChange(var)
        }
    }
    impl ::std::convert::From<PeerSetPermissionGraphIPFSPointerCall> for PeerSetSmartContractCalls {
        fn from(var: PeerSetPermissionGraphIPFSPointerCall) -> Self {
            PeerSetSmartContractCalls::PeerSetPermissionGraphIPFSPointer(var)
        }
    }
    impl ::std::convert::From<PeersCall> for PeerSetSmartContractCalls {
        fn from(var: PeersCall) -> Self {
            PeerSetSmartContractCalls::Peers(var)
        }
    }
    impl ::std::convert::From<PendingGraphIPFSPointerCall> for PeerSetSmartContractCalls {
        fn from(var: PendingGraphIPFSPointerCall) -> Self {
            PeerSetSmartContractCalls::PendingGraphIPFSPointer(var)
        }
    }
    impl ::std::convert::From<PendingRequestIdCall> for PeerSetSmartContractCalls {
        fn from(var: PendingRequestIdCall) -> Self {
            PeerSetSmartContractCalls::PendingRequestId(var)
        }
    }
    impl ::std::convert::From<ProposePermissionGraphChangeCall> for PeerSetSmartContractCalls {
        fn from(var: ProposePermissionGraphChangeCall) -> Self {
            PeerSetSmartContractCalls::ProposePermissionGraphChange(var)
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
    pub struct LatestPeerSetPermissionGraphIPFSPointerReturn(pub String);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `peerRequestingChange` function with signature `peerRequestingChange()` and selector `0xe9ad1c0f`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeerRequestingChangeReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `peerSetPermissionGraphIPFSPointer` function with signature `peerSetPermissionGraphIPFSPointer()` and selector `0xc5ff6d0e`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeerSetPermissionGraphIPFSPointerReturn(pub String);
    #[doc = "Container type for all return fields from the `peers` function with signature `peers(address)` and selector `0x1c8590ba`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PeersReturn(pub bool);
    #[doc = "Container type for all return fields from the `pendingGraphIPFSPointer` function with signature `pendingGraphIPFSPointer()` and selector `0x191f5b03`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingGraphIPFSPointerReturn(pub String);
    #[doc = "Container type for all return fields from the `pendingRequestId` function with signature `pendingRequestId()` and selector `0xf43806af`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingRequestIdReturn(pub [u8; 32]);
}
