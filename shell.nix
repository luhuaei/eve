{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    gcc
    xorg.libX11.dev
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    pkg-config
  ];
  buildInputs = with pkgs; [
    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
