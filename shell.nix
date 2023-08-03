{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "conetto";
  buildInputs = [
    # I have this on my system and don't want it installed. You all will have to suffer for right now.
    # pkgs.google-cloud-sdk
    pkgs.openssl
  ];

  shellHook = ''
    export GCLOUD_BEARER=$(gcloud auth application-default print-access-token)
  '';
}
