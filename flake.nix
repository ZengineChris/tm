{
  description = "TM - Task Manager with Git Worktrees";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain with components
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # Native build inputs for git2 crate
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        # Build inputs for git2 crate (libgit2, openssl)
        buildInputs = with pkgs; [
          libgit2
          openssl
          zlib
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          packages = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-edit
            rust-analyzer
          ];

          # Set environment variables for git2-rs compilation
          LIBGIT2_SYS_USE_PKG_CONFIG = "1";
          PKG_CONFIG_PATH = "${pkgs.libgit2}/lib/pkgconfig";

          shellHook = ''
            echo "TM Development Environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo run            - Run the CLI"
            echo "  cargo clippy         - Run linter"
            echo "  cargo fmt            - Format code"
            echo "  cargo watch -x test  - Watch mode for tests"
          '';
        };

        # Package definition for building with Nix
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "tm";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          LIBGIT2_SYS_USE_PKG_CONFIG = "1";

          meta = with pkgs.lib; {
            description = "Task Manager - Git worktree-based task management CLI";
            homepage = "https://github.com/zengineChris/tm";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      }
    );
}
