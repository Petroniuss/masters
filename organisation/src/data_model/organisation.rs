use ethers::types::Address;
use ethers_signers::{LocalWallet, Signer};

/// we should make a distinction between organisation for which we
/// have secrets (like wallet etc) and other organisations that we only know exist.
/// this needs to somehow have a reference to all peer sets/
/// that a given organisation is part-of.
///
/// Some other organisations that ExecutingOrganisation is interested in.
#[derive(Clone, Debug)]
pub struct Organisation {
    pub name: String,
    pub ethereum_address: Address,
}

/// Currently running organisation - meaning the one that is running this code.
/// todo: come up with a better name :P
#[derive(Clone, Debug)]
pub struct ExecutingOrganisation {
    pub organisation: Organisation,
    pub wallet: LocalWallet,
}

impl ExecutingOrganisation {
    pub fn address(&self) -> Address {
        self.wallet.address()
    }
}
