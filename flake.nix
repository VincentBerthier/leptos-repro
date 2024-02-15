{
  description = "A Nix-flake-based Node.js development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        overlays = [
          (import rust-overlay)
          (final: prev: rec {
            nodejs = prev.nodejs-18_x;
            pnpm = prev.nodePackages.pnpm;
            yarn = (prev.yarn.override { inherit nodejs; });
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in with pkgs; {
      devShells.default = mkShell {
        buildInputs = [ 
          rust
          rust-analyzer
          dprint                                    # formatter
          emmet-ls                                  # language server
          openssl
          pkg-config
          cargo-generate                            # Utility crate for Rust Web
          cargo-leptos                              # Main crate for web development in Rust
          leptosfmt                                 # formatter for Leptos
          tailwindcss                               # Tailwind CSS engine
          tailwindcss-language-server               # language server
          nodePackages.prettier                     # Formatter web
          nodePackages.eslint                       # Linter web
          nodePackages.vscode-langservers-extracted # language server web
          openssl
          sass                                      # Sass and CSS extension language
          openvpn
        ];

        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
          if [ -z $HOME/.cargo/bin/gitmoji ]; then cargo install -q gitmoji; fi
          if [ $(hostname) == "athena" ]; then export CARGO_BUILD_JOBS=6; fi
          export ANDROID_HOME="$HOME/Android/Sdk"
          export PATH="$PATH:$ANDROID_HOME/emulator"
          export PATH="$PATH:$ANDROID_HOME/platform-tools"
          export ANDROID_AVD_PATH="$HOME/.android/avd"
        '';
      };
    });
}
