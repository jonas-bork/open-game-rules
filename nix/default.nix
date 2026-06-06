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
in
{
  devShells = forAllSystems (
    system:
    (import ./devShells (
      inputs
      // {
        inherit system;
        pkgs = mkPkgs system;
      }
    ))
  );
}
