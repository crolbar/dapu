{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-parts,
    rust-overlay,
    ...
  } @ inputs: let
    overlays = [(import rust-overlay)];
  in
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {system, ...}: let
        pkgs = import nixpkgs {inherit system overlays;};
      in {
        _module.args.pkgs = pkgs;

        devShells.default = let
          rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = ["rust-src" "rust-analyzer"];
          };
        in
          with pkgs;
            mkShell {
              nativeBuildInputs = [
                pkg-config
                openssl
                rust
              ];
            };

        packages = rec {
          dapu = pkgs.callPackage ./package.nix {};
          default = dapu;
        };
      };
    };
}
