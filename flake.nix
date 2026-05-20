{
	description = "rust + python devenv";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

		flake-utils.url = "github:numtide/flake-utils";

		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		crane.url = "github:ipetkov/crane";
	};

	outputs = { self, nixpkgs, flake-utils, fenix, crane, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };

				tc = fenix.packages.${system}.stable;
				toolchain = tc.withComponents [
					"cargo"
					"clippy"
					"rust-src"
					"rustc"
					"rustfmt"
					"rust-analyzer"
				];

				craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
				src = craneLib.cleanCargoSource ./.;
				commonArgs = {
					inherit src;
					strictDeps = true;
				};

				cargoArtifacts = craneLib.buildDepsOnly commonArgs;
				crate = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

			in {
				devShells.default = pkgs.mkShell {
					packages = [
						toolchain
						fenix.packages.${system}.rust-analyzer
						pkgs.bacon

						pkgs.uv
					];

					RUST_SRC_PATH = "${tc.rust-src}/lib/rustlib/src/rust/library";
					
					LIBTORCH_USE_PYTORCH = "1";

					shellHook = ''
						echo "Syncing python virtual env with uv ..."
						uv sync
						
						export PATH="$PWD/.venv/bin:$PATH"
						
						PY_VER=$(uv run python -c "import sys; print(f'python{sys.version_info.major}.{sys.version_info.minor}')")
						
						export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.zlib}/lib:$PWD/.venv/lib/$PY_VER/site-packages/torch/lib:$LD_LIBRARY_PATH"
						
						echo "Devenv ready! PyTorch runtime bound via uv ($PY_VER)."
					'';
				};
				packages.default = crate;
			});
}
