use super::messages::{PostMpnWithdrawRequest, PostMpnWithdrawResponse};
use super::{NodeContext, NodeError};
use crate::blockchain::Blockchain;
use crate::core::MpnSourcedTx;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn post_mpn_withdraw<B: Blockchain>(
    client: Option<SocketAddr>,
    context: Arc<RwLock<NodeContext<B>>>,
    req: PostMpnWithdrawRequest,
) -> Result<PostMpnWithdrawResponse, NodeError> {
    let mut context = context.write().await;
    let now = context.local_timestamp();
    let is_local = client.map(|c| c.ip().is_loopback()).unwrap_or(false);
    context
        .mempool
        .add_mpn_sourced(MpnSourcedTx::MpnWithdraw(req.tx), is_local, now);
    Ok(PostMpnWithdrawResponse {})
}
