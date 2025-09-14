{ pkgs, ... }:

{
  packages = with pkgs; [
    pkg-config
    openssl
  ];

  languages = {
    rust = {
      enable = true;
      channel = "stable";
    };

    javascript = {
      enable = true;
      npm = {
        enable = true;
      };
    };
  };
}
