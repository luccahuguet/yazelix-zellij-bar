{
  description = "Standalone Zellij zjstatus bar preset from Yazelix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      mkPkgs = system: nixpkgs.legacyPackages.${system};
      generatorPackage =
        system: pkgs:
        let
          rustToolchain = fenix.packages.${system}.combine [
            fenix.packages.${system}.stable.cargo
            fenix.packages.${system}.stable.rustc
          ];
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
          source = pkgs.lib.cleanSourceWith {
            name = "yazelix-bar-source";
            src = ./.;
            filter =
              path: _type:
              let
                relativePath = pkgs.lib.removePrefix ((toString ./.) + "/") (toString path);
              in
              relativePath != "target"
              && !pkgs.lib.hasPrefix "target/" relativePath
              && relativePath != ".git"
              && !pkgs.lib.hasPrefix ".git/" relativePath;
          };
        in
        rustPlatform.buildRustPackage {
          pname = "yazelix_bar_generator";
          version = "0.1.0";

          src = source;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "--bin"
            "yazelix_bar_generate"
          ];

          meta = {
            description = "Generator for standalone Yazelix bar zjstatus presets";
            homepage = "https://github.com/luccahuguet/yazelix-bar";
            license = pkgs.lib.licenses.asl20;
            mainProgram = "yazelix_bar_generate";
          };
        };
      barPackage =
        system: pkgs:
        let
          generator = generatorPackage system pkgs;
        in
        pkgs.stdenvNoCC.mkDerivation {
          pname = "yazelix_bar";
          version = "0.1.0";
          src = pkgs.lib.cleanSourceWith {
            name = "yazelix-bar-assets";
            src = ./.;
            filter =
              path: _type:
              let
                relativePath = pkgs.lib.removePrefix ((toString ./.) + "/") (toString path);
              in
              relativePath == "assets"
              || relativePath == "assets/zjstatus.wasm"
              || relativePath == "presets"
              || relativePath == "presets/examples"
              || pkgs.lib.hasPrefix "presets/" relativePath
              || relativePath == "README.md";
          };

          dontConfigure = true;
          dontBuild = true;

          installPhase = ''
            runHook preInstall

            install -Dm644 assets/zjstatus.wasm "$out/share/yazelix_bar/zjstatus.wasm"
            substitute presets/yazelix_bar.kdl "$out/share/yazelix_bar/yazelix_bar.kdl" \
              --replace-fail "__YAZELIX_BAR_ZJSTATUS_WASM__" "file:$out/share/yazelix_bar/zjstatus.wasm"
            install -Dm644 presets/yazelix_bar.kdl "$out/share/yazelix_bar/yazelix_bar.template.kdl"
            install -Dm755 ${generator}/bin/yazelix_bar_generate "$out/bin/yazelix_bar_generate"
            mkdir -p "$out/share/yazelix_bar/generated"
            "$out/bin/yazelix_bar_generate" \
              --wasm-url "file:$out/share/yazelix_bar/zjstatus.wasm" \
              > "$out/share/yazelix_bar/generated/yazelix_bar.kdl"
            cp -R presets/examples "$out/share/yazelix_bar/examples"
            install -Dm644 README.md "$out/share/doc/yazelix_bar/README.md"

            runHook postInstall
          '';

          doInstallCheck = true;
          nativeInstallCheckInputs = [
            pkgs.coreutils
            pkgs.gnugrep
          ];
          installCheckPhase = ''
            runHook preInstallCheck

            test -s "$out/share/yazelix_bar/zjstatus.wasm"
            test -x "$out/bin/yazelix_bar_generate"
            grep -q 'location="file:' "$out/share/yazelix_bar/yazelix_bar.kdl"
            grep -q 'format_right' "$out/share/yazelix_bar/generated/yazelix_bar.kdl"
            ! grep -q '__YAZELIX_BAR_ZJSTATUS_WASM__' "$out/share/yazelix_bar/yazelix_bar.kdl"
            test -s "$out/share/yazelix_bar/examples/custom_command_widgets.kdl"
            test -s "$out/share/yazelix_bar/examples/yazelix_runtime_widgets.kdl"

            runHook postInstallCheck
          '';

          passthru = {
            presetPath = "share/yazelix_bar/yazelix_bar.kdl";
            templatePath = "share/yazelix_bar/yazelix_bar.template.kdl";
            generatedPresetPath = "share/yazelix_bar/generated/yazelix_bar.kdl";
            examplesPath = "share/yazelix_bar/examples";
            generatorPath = "bin/yazelix_bar_generate";
            wasmPath = "share/yazelix_bar/zjstatus.wasm";
          };

          meta = {
            description = "Standalone Zellij zjstatus bar preset from Yazelix";
            homepage = "https://github.com/luccahuguet/yazelix-bar";
            license = pkgs.lib.licenses.asl20;
            mainProgram = "yazelix_bar_generate";
          };
        };
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
          bar = barPackage system pkgs;
          generator = generatorPackage system pkgs;
        in
        {
          default = bar;
          yazelix_bar = bar;
          yazelix-bar = bar;
          yazelix_bar_generate = generator;
        }
      );

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.yazelix_bar_generate}/bin/yazelix_bar_generate";
        };
        yazelix_bar_generate = {
          type = "app";
          program = "${self.packages.${system}.yazelix_bar_generate}/bin/yazelix_bar_generate";
        };
      });

      checks = forAllSystems (system: {
        yazelix_bar = self.packages.${system}.yazelix_bar;
        yazelix_bar_generate = self.packages.${system}.yazelix_bar_generate;
      });
    };
}
