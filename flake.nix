{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      self,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            clang
            # lld is much faster at linking than the default Rust linker
            lld
          ];
          buildInputs =
            with pkgs;
            [
              # rust toolchain
              # use rust-analyzer-nightly for better type inference
              rust-analyzer
              cargo-watch
              cargo-flamegraph
              gnuplot
              wgsl-analyzer
            ]
            # https://github.com/bevyengine/bevy/blob/v0.14.2/docs/linux_dependencies.md#nix
            ++ (lib.optionals pkgs.stdenv.isLinux [
              udev
              alsa-lib
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr # To use the x11 feature
              libxkbcommon
              wayland # To use the wayland feature
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rustfmt"
                  "clippy"
                  "llvm-tools"
                ];
                targets = [ "x86_64-pc-windows-gnu" ];
              })
            ])
            ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # https://discourse.nixos.org/t/the-darwin-sdks-have-been-updated/55295/1
              apple-sdk_15
              (rust-bin.stable.latest.default)
            ]);
          # LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
