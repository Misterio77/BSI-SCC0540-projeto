{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    {
      overlay = final: prev: {
        projeto-bd = final.callPackage ./default.nix { };
      };
    } //
    (utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; overlays = [ self.overlay ]; };
      in {
        defaultPackage = pkgs.callPackage ./default.nix { };

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
            # Diagrams
            plantuml
          ];
        };
      }
    ));
}
