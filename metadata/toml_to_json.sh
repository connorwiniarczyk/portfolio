#!/bin/sh
# Fetch the `media` array from each toml file in the metadata directory.
# Convert them into JSON strings using
# [toml-cli](https://github.com/gnprice/toml-cli) and use
# [jq](https://stedolan.github.io/jq/)
# to flatten them into a single array

tomlfile=$1
cd metadata

for file in *.toml
do
	toml get "$file" media
done | jq -s '[.[] | .[]]'
