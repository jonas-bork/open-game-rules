{ pkgs, rustToolchain, ... }:
pkgs.writeShellScriptBin "format-all" ''
  echo "=== Formatting Nix files... ==="
  find . -type f -name "*.nix" -exec ${pkgs.nixfmt}/bin/nixfmt {} +

  echo ""
  echo "=== Formatting Rust files... ==="
  ${rustToolchain}/bin/cargo fmt

  echo ""
  echo "=== Done! ==="
''
