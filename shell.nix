let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  name = "lange";
  buildInputs = with pkgs; [
    cargo
    cargo-edit
    cargo-tree
    cargo-udeps
    rustfmt
    rust-analyzer

    nixpkgs-fmt
  ];
}
