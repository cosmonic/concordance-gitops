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
