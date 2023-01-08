
use crate::on_chain::ethereum_client::EthereumClient;

use std::sync::Arc;

struct OnChainEventListener {
    ethereum_client: Arc<EthereumClient>,
}

/// One thing that we could do:
///  create a smart contract for an organisation to register itself.
///  
///  and then we could simply query
///     that smart contract for the list of organisations.
///
///
///
///
/// Expose function for iterating over old events.
/// Expose function for polling for new events.
impl OnChainEventListener {
    /// Should return an iterator over events that have already occurred.
    async fn scan_events(&self) {
        // let events =
        //     self.smart_contract_client.events().from_block(0);

        // note that to get future events we call .stream
        // and for historical events we call .query
        // there's a feature request to change this;
        // https://github.com/gakonst/ethers-rs/issues/988

        // how to get all transactions for an address?
        // https://stackoverflow.com/questions/36291117/how-to-get-ethereum-transaction-list-by-address
        // https://github.com/ethereum/go-ethereum/issues/1897

        // how to watch all instances of a smart contract?
        // https://wiki.polygon.technology/docs/edge/get-started/set-up-ibft-locally/

        // get all events for a smart contract.
        // https://ethereum.stackexchange.com/questions/16313/how-can-i-view-event-logs-for-an-ethereum-contract

        // eth-get-logs
        // https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs
        // We can filter events/logs using fromBlock, toBlock, address, topics, or blockHash
        // so it seems like the most useful for us is to filter by address.
        // But now the question is:
        //   how do we get the address of the smart contract?
        //   on-chain:
        //      - create a smart contract and all organisations would need to register themselves.
        //      - every peer-set smart contract should also be registered
        //           there to broadcast information to others.
        //
        //   off-chain:
        //      ... - well this depends on how OneData works,.
        //

        // let mut stream = events.stream().await?;
        //
        // while let Some(Ok(evt)) = stream.next().await {
        //     info!("event: {:?}", evt);
        // }
    }
}
