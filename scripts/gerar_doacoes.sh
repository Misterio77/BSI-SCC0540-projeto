#!/usr/bin/env bash
pessoas="$(psql -c "COPY (SELECT cpfcnpj FROM individuo WHERE cpfcnpj SIMILAR TO '[0-9]{11}') TO STDOUT WITH CSV")"
candidaturas="$(psql -c "COPY (SELECT candidato,ano FROM candidatura) TO STDOUT WITH CSV")"

for i in {1..2000}; do
    candidatura="$(shuf -n 1 <<< "$candidaturas")"

    candidato="'$(cut -d ',' -f 1 <<< "$candidatura")'"
    ano="'$(cut -d ',' -f 2 <<< "$candidatura")'"

    doador="'$(shuf -n 1 <<< "$pessoas")'"

    valor="'$(shuf -n 1 -i 100-10000)'"


    psql -c "INSERT INTO doacao VALUES (DEFAULT, $valor, $doador, $candidato, $ano)"
done
