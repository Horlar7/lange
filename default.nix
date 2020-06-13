{ pkgs ? null }:
let
  pkgs' = import ./nix { inherit pkgs; };
  generated = pkgs'.crate2nix.generatedCargoNix {
    name = "lange";
    src = ./.;
  };
  lange = pkgs'.callPackage generated { };
in
lange.rootCrate.build
