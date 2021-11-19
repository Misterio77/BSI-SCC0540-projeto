#!/usr/bin/env bash
pessoas="$(psql -c "COPY (SELECT cpfcnpj FROM individuo WHERE cpfcnpj SIMILAR TO '[0-9]{11}') TO STDOUT WITH CSV")"
candidaturas="$(psql -c "COPY (SELECT candidato,ano FROM candidatura) TO STDOUT WITH CSV")"

for i in {1..100}; do
    candidatura="$(shuf -n 1 <<< "$candidaturas")"

    apoiador="'$(shuf -n 1 <<< "$pessoas")'"
    candidato="'$(cut -d ',' -f 1 <<< "$candidatura")'"
    ano="'$(cut -d ',' -f 2 <<< "$candidatura")'"
    remunerado="'$(shuf -n 1 -i 0-1)'"
    funcao="'$(shuf -n 1 -e Designer Panfleteiro Marketeiro Tesoureiro Site)'"

    psql -c "INSERT INTO apoio VALUES ($apoiador, $candidato, $ano, $funcao, $remunerado)"
done
