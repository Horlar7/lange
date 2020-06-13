{ pkgs ? null, system ? builtins.currentSystem }:
let
  sources = import ./sources.nix;
  overlays = [
    (import sources.nixpkgs-mozilla)
    (self: super: {
      rustChannel = self.rustChannelOf { channel = "stable"; };
      rustFull = (self.rustChannel.rust.override {
        extensions = [
          "clippy-preview"
          "rls-preview"
          "rust-analysis"
          "rust-src"
          "rust-std"
          "rustfmt-preview"
        ];
      });
      cargo = self.rustFull;
      rustc = self.rustFull;
    })
    (self: super: { crate2nix = self.callPackage (sources.crate2nix + "/tools.nix") { }; })
  ];
  nixpkgs = if pkgs == null then sources.nixpkgs else pkgs;
in
import nixpkgs { inherit overlays system; }
