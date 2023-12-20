# Bank Account Application Scripts

## Prerequisites

- [NATS CLI](https://github.com/nats-io/natscli#installation) for publishing commands and inspecting the event log
- [jq](https://jqlang.github.io/jq/) for convenience parsing with JSON requests and responses
- A Cosmonic account and the [cosmo](https://cosmonic.com/docs/getting-started/get-the-cli) CLI for creating local account credentials
- [make (optional)](https://www.gnu.org/software/make/) for convenience running the scripts

## Overview

This directory contains sample payloads and scripts for interacting with the Bank Account application. As this application
evolves from v0.0.0 to v0.3.0 new functionality is added, so it's recommended to use the appropriate scripts for each version.

Make sure to follow the steps in the [README](../README.md) to deploy the Bank Account application before running the scripts. Each of the scripts below is also contained in the [Makefile](./Makefile) for convenience if you have `make` installed.

## v0.0.0

As of v0.0.0, the Bank Account application does not contain any functionality. You can deploy the application and view the empty event catalog, but there are no commands to publish.

## v0.1.0

After upgrading to v0.1.0, you can publish the `CreateAccount` command to create a new bank account. The script for this version is [create-account.sh](./v0.1.0/create-account.sh) and will publish the payload in [./v0.1.0/create_account_cmd.json](./v0.1.0/create_account_cmd.json) to create an account `ACCT001`. You can run `make create_account` to run this command.

Now that an account is created, we can inspect the projection state to see account details with `make get_balance`.

```json
{
  "balance": 5000,
  "min_balance": 0,
  "account_number": "ACCT0001",
  "customer_id": "CUSTBOB",
  "reserved_funds": {}
}
```

## v0.2.0

After upgrading your application to `v0.2.0`, you can deposit and withdraw funds from your account. Use the commands `make deposit_funds` and `make withdraw_funds` to deposit and withdraw $1000 as many times as you'd like to change your account balance.

As you're depositing and withdrawing funds, you can check the immutable event log by using `make view_events` and see each of the commands and events that you've published.

## v0.3.0

In the latest version of the application, you can wire transfer money between accounts. Use `make create_account_2` to create a second account, and then `make wire_transfer` to transfer $1000 from ACCT0001 to ACCT0002.
