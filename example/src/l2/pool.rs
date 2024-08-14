use rollups_interface::l2::pool::{BatchSettings, TransactionPool};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedPool = Arc<RwLock<TransactionPoolImpl>>;

pub struct SimpleBatchSettings {
    pub max_size: usize,
}

impl Default for SimpleBatchSettings {
    fn default() -> Self {
        Self { max_size: 1024 }
    }
}

#[derive(Default)]
pub struct TransactionPoolImpl {
    pub transactions: Vec<super::tx::L2Transaction>,
}

impl TransactionPool for TransactionPoolImpl {
    type TxIn = super::tx::L2Transaction;
    type TxOut = super::tx::L2Transaction;
    type Settings = SimpleBatchSettings;

    fn insert(&mut self, tx: Self::TxIn) -> rollups_interface::error::Result<()> {
        // if `Self::TxIn` and `Self::TxOut` are not the same type, we should convert here
        self.transactions.push(tx);
        Ok(())
    }

    fn next_batch(&mut self, settings: Self::Settings) -> Vec<Self::TxOut> {
        if self.transactions.len() >= settings.max_size() {
            self.transactions.drain(..settings.max_size()).collect()
        } else {
            self.transactions.drain(..).collect()
        }
    }
}

impl BatchSettings for SimpleBatchSettings {
    fn max_size(&self) -> usize {
        self.max_size
    }
}
