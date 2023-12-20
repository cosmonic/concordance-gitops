use std::collections::HashMap;

use crate::*;

use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::{debug, error};

// Note an invariant: the last() element in a ledger's effective_balance field is
// always the same as the balance stored in the balance.{account} key.

/// Creates a new AccountLedger instance with an initial transaction as a deposit,
/// sets the current balance to the initial amount
pub async fn initialize_account(event: AccountCreated) -> Result<()> {
    debug!("Initializing account {}", event.account_number);
    let kv = KeyValueSender::new();

    let account_number = event.account_number.to_string();
    let ctx = Context::default();

    let initial_balance = event.initial_balance.unwrap_or_default() as u32;

    // Set up the initial ledger
    let ledger_key = format!("ledger.{account_number}");
    let ledger = AccountLedger::new(event.account_number, initial_balance);
    let ledger_json = serde_json::to_string(&ledger).unwrap(); // we know this won't fail

    // set the current balance
    let balance_key = format!("balance.{account_number}");

    set(&ctx, &kv, ledger_key, ledger_json).await;
    set(&ctx, &kv, balance_key, initial_balance.to_string()).await;

    Ok(())
}

async fn set(ctx: &Context, kv: &KeyValueSender<WasmHost>, key: String, value: String) {
    if let Err(e) = kv
        .set(
            ctx,
            &SetRequest {
                key: key.clone(),
                value,
                expires: 0,
            },
        )
        .await
    {
        error!("Failed to set {key} in store: {e}");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountLedger {
    pub account_number: String,
    pub ledger_lines: Vec<LedgerLine>,
    pub holds: HashMap<String, u32>,
}

impl AccountLedger {
    fn new(account_number: String, initial_balance: u32) -> AccountLedger {
        AccountLedger {
            account_number,
            holds: HashMap::new(),
            ledger_lines: vec![LedgerLine {
                amount: initial_balance,
                tx_type: TransactionType::Deposit,
                effective_balance: initial_balance,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LedgerLine {
    pub amount: u32,
    pub tx_type: TransactionType,
    pub effective_balance: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum TransactionType {
    Withdrawal,
    Deposit,
    Transfer,
    FundsReserve,
    FundsRelease,
    Unknown,
}
