{ pkgs, ... }:

{
  packages = with pkgs; [ 
    bkt
    just
    mdsh
  ];

  languages.rust.enable = true;
}
