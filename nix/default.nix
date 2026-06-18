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
            "android-studio"
            "platform-tools"
            "android-sdk-cmdline-tools"
            "android-sdk-tools"
            "android-sdk-platform-tools"
            "android-sdk-emulator"
            "system-image-36-default-x86_64"
            "android-sdk-system-image-36-default-x86_64"
            "emulator"
            "tools"
            "build-tools"
            "platforms"
            "sources"
            "ndk"
            "cmake"
            "cmdline-tools"
            "android-sdk-build-tools"
            "android-sdk-platforms"
            "android-sdk-sources"
            "android-sdk-ndk"
          ];

        android_sdk.accept_license = true;
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
