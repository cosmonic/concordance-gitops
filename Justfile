set shell := ["bash", "-cu"]

build:
    # Ensure events are up-to-date
    cd eventcatalog && npm run build
    # Build all the actors using `wash build`
    for dir in $(find . -type f -name 'Cargo.toml' | xargs -n1 dirname | rg -v node_modules); do \
        echo "Building $dir"; \
        export WASH_ISSUER_KEY=$(pwd)/.keys/bank_issuer.nk; \
        (cd $dir && wash build); \
    done

version := "0.3.0"
push:
    # Push to GHCR
    wash push ghcr.io/cosmonic/cosmonic-gitops/wiretransfer_processmanager:{{version}} process_manager/build/wiretransfer_processmanager_s.wasm
    wash push ghcr.io/cosmonic/cosmonic-gitops/bankaccount_projector:{{version}} projector/build/bankaccount_projector_s.wasm
    wash push ghcr.io/cosmonic/cosmonic-gitops/bankaccount_aggregate:{{version}} aggregate/build/bankaccount_aggregate_s.wasm
    wash push ghcr.io/cosmonic/cosmonic-gitops/bankaccount_catalog:{{version}} eventcatalog/actor/build/bankaccountcatalog_s.wasm