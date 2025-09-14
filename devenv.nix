{ pkgs, ... }:

{
  packages = with pkgs; [
    pkg-config
    openssl
    cargo-watch
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

  # Libs
  scripts.svelte-lib.exec = ''
    cargo watch -C $DEVENV_ROOT/packages/svelte -s 'npm run build'
  '';
}
