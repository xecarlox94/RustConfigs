{ pkgs ? import <nixpkgs> {} }:
with pkgs;
  mkShell {
    buildInputs = [
      cargo
      rust-analyzer
      lunarvim
    ];
  }
