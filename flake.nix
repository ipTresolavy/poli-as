{
  description = "A flake to set up a development environment with Rust and other dependencies";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      rust_overlay = import (
        builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"
      );
      pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
      rustVersion = "latest";
      #rustVersion = "1.62.0";
      rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
        extensions = [
          "rust-src" # for rust-analyzer
          "rust-analyzer"
        ];
      };
    in
    {
      devShells.aarch64-darwin.default = pkgs.mkShell {
        buildInputs =
          [ rust ]
          ++ (with pkgs; [
            pkg-config
            lazygit
            gcc-arm-embedded-13
            qemu
          ]);
        RUST_BACKTRACE = 1;
      };
    };
}
