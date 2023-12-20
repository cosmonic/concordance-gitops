#!/bin/bash

export NATS_URL=tls://connect.cosmonic.sh
export NATS_CREDS=~/.cosmo/user.creds

nats req cc.commands.bankaccount "`jq -c . create_account_cmd.json`"