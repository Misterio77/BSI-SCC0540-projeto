{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "projeto-bd";
  version = "1.4.0";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "rocket-0.5.0-rc.1" = "sha256-wmC/nekpOx7Dwy4dRVoEWxrznnlw9r3Nmq8J9X+Kbmo=";
      "rust_decimal-1.17.0" = "sha256-RjDMWS/F82HVHmSxy/JCSDoyBEFb3U0ux8LiZpOiX1M=";
    };
  };

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
