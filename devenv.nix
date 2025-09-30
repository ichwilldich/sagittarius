{ pkgs, ... }:

{
  packages = with pkgs; [
    pkg-config
    openssl
    cargo-watch
    cargo-nextest
    git
    sea-orm-cli
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

  # Apps
  scripts.run.exec = ''
    docker-compose -f $DEVENV_ROOT/docker-compose.yml up
  '';

  # Tests
  scripts.test-backend.exec = ''
    cargo watch -x "nextest run --workspace --all-features"
  '';
}
