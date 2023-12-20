.PHONY: help
.DEFAULT: help

NATS ?= nats
COSMO ?= cosmo
NATS_URL ?= tls://connect.cosmonic.sh
NATS_CREDS ?= ~/.cosmo/user.creds

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

login: ## Authenticate your local machine with Cosmonic
	$(COSMO) login

purge: ## Clear all data from the bank account service
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) stream purge CC_EVENTS -f
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) stream purge CC_COMMANDS -f
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) stream purge KV_CC_STATE -f

view_events: ## v0.0.0: View the bank account event stream
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) stream view CC_EVENTS

create_account: ## v0.1.0: Create a new bank account 
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) req cc.commands.bankaccount '$(shell jq -c . v0.1.0/create_account_cmd.json)'

get_balance: ## v0.1.0: Get the balance of the bank account
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) kv get CC_STATE agg.bankaccount.ACCT0001

deposit_funds: ## v0.2.0: Deposit $1000 into the bank account. Feel free to modify ./v0.2.0/deposit_funds_cmd.json to deposit more or less.
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) req cc.commands.bankaccount '$(shell jq -c . v0.2.0/deposit_funds_cmd.json)'

withdraw_funds: ## v0.2.0: Withdraw $1000 from the bank account. Feel free to modify ./v0.2.0/withdraw_funds_cmd.json to withdraw more or less.
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) req cc.commands.bankaccount '$(shell jq -c . v0.2.0/withdraw_funds_cmd.json)'

create_account_2: ## v0.3.0: Create a second new bank account 
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) req cc.commands.bankaccount '$(shell jq -c . v0.3.0/create_account_cmd.json)'

get_balance_2: ## v0.3.0: Get the balance of the second bank account
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) kv get CC_STATE agg.bankaccount.ACCT0002

wire_transfer: ## v0.3.0: Transfer $1000 from the first bank account to the second bank account. Feel free to modify ./v0.3.0/wire_funds_cmd.json to transfer more or less.
	NATS_URL=$(NATS_URL) NATS_CREDS=$(NATS_CREDS) $(NATS) req cc.commands.bankaccount '$(shell jq -c . v0.3.0/wire_funds_cmd.json)'

