{ pkgs, ... }:
let
  rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ../../rust-toolchain.toml;

  coreShell = pkgs.mkShell coreShellConfig;
  coreShellConfig = {
    packages = [
      rustToolchain
      pkgs.pnpm # `pnpm` is currently needed for TypeScript type generation
      pkgs.trunk
    ];
  };
in
{
  default = coreShell;
  core = coreShell;
}
