#!/usr/bin/env bash
FILE="Makefile"
read -r -p "Enter your PostgreSQL version : " YOUR_PG_VERSION

# Remplace UNIQUEMENT la ligne PG_VERSION, en acceptant des espaces optionnels
sed -E -i.bak "s/^[[:space:]]*(PG_VERSION)[[:space:]]*=[[:space:]]*.*/\1 = $YOUR_PG_VERSION/" -- "$FILE"

echo "Makefile created for PostgreSQL v${YOUR_PG_VERSION}"