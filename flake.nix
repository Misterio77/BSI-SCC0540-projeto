{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        pname = "projeto-bd";
      in
        rec {
          packages."${pname}" = naersk-lib.buildPackage {
            inherit pname;
            root = ./.;
          };
          defaultPackage = packages."${pname}";

          apps."${pname}" = flake-utils.lib.mkApp {
            drv = packages."${pname}";
          };
          defaultApp = apps."${pname}";

          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              # Rust tooling
              rustc
              cargo
              rust-analyzer
              rustfmt
              clippy
              # Postgres tooling
              postgresql
              pgformatter
              sqls
              # HTML/CSS tooling
              nodePackages.prettier
              sass
            ];
          };
        }
    );
}
