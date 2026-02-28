#!/usr/bin/env bash
set -euo pipefail

read -r -p "Insert the version: " version

if [[ -z "${version// }" ]]; then
  echo "Version cannot be empty."
  exit 1
fi

tag="v${version}"

git tag "$tag"
mise changelog
git tag -d "$tag"

echo "✅ All done with success"