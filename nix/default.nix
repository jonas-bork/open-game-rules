{
  nixpkgs,
  rust-overlay,
  ...
}@inputs:
let
  inherit (nixpkgs) lib;
  forAllSystems = lib.genAttrs lib.systems.flakeExposed;

  mkPkgs =
    system:
    import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];

      config = {
        allowUnfreePredicate =
          pkg:
          builtins.elem (lib.getName pkg) [
            # Allow specific unfree packages here by name
          ];
      };
    };

  forAllSystemsImport =
    file:
    forAllSystems (
      system:
      (import file (
        inputs
        // rec {
          inherit system;
          pkgs = mkPkgs system;
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
        }
      ))
    );
in
{
  devShells = forAllSystemsImport ./devShells;
  formatter = forAllSystemsImport ./formatter;
}
