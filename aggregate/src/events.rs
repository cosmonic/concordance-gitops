use crate::*;

impl From<AccountCreated> for BankAccountAggregateState {
    fn from(input: AccountCreated) -> BankAccountAggregateState {
        BankAccountAggregateState {
            balance: input.initial_balance.unwrap_or(0) as _,
            min_balance: input.min_balance.unwrap_or(0) as _,
            account_number: input.account_number,
            customer_id: input.customer_id,
            reserved_funds: HashMap::new(),
        }
    }
}

pub(crate) fn apply_account_created(input: AccountCreated) -> Result<StateAck> {
    Ok(StateAck::ok(Some(BankAccountAggregateState::from(input))))
}
