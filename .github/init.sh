#!/usr/bin/env bash

# Remove the init workflow so it won't get run more than once
git rm .github/workflows/init.yml

# Update links in the README
UPSTREAM_REPOSITORY="github.com/${TEMPLATE_REPOSITORY}/compare"
CURRENT_REPOSITORY="github.com/${GITHUB_REPOSITORY}/compare"
sed -i "s|${UPSTREAM_REPOSITORY}|${CURRENT_REPOSITORY}|g" README.md

# Update the Argo CD Application definition to point to this repository
sed -i "s|${TEMPLATE_REPOSITORY}|${GITHUB_REPOSITORY}|g" setup/argocd-application.yaml