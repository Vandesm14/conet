{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "conetto";
  buildInputs = [
    pkgs.google-cloud-sdk
    pkgs.openssl
    pkgs.espeak-classic
  ];

  shellHook = ''
    export GCLOUD_BEARER=$(gcloud auth application-default print-access-token)
  '';
}
