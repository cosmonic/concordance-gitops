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

pub(crate) fn apply_funds_deposited(
    input: FundsDeposited,
    state: Option<BankAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting funds deposited event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<BankAccountAggregateState>,
        ));
    };
    let state = BankAccountAggregateState {
        balance: state.balance + input.amount as u32,
        ..state
    };
    Ok(StateAck::ok(Some(state)))
}

pub(crate) fn apply_funds_withdrawn(
    input: FundsWithdrawn,
    state: Option<BankAccountAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        error!(
            "Rejecting funds withdrawn event. Account {} does not exist.",
            input.account_number
        );
        return Ok(StateAck::error(
            "Account does not exist",
            None::<BankAccountAggregateState>,
        ));
    };
    let state = state.withdraw(input.amount as u32);
    Ok(StateAck::ok(Some(state)))
}
