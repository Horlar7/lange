let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  name = '' name '';
  buildInputs = with pkgs; '' build_inputs '';
  shellHook = '' shell_hook '';
}
