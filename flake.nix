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

        myRustPkg = pkgs.rustPlatform.buildRustPackage {

          pname = "rust-configs";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;  # Ensures reproducibility
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

      in {

        devShells = {
          default = pkgs.mkShell {
            buildInputs = buildInputs ++ [ pkgs.xorg.xhost ];

            shellHook = ''
              alias find=fd
            '';
          };
        };

        apps = {
          default = flake-utils.lib.mkApp {
            drv = myRustPkg;
          };
        };

      }
    );
}

