let
  pkgs = import ./nix { };
in
pkgs.mkShell {
  name = "lange";
  buildInputs = with pkgs; [
    cargo
    cargo-edit
    cargo-tree
    cargo-udeps
    rust-analyzer

    cachix
    nixpkgs-fmt
    niv
  ];
}
