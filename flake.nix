{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        buildInputs = with pkgs; [
          openssl
          pkg-config
          fd
          rust-analyzer
          rust-bin.beta.latest.default
        ];

      in {

        devShells = {
          default = pkgs.mkShell {
            buildInputs = buildInputs;

            shellHook = ''
              alias find=fd
            '';
          };
        };

        apps = {
          default = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "run-rust-app" ''
              cargo run
            '';
          };
        };

      }
    );
}

