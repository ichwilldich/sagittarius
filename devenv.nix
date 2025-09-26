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
  scripts.run-s3.exec = ''
    docker-compose -f $DEVENV_ROOT/apps/s3/docker-compose.yml up
  '';

  # Tests
  scripts.test-s3.exec = ''
    cargo watch -x "nextest run --workspace --all-features"
  '';
}
