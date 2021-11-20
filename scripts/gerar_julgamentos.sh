#!/usr/bin/env bash
processos="$(psql -c "COPY (SELECT id FROM processo) TO STDOUT WITH CSV")"

for i in {1..300}; do
    processo="'$(shuf -n 1 <<< "$processos")'"
    instancia="$(shuf -n 1 -e "'Primeira Instância'" "'Segunda Instância'" "'Tribunal Superior'")"
    data="'$(shuf -n 1 -i 1980-2021)-$(shuf -n 1 -i 01-12)-$(shuf -n 1 -i 01-28)'"
    procedente="'$(shuf -n 1 -i 0-1)'"

    psql -c "INSERT INTO julgamento VALUES ($processo, $instancia, $data, $procedente)"
done
