{

inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

outputs = { self, nixpkgs }: let
  allSystems = output: nixpkgs.lib.genAttrs
    nixpkgs.lib.systems.flakeExposed
    (system: output nixpkgs.legacyPackages.${system});
in {
  packages = allSystems (pkgs: {
    default = pkgs.rustPlatform.buildRustPackage {
      pname = "cirno-rust";
      version = "0.1.0";
      src = ./cirno;
      cargoLock.lockFile = ./cirno/Cargo.lock;
      patchPhase = ''
        sed -i -e "s'../stdlib'${self}/stdlib'g" src/lib.rs
      '';
      meta = {
        license = pkgs.lib.licenses.mit;
        mainProgram = "cirno";
      };
    };
  });

  devShells = allSystems (pkgs: {
    default = pkgs.mkShell {
      packages = [ pkgs.cargo ];
    };
  });
};

}
