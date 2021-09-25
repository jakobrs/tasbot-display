{ pkgs ? import <nixpkgs> {}, crossPkgs ? pkgs.pkgsCross.muslpi }:

crossPkgs.mkShell {
  name = "dev";

  buildInputs = with crossPkgs.buildPackages; [
    #gcc # implicit
  ];
}
