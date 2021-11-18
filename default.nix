{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "projeto-bd";
  version = "1.0.0-pre2";

  src = lib.cleanSource ./.;

  # cargoSha256 = lib.fakeSha256;
  cargoSha256 = "sha256-ZRq/2rfz/Rutaoum1zXU+9cdO1AORgJPEjWYTB/GEeM=";

  postInstall = ''
    install -d $out/etc
    cp -r templates assets DDL.sql DML.sql $out/etc
  '';

  meta = with lib; {
    description = "Projeto de BD 2021";
    homepage = "https://sr.ht/~misterio/BSI-SCC0540-projeto";
    license = licenses.mit;
    platforms = platforms.all;
  };
}
