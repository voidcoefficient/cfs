{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  cachix.enable = true;

  packages = with pkgs; [ ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
