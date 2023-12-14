use log::debug;
use sui_rust_operator::account::SuiAccount;

pub fn init_account() {
    let account = SuiAccount::new_account();
    debug!("account address is : {}", account.to_address())
}
