{
  system ? builtins.currentSystem,
  pkgs,
  lib ? pkgs.lib,
  crane,
  fenix,
  flake-utils,
  stdenv ? pkgs.stdenv,
  ...
}: let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-Qxt8XAuaUR2OMdKbN4u8dBJOhSHxS+uS06Wl9+flVEk=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.overrideToolchain toolchain;

  # buildInputs for Examples
  buildInputs = with pkgs; [
    stdenv.cc.cc.lib
    alsa-lib
    udev
    libxkbcommon
    libxkbcommon.dev
    wayland
    wayland-protocols
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-loader
  ];

  # Base args, need for build all crate artifacts and caching this for late builds
  deps = {
    nativeBuildInputs = with pkgs;
      [
        pkg-config
        autoPatchelfHook
      ]
      ++ lib.optionals stdenv.buildPlatform.isDarwin [
        libiconv
      ]
      ++ lib.optionals stdenv.buildPlatform.isLinux [
        libxkbcommon.dev
      ];
    runtimeDependencies = with pkgs;
      lib.optionals stdenv.isLinux [
        wayland
        libGL
        libxkbcommon
      ];
    inherit buildInputs;
  };

  # Lambda for build packages with cached artifacts
  commonArgs = targetName:
    deps
    // {
      src = lib.cleanSourceWith {
        src = craneLib.path ./.;
        filter = craneLib.filterCargoSources;
      };
      doCheck = false;
      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
      CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";
      HOST_CC = "${stdenv.cc.nativePrefix}cc";
      pname = targetName;
      cargoExtraArgs = "-F dev --example ${targetName}";
    };
    bundleApp = targetName: flake-utils.lib.mkApp {
      drv = craneLib.buildPackage ((commonArgs targetName) // {
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs targetName);
      });
    };
    customSkipApp = bundleApp "custom_skip";
    layoutsApp = bundleApp "layouts";
    screensApp = bundleApp "screens";
    simpleApp = bundleApp "simple";
in {
  # `nix run`
  apps = rec {
    simple = simpleApp.app;
    layouts= layoutsApp.app;
    screens = screensApp.app;
    customSkip = customSkipApp.app;
    default = simple;
  };
  # `nix develop`
  devShells.default = craneLib.devShell {
    packages = with pkgs;
      [
        toolchain
        pkg-config
        cargo-release
      ] ++ buildInputs;
    LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  };
}
