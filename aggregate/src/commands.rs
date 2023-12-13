use crate::*;

pub(crate) fn handle_create_account(input: CreateAccount) -> Result<EventList> {
    Ok(vec![Event::new(
        AccountCreated::TYPE,
        STREAM,
        &AccountCreated {
            initial_balance: input.initial_balance,
            account_number: input.account_number.to_string(),
            min_balance: input.min_balance,
            customer_id: input.customer_id,
        },
    )])
}

pub(crate) fn handle_withdraw_funds(
    input: WithdrawFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!(
            "Rejected command to withdraw funds. Account {} does not exist.",
            input.account_number
        ));
    };

    if state.available_balance() < input.amount as u32 {
        error!(
                "Rejecting command to withdraw funds, account {} does not have sufficient funds. Available {}",
                &input.account_number, state.available_balance()
            );
        Ok(vec![])
    } else {
        Ok(vec![Event::new(
            FundsWithdrawn::TYPE,
            STREAM,
            &FundsWithdrawn {
                note: input.note,
                account_number: input.account_number.to_string(),
                amount: input.amount,
                customer_id: input.customer_id,
            },
        )])
    }
}

pub(crate) fn handle_deposit_funds(
    input: DepositFunds,
    state: Option<BankAccountAggregateState>,
) -> Result<EventList> {
    if state.is_none() {
        return Err(anyhow::anyhow!(
            "Rejected command to deposit funds. Account {} does not exist.",
            input.account_number
        ));
    };

    Ok(vec![Event::new(
        FundsDeposited::TYPE,
        STREAM,
        &FundsDeposited {
            note: input.note,
            account_number: input.account_number.to_string(),
            amount: input.amount,
            customer_id: input.customer_id,
        },
    )])
}
