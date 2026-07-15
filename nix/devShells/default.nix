{
  pkgs,
  git-hooks,
  system,
  rustToolchain,
  ...
}:
let
  inherit (pkgs) lib;

  libraries = with pkgs; [
    webkitgtk_4_1
    gtk3
    libsoup_3
    openssl
    gdk-pixbuf
    glib
    gobject-introspection
    cairo
  ];

  coreShell = pkgs.mkShell coreShellConfig;
  coreShellConfig = {
    inherit (preCommitCheck) shellHook;
    packages = with pkgs; [
      rustToolchain
      pnpm
    ];

    env = {
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
    };

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = libraries;
  };

  desktopShellConfig = lib.recursiveUpdate coreShellConfig {
    packages =
      coreShellConfig.packages
      ++ (with pkgs; [
        trunk
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

  preCommitCheck = git-hooks.lib.${system}.run {
    src = ../../.;
    hooks = {
      nixfmt.enable = true; # Nix formatter
      statix.enable = true; # Nix linter
      deadnix.enable = true; # Nix dead code checker
      markdownlint.enable = true; # Markdown
      trufflehog.enable = true; # Secret scanning
      yamllint.enable = true; # YAML linting

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
  desktop = pkgs.mkShell desktopShellConfig;
  android = pkgs.mkShell androidShellConfig;
}
