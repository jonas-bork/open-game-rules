{ pkgs, ... }:
let
  ci = pkgs.writeShellApplication {
    name = "ci";
    text = ''
      set -e
      echo "Running pre-commit checks..."
      nix develop .#core -c pre-commit run -a

      echo "Running cargo tests..."
      nix develop .#core -c cargo test
    '';
  };
in
{
  ci = {
    type = "app";
    program = "${ci}/bin/ci";
  };
}
