{
  description = "A database viewer for the GNU/Linux Desktop";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        deps = with pkgs; [
              # rust deps
              cairo
              clippy
              rust-analyzer
              rustc
              rustfmt

              # glib deps
              gtk4
              libadwaita
              wrapGAppsHook4

              # compiler deps
              appstream
              appstream-glib
              desktop-file-utils
              glib
              libxml2
              meson
              ninja
              pkg-config

              # library deps
              fish
              openssl
              sqlite
            ];
      in
      {
        packages = rec {
          based = pkgs.stdenv.mkDerivation {
            pname = "Based";
            version = "1.0.0-dev00";
            src = ./.;

            cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
            buildInputs = deps;
            nativeBuildInputs = deps;

            meta = with pkgs.lib; {
              description = "A database viewer for the GNU/Linux Desktop";
              homepage = "https://github.com/Kylobytes/Based";
              license = licenses.gpl3;
            };
          };
          default = based;
        };

        devShell = pkgs.mkShell {
          cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
          buildInputs = deps;
          nativeBuildInputs = deps;

          shellHook = "exec fish";
        };
      }
    );
}
