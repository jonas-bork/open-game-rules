{
  pkgs,
  shared,
  rustToolchain,
  ...
}:
let
  inherit (pkgs) lib;

  coreShell = pkgs.mkShell coreShellConfig;
  coreShellConfig = {
    inherit (shared.preCommit) shellHook;
    inherit (shared.core) nativeBuildInputs buildInputs env;
  };

  webShellConfig = lib.recursiveUpdate coreShellConfig {
    nativeBuildInputs =
      coreShellConfig.nativeBuildInputs
      ++ (with pkgs; [
        trunk
      ]);
  };

  desktopShellConfig = lib.recursiveUpdate webShellConfig {
    nativeBuildInputs =
      webShellConfig.nativeBuildInputs
      ++ (with pkgs; [
        cargo-tauri
      ]);
  };

  androidShellConfig = lib.recursiveUpdate desktopShellConfig {
    packages =
      desktopShellConfig.packages
      ++ (with pkgs; [
        android-tools
        (pkgs.android-studio.withSdk androidSetup.packages.androidsdk)
      ]);

    env = {
      ANDROID_HOME = androidSetup.androidHome;
      NDK_HOME = androidSetup.ndkHome;
    };
  };

  mkAndroidSetup =
    {
      platformVersion,
      platformToolsVersion,
      abiVersion, # armeabi-v7a, mips or x86_64
      systemImageType,
      buildToolsVersion,
      emulatorVersion,
      ndkVersion,
    }:
    rec {
      packages = pkgs.androidenv.composeAndroidPackages {
        inherit platformToolsVersion emulatorVersion;
        buildToolsVersions = [ buildToolsVersion ];
        includeEmulator = true;
        platformVersions = [
          platformVersion
        ];
        includeSources = true;
        includeSystemImages = true;
        systemImageTypes = [ systemImageType ];
        abiVersions = [
          abiVersion
        ];
        includeNDK = true;
        ndkVersions = [ ndkVersion ];
        # useGoogleAPIs = true;
        # useGoogleTVAddOns = false;
        # includeExtras = [
        #   "extras;google;gcm"
        # ];
      };

      androidHome = "${packages.androidsdk}/libexec/android-sdk";
      ndkHome = "${androidHome}/ndk/${ndkVersion}";
    };

  androidSetup = mkAndroidSetup {
    platformVersion = "36";
    abiVersion = "x86_64";
    systemImageType = "default";
    platformToolsVersion = "35.0.2";
    emulatorVersion = "35.5.2";
    buildToolsVersion = "35.0.0";
    ndkVersion = "28.0.13004108";
  };
in
{
  default = coreShell;
  core = coreShell;
  web = pkgs.mkShell webShellConfig;
  desktop = pkgs.mkShell desktopShellConfig;
  android = pkgs.mkShell androidShellConfig;

  rust = pkgs.mkShell {
    packages = [ rustToolchain ];
  };
}
