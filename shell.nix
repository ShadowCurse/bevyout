{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    pkgs.vulkan-loader
    pkgs.libxkbcommon
    pkgs.wayland
    pkgs.udev
    pkgs.alsaLib
  ]}"'';
  buildInputs = with pkgs; [
    wayland
    pkg-config
    udev
    alsaLib
  ];
}
