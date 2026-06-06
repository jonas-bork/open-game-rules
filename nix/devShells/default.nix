{
  pkgs,
  git-hooks,
  system,
  ...
}:
let
  rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ../../rust-toolchain.toml;

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
      nixfmt.enable = true;
      trufflehog.enable = true;
      yamllint.enable = true;
      markdownlint.enable = true;
    };
  };
in
{
  default = coreShell;
  ci = pkgs.mkShell {
    inherit (preCommitCheck) shellHook;
  };
}
