#!/bin/bash
set -ex

scriptName="keep-alive-workflows (github-actions)"
branchName="keep-alive-workflows"

# github actions avatar
git config --global user.email "41898282+github-actions[bot]@users.noreply.github.com"
git config --global user.name "$scriptName"


# try to fetch the remote branch
if git fetch -f origin "$branchName"; then
   # fetched remote branch, check it out
   git checkout "$branchName"
else
   # failed fetch, need to create the branch locally
   echo "creating git branch [$branchName] ..."
   git checkout -b "$branchName"
fi

date +%Y-%m-%dT%H:%M:%SZ >> keep-alive-workflow
git add .
git commit -m "$scriptName"
git push --set-upstream origin keep-alive-workflows
