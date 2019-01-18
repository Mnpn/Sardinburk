with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "SARDINBURK-${version}";
  version = "YEE";

  buildInputs = [ pkgconfig openssl ];
}
