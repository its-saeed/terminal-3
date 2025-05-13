#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;

sp_api::decl_runtime_apis! {
    pub trait TemplateApi<AccountId, Username>
    where
    AccountId: Codec,
    Username: Codec,
     {
        fn get_value() -> u32;
        fn get_username(account_id: AccountId) -> Username;
        fn set_username(account_id: AccountId, username: Username) -> bool;
    }
}
