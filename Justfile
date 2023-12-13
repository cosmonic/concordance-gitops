set shell := ["bash", "-cu"]

build:
    # Build all the actors using `wash build`
    for dir in $(find . -type f -name 'Cargo.toml' | xargs -n1 dirname | rg -v node_modules); do \
        echo "Building in $dir"; \
        export WASH_ISSUER_KEY=$(pwd)/.keys/bank_issuer.nk; \
        (cd $dir && wash build); \
    done
