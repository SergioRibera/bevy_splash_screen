{
  description = "SplashScreen Bevy lib";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
  # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        bevyLibBundle = import ./. {
          inherit system pkgs flake-utils;
          crane = inputs.crane.mkLib pkgs;
          fenix = inputs.fenix.packages;
        };
      in {
        inherit (bevyLibBundle) apps devShells;
      }
    );
}
