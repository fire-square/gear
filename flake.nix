{
  description = "Firesquare backend";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        postgres_setup = ''
          export PGDATA=$PWD/postgres/data
          export PGHOST=$PWD/postgres
          export LOG_PATH=$PWD/postgres/LOG
          export PGDATABASE=postgres
          export DATABASE_CLEANER_ALLOW_REMOTE_DATABASE_URL=true
        '';

        startpgsql = pkgs.writeShellScriptBin "startpgsql" ''
          if [ ! -d $PGHOST ]; then
            mkdir -p $PGHOST
          fi
          if [ ! -d $PGDATA ]; then
            echo 'Initializing postgresql database...'
            LC_ALL=C.utf8 initdb $PGDATA --auth=trust >/dev/null
          fi
          pg_ctl start -l $LOG_PATH -o "-c listen_addresses= -c unix_socket_directories=$PGHOST"
          psql -tAc "SELECT 1 FROM pg_database WHERE datname = 'gear'" | grep -q 1 || psql -tAc 'CREATE DATABASE "gear"'
          # psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='gear'" | grep -q 1 || psql -tAc 'CREATE USER "gear"'
          # psql -tAc 'GRANT ALL PRIVILEGES ON gear TO "gear"'
        '';

        stoppgsql = pkgs.writeShellScriptBin "stoppgsql" ''
          pg_ctl -D $PGDATA stop
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustVersion
            diesel-cli
            postgresql
            startpgsql
            stoppgsql
          ];

          shellHook = ''
            ${postgres_setup}
            export DATABASE_URL="postgresql:///gear?host=$PWD/postgres";
          '';
        };
      }
    );
}
