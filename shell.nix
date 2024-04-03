with import <nixpkgs> {};

mkShell {
	nativeBuildInputs = [ rustup rust-analyzer ];
}
