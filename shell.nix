{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/0e6684e6c5755325f801bda1751a8a4038145d7d.tar.gz") { }
}:

let
  libInputs = with pkgs; [
    libGL
    libxkbcommon
    xorg.libX11
    xorg.libXi
  ];
  libPath = pkgs.lib.makeLibraryPath libInputs;
  shell = pkgs.mkShellNoCC {
    shellHook = ''
      export "LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${libPath}"
    '';
  };
in
shell
