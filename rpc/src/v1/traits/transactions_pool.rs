//! Transactions pool PUB-SUB rpc interface.

use jsonrpc_core::Result;
use jsonrpc_pubsub::{typed, SubscriptionId};
use jsonrpc_derive::rpc;
use miner::pool::TxStatus;

use ethereum_types::H256;

/// Transactions Pool PUB-SUB rpc interface.
#[rpc(server)]
pub trait TransactionsPool {
	/// Pub/Sub Metadata
	type Metadata;

	/// Subscribe to Transactions Pool subscription.
	#[pubsub(subscription = "parity_watchTransactionsPool", subscribe, name = "parity_watchTransactionsPool")]
	fn subscribe(&self, _: Self::Metadata, _: typed::Subscriber<(H256, TxStatus)>);

	/// Unsubscribe from existing Transactions Pool subscription.
	#[pubsub(subscription = "parity_watchTransactionsPool", unsubscribe, name = "parity_unwatchTransactionsPool")]
	fn unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}
