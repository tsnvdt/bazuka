use crate::blockchain::Blockchain;
use crate::cli::BazukaConfig;
use crate::{
    blockchain::KvStoreChain, config::blockchain::get_blockchain_config, db::LevelDbKvStore,
};

pub async fn rollback(conf: &BazukaConfig) {
    let mut chain = KvStoreChain::new(
        LevelDbKvStore::new(&conf.db, 64).unwrap(),
        get_blockchain_config(),
    )
    .unwrap();
    chain.rollback().unwrap();
}
