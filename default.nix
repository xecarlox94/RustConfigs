{ pkgs ? import <nixpkgs> {} }:
with pkgs;
  mkShell {
    buildInputs = [
      cargo
      rustc
      rust-analyzer
      lunarvim
      git
    ];
  }
