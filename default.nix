let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  dimooperChannel = pkgs.rustChannelOf {
    channel = "1.20.0";
  };
in rec {
  grossEnv = stdenv.mkDerivation rec {
    name = "gross-env";
    version = "0.0.1";
    buildInputs = [ pkgs.portmidi
                    pkgs.SDL2
                    pkgs.SDL2_ttf
                    pkgs.kcov
                    dimooperChannel.rust
                    dimooperChannel.rust-src ];
    RUST_SRC_PATH = "${dimooperChannel.rust-src}/lib/rustlib/src/rust/src";
  };
}
