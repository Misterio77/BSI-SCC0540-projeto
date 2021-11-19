#!/usr/bin/env bash
candidaturas="$(psql -c "COPY (SELECT candidato, ano FROM candidatura) TO STDOUT WITH CSV")"
# Para cada candidatura
while read -r candidatura; do
    candidato="'$(cut -d ',' -f 1 <<< "$candidatura")'"
    ano="'$(cut -d ',' -f 2 <<< "$candidatura")'"

    votos="'$(shuf -n 1 -i 00000000-99999999)'"

    psql -c "INSERT INTO pleito VALUES ($candidato, $ano, '1', $votos)"

done <<< "$candidaturas"

candidaturas_executivo="$(psql -c "COPY (SELECT candidato, ano FROM candidatura WHERE cargo_tipo = 'Presidente' OR cargo_tipo = 'Prefeito' OR cargo_tipo = 'Governador') TO STDOUT WITH CSV")"
# Para cada candidatura do executivo
while read -r candidatura; do
    candidato="'$(cut -d ',' -f 1 <<< "$candidatura")'"
    ano="'$(cut -d ',' -f 2 <<< "$candidatura")'"

    votos="'$(shuf -n 1 -i 00000000-99999999)'"

    psql -c "INSERT INTO pleito VALUES ($candidato, $ano, '2', $votos)"

done <<< "$candidaturas_executivo"
