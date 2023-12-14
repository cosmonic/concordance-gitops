use anyhow::Result;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod commands;
mod events;
mod state;

use state::BankAccountAggregateState;

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "bank account"
});

impl BankAccountAggregate for BankAccountAggregateImpl {
    // -- Commands --
    fn handle_create_account(
        &self,
        input: CreateAccount,
        _state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<EventList> {
        commands::handle_create_account(input)
    }

    // -- Events --
    fn apply_account_created(
        &self,
        input: AccountCreated,
        _state: Option<BankAccountAggregateState>,
    ) -> anyhow::Result<StateAck> {
        events::apply_account_created(input)
    }
}

const STREAM: &str = "bankaccount";
