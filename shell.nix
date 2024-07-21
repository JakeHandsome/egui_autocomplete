let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";

  pkgs = import <nixpkgs> {
    overlays = [ (import rust-overlay) ];
  };

  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
pkgs.mkShell rec {
  packages = [ toolchain ];

  buildInputs = with pkgs; [
    rustup
    trunk
  
    # misc. libraries
    openssl
    pkg-config
  
    # GUI libs
    libxkbcommon
    libGL
    fontconfig
  
    # wayland libraries
    wayland
  
    # x11 libraries
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
    ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
  '';
}
