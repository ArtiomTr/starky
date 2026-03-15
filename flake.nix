{
  description = "Rust development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    hax.url = "github:cryspen/hax";
  };

  outputs = { self, nixpkgs, flake-utils, hax }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain;
        rustChannel = rustToolchain.channel;
        rustComponents = [ "clippy" "rustfmt" ];
      in {
        devShells.default = pkgs.mkShell {
          packages = [ pkgs.rust-analyzer pkgs.rustup hax.packages.${system}.default ];

          RUSTUP_TOOLCHAIN = rustChannel;

          shellHook = ''
            export CARGO_HOME="$PWD/.direnv/cargo"
            export RUSTUP_HOME="$PWD/.direnv/rustup"
            export PATH="$CARGO_HOME/bin:$PATH"

            mkdir -p "$CARGO_HOME" "$RUSTUP_HOME"

            rustup toolchain install "$RUSTUP_TOOLCHAIN" \
              --profile minimal \
              --component ${builtins.concatStringsSep "," rustComponents}
          '';
        };
      });
}
