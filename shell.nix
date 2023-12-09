let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rustChannel = pkgs.rustChannelOf {
    channel = "stable";
  };
  rust = (rustChannel.rust.override {
    targets = [
      "wasm32-unknown-unknown"
    ];
  });
in
with pkgs;
mkShell {
  buildInputs = [
    rust
    rust-analyzer
    pkgconfig
    openssl
    sass
    glib
    glibc
    cairo
    pango
    atk
    gdk-pixbuf
    libsoup
    gtk3
    webkitgtk
    librsvg
    patchelf
    clippy
  ];
}
