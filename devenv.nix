{ pkgs, ... }:

{
  packages = with pkgs; [ 
    mdsh
  ];

  languages.rust.enable = true;
}
