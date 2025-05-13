use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::{error::ErrorCode, ErrorObject},
};
pub use pallet_template_runtime_api::TemplateApi as TemplateRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use codec::Codec;

use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

#[rpc(client, server)]
pub trait TemplateApi<BlockHash, AccountId> {
    #[method(name = "get_value2")]
    fn get_value(&self, at: Option<BlockHash>) -> RpcResult<u32>;

    #[method(name = "get_username")]
    fn get_username(&self, account_id: AccountId) -> RpcResult<String>;

    #[method(name = "set_username")]
    fn set_username(&self, account_id: AccountId, username: String) -> RpcResult<bool>;
}

/// A struct that implements the `TemplateApi`.
pub struct TemplatePallet<C, Block> {
    // If you have more generics, no need to TemplatePallet<C, M, N, P, ...>
    // just use a tuple like TemplatePallet<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> TemplatePallet<C, Block> {
    /// Create new `TemplatePallet` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId> TemplateApiServer<<Block as BlockT>::Hash, AccountId> for TemplatePallet<C, Block>
where
    Block: BlockT,
    AccountId: Codec,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api: TemplateRuntimeApi<Block, AccountId, String>,
{
    fn get_value(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
        let api = self.client.runtime_api();
        let at = at.unwrap_or_else(|| self.client.info().best_hash);

        api.get_value(at).map_err(|_| {
            ErrorObject::owned(
                ErrorCode::InvalidParams.code(),
                "Failed to fetch username".to_string(),
                None::<()>,
            )
        })
    }

    fn get_username(
        &self,
        account_id: AccountId,
    ) -> RpcResult<String> {
        let api = self.client.runtime_api();

        let at = self.client.info().best_hash;
        api.get_username(at, account_id).map(|username| username.to_string()).map_err(|e| {
            ErrorObject::owned(
                ErrorCode::InvalidParams.code(),
                "Failed to fetch username".to_string(),
                Some(e.to_string()),
            )
        })
    }

    fn set_username(
        &self,
        account_id: AccountId,
        username: String
    ) -> RpcResult<bool> {
        let api = self.client.runtime_api();

        let at = self.client.info().best_hash;
        api.set_username(at, account_id, username).map_err(|e| {
            ErrorObject::owned(
                ErrorCode::InvalidParams.code(),
                "Failed to fetch username".to_string(),
                Some(e.to_string()),
            )
        })
    }
}
