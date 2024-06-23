{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	nativeBuildInputs = with pkgs.buildPackages; [
		rustc
		cargo
		gcc
		rust-analyzer
	];

	RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
