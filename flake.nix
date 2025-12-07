{
  description = "Rust development environment (general-purpose)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Use oxalica’s overlay to get a consistent Rust toolchain with rust-src included
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.rust-analyzer
          ];

          RUST_BACKTRACE = 1;
        };
      }
    );
}
