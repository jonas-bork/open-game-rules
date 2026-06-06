{
  pkgs,
  git-hooks,
  system,
  rustToolchain,
  ...
}:
let
  coreShell = pkgs.mkShell coreShellConfig;
  coreShellConfig = {
    inherit (preCommitCheck) shellHook;
    packages = [
      rustToolchain
      pkgs.pnpm # `pnpm` is currently needed for TypeScript type generation
      pkgs.trunk
    ];
  };

  preCommitCheck = git-hooks.lib.${system}.run {
    src = ../../.;
    hooks = {
      nixfmt.enable = true; # Nix formatter
      statix.enable = true; # Nix linter
      deadnix.enable = true; # Nix dead code checker
      markdownlint.enable = true; # Markdown
      trufflehog.enable = true; # Secret scanning

      # Rust
      rustfmt = {
        enable = true;
        packageOverrides = {
          rustfmt = rustToolchain;
          cargo = rustToolchain;
        };
      };
      clippy = {
        enable = true;
        packageOverrides = {
          clippy = rustToolchain;
          cargo = rustToolchain;
        };
      };
      cargo-check.enable = true; # Check Cargo
      cargo-sort.enable = true; # Sort Cargo dependencies

      # Misc
      check-added-large-files.enable = true;
      check-case-conflicts.enable = true;
      check-merge-conflicts.enable = true;
    };
  };
in
{
  default = coreShell;
  ci = pkgs.mkShell {
    inherit (preCommitCheck) shellHook;
  };
}
