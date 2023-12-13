use anyhow::Result;
use serde::{Deserialize, Serialize};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "projector",
    entity: "bank account"
});

mod store;

#[async_trait]
impl BankAccountProjector for BankAccountProjectorImpl {
    async fn handle_account_created(&self, input: AccountCreated) -> Result<()> {
        store::initialize_account(input).await
    }

    async fn handle_funds_deposited(&self, input: FundsDeposited) -> Result<()> {
        store::record_funds_deposited(input).await
    }

    async fn handle_funds_withdrawn(&self, input: FundsWithdrawn) -> Result<()> {
        store::record_funds_withdrawn(input).await
    }
}
