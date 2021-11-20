#!/usr/bin/env nix-shell
#! nix-shell -p pandoc -p texlive.combined.scheme-small -p plantuml -p pandoc-plantuml-filter -p drawio -p zip -i bash
#! nix-shell -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz

zip_name="10856803_11914601_11816021.zip"
DIR="/tmp/projeto-bd"

mkdir -p "$DIR"

drawio --export -o "$DIR"/03-ER.pdf ER.drawio

echo '```plantuml' > "$DIR"/Relacional.md
cat Relacional.uml >> "$DIR"/Relacional.md
echo '```' >> "$DIR"/Relacional.md
pandoc "$DIR"/Relacional.md -o "$DIR"/04-Relacional.pdf --filter pandoc-plantuml
rm "$DIR"/Relacional.md

cp DDL.sql "$DIR"/05-DDL.sql

cp DML.sql "$DIR"/06-DML.sql

zip "$DIR"/07-Prototipo.zip README.md assets templates Rocket.toml Cargo.* src -r

pandoc README.md -o "$DIR"/README.pdf

rm plantuml-images -r 2> /dev/null

(
    cd "$DIR"
    rm "$zip_name" 2> /dev/null
    zip "$zip_name" *
)

mv "$DIR"/"$zip_name" .
