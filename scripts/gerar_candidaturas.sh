#!/usr/bin/env bash
pessoas="$(psql -c "COPY (SELECT cpfcnpj FROM individuo WHERE cpfcnpj SIMILAR TO '[0-9]{11}') TO STDOUT WITH CSV")"
cargos="$(psql -c "COPY (SELECT tipo, local FROM cargo) TO STDOUT WITH CSV")"
partidos="$(psql -c "COPY (SELECT numero FROM partido) TO STDOUT WITH CSV")"

# Para cada cargo
while read -r cargo; do
    cargo_tipo="'$(cut -d ',' -f 1 <<< "$cargo")'"
    cargo_local="'$(cut -d ',' -f 2 <<< "$cargo")'"

    # Escolher ano baseado no cargo
    if [ "$cargo_tipo" = "'Vereador'" ] || [ "$cargo_tipo" = "'Prefeito'" ]; then
        anos="2016 2020"
    else
        anos="2014 2018"
    fi

    # Para cada ano
    for ano in $anos; do
        # 3 candidatos
        for i in {1..3}; do
            # Escolher partido
            partido="$(shuf -n 1 <<< "$partidos")"

            # Escolher candidato
            candidato="'$(shuf -n 1 <<< "$pessoas")'"
            vice_candidato="NULL"

            # Gerar número e vice (se houver)
            if [ "$cargo_tipo" = "'Vereador'" ] || [ "$cargo_tipo" = "'DeputadoEstadual'" ]; then
                # 5 Dígitos
                numero="$partido$(printf "%03d\n" $(shuf -n 1 -i 000-999))"
            elif [ "$cargo_tipo" = "'DeputadoFederal'" ]; then
                # 4 Dígitos
                numero="$partido$(printf "%02d\n" $(shuf -n 1 -i 00-99))"
            elif [ "$cargo_tipo" = "'Senador'" ]; then
                # 3 Dígitos
                numero="$partido$(shuf -n 1 -i 0-9)"
            else
                # 2 Dígitos
                numero="$partido"
                vice_candidato="'$(shuf -n 1 <<< "$pessoas")'"
            fi

            psql -c "INSERT INTO candidatura VALUES ($candidato, $vice_candidato, $ano, $cargo_tipo, $cargo_local, $numero, $partido)"
        done
    done
done <<< "$cargos"
