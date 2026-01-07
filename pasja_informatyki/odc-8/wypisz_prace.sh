#!/bin/bash

set -e

i=1

for sciezka_do_dzialanika in ./*/src/main.rs; do
	katalog_dzialalnika=$(dirname "$(dirname "$sciezka_do_dzialanika")")
	nazwa_dzialanika=$(basename "$katalog_dzialalnika")
	echo "$i. \`$nazwa_dzialanika\`:"
	echo "\`\`\`"
	cat "$sciezka_do_dzialanika"
	echo "\`\`\`"
	echo

	((i++))
done
