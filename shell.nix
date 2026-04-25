let
  tarball = import ./nix/nixpkgs-tarball.nix;
  pkgs = import tarball { };
in
import ./nix/devshell.nix pkgs
