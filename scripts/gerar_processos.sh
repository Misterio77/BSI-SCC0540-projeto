#!/usr/bin/env bash
pessoas="$(psql -c "COPY (SELECT cpfcnpj FROM individuo WHERE cpfcnpj SIMILAR TO '[0-9]{11}') TO STDOUT WITH CSV")"

for i in {1..600}; do
    reu="'$(shuf -n 1 <<< "$pessoas")'"
    crime="$(shuf -n 1 -e "'Assassinato'" "'Tráfico de Drogas'" "'Estelionato'" "'Corrupção'" "'Maus Tratos'" "'Obstrução da Justiça'" "'Falsidade Ideológica'" "'Abacaxi na pizza'")"

    psql -c "INSERT INTO processo VALUES (DEFAULT, $reu, $crime)"
done
