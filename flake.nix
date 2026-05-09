{
  description = "Standalone Zellij bar plugin package from Yazelix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zjstatus = {
      url = "github:dj95/zjstatus";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      zjstatus,
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
      toolPackage =
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
            name = "yazelix-zellij-bar-source";
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
          pname = "yazelix_zellij_bar_tools";
          version = "0.1.0";

          src = source;
          cargoLock.lockFile = ./Cargo.lock;

          meta = {
            description = "Standalone Yazelix Zellij bar generator and widget commands";
            homepage = "https://github.com/luccahuguet/yazelix-zellij-bar";
            license = pkgs.lib.licenses.asl20;
            mainProgram = "yazelix_zellij_bar_generate";
          };
        };
      barPackage =
        system: pkgs:
        let
          tools = toolPackage system pkgs;
          zjstatusPackage = zjstatus.packages.${system}.default;
        in
        pkgs.stdenvNoCC.mkDerivation {
          pname = "yazelix_zellij_bar";
          version = "0.1.0";
          src = pkgs.lib.cleanSourceWith {
            name = "yazelix-zellij-bar-assets";
            src = ./.;
            filter =
              path: _type:
              let
                relativePath = pkgs.lib.removePrefix ((toString ./.) + "/") (toString path);
              in
              relativePath == "presets"
              || relativePath == "presets/examples"
              || pkgs.lib.hasPrefix "presets/" relativePath
              || relativePath == "README.md";
          };

          dontConfigure = true;
          dontBuild = true;

          installPhase = ''
            runHook preInstall

            install -Dm644 ${zjstatusPackage}/bin/zjstatus.wasm "$out/share/yazelix_zellij_bar/zjstatus.wasm"
            substitute presets/yazelix_zellij_bar.kdl "$out/share/yazelix_zellij_bar/yazelix_zellij_bar.kdl" \
              --replace-fail "__YAZELIX_ZELLIJ_BAR_ZJSTATUS_WASM__" "file:$out/share/yazelix_zellij_bar/zjstatus.wasm"
            install -Dm644 presets/yazelix_zellij_bar.kdl "$out/share/yazelix_zellij_bar/yazelix_zellij_bar.template.kdl"
            install -Dm755 ${tools}/bin/yazelix_zellij_bar_generate "$out/bin/yazelix_zellij_bar_generate"
            install -Dm755 ${tools}/bin/yazelix_zellij_bar_widget "$out/bin/yazelix_zellij_bar_widget"
            mkdir -p "$out/share/yazelix_zellij_bar/generated"
            "$out/bin/yazelix_zellij_bar_generate" \
              --wasm-url "file:$out/share/yazelix_zellij_bar/zjstatus.wasm" \
              > "$out/share/yazelix_zellij_bar/generated/yazelix_zellij_bar.kdl"
            cp -R presets/examples "$out/share/yazelix_zellij_bar/examples"
            install -Dm644 README.md "$out/share/doc/yazelix_zellij_bar/README.md"

            runHook postInstall
          '';

          doInstallCheck = true;
          nativeInstallCheckInputs = [
            pkgs.coreutils
            pkgs.gnugrep
          ];
          installCheckPhase = ''
            runHook preInstallCheck

            test -s "$out/share/yazelix_zellij_bar/zjstatus.wasm"
            test -x "$out/bin/yazelix_zellij_bar_generate"
            test -x "$out/bin/yazelix_zellij_bar_widget"
            grep -q 'location="file:' "$out/share/yazelix_zellij_bar/yazelix_zellij_bar.kdl"
            grep -q 'format_right' "$out/share/yazelix_zellij_bar/generated/yazelix_zellij_bar.kdl"
            ! grep -q '__YAZELIX_ZELLIJ_BAR_ZJSTATUS_WASM__' "$out/share/yazelix_zellij_bar/yazelix_zellij_bar.kdl"
            test -s "$out/share/yazelix_zellij_bar/examples/custom_command_widgets.kdl"
            test -s "$out/share/yazelix_zellij_bar/examples/standalone_zellij_layout.kdl"
            test -s "$out/share/yazelix_zellij_bar/examples/yazelix_runtime_widgets.kdl"

            runHook postInstallCheck
          '';

          passthru = {
            presetPath = "share/yazelix_zellij_bar/yazelix_zellij_bar.kdl";
            templatePath = "share/yazelix_zellij_bar/yazelix_zellij_bar.template.kdl";
            generatedPresetPath = "share/yazelix_zellij_bar/generated/yazelix_zellij_bar.kdl";
            examplesPath = "share/yazelix_zellij_bar/examples";
            generatorPath = "bin/yazelix_zellij_bar_generate";
            widgetPath = "bin/yazelix_zellij_bar_widget";
            wasmPath = "share/yazelix_zellij_bar/zjstatus.wasm";
          };

          meta = {
            description = "Standalone Zellij bar plugin package from Yazelix";
            homepage = "https://github.com/luccahuguet/yazelix-zellij-bar";
            license = pkgs.lib.licenses.asl20;
            mainProgram = "yazelix_zellij_bar_generate";
          };
        };
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
          bar = barPackage system pkgs;
          tools = toolPackage system pkgs;
        in
        {
          default = bar;
          yazelix_zellij_bar = bar;
          yazelix-zellij-bar = bar;
          yazelix_zellij_bar_generate = tools;
          yazelix_zellij_bar_widget = tools;
        }
      );

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.yazelix_zellij_bar_generate}/bin/yazelix_zellij_bar_generate";
        };
        yazelix_zellij_bar_generate = {
          type = "app";
          program = "${self.packages.${system}.yazelix_zellij_bar_generate}/bin/yazelix_zellij_bar_generate";
        };
        yazelix_zellij_bar_widget = {
          type = "app";
          program = "${self.packages.${system}.yazelix_zellij_bar_widget}/bin/yazelix_zellij_bar_widget";
        };
      });

      checks = forAllSystems (system: {
        yazelix_zellij_bar = self.packages.${system}.yazelix_zellij_bar;
        yazelix_zellij_bar_generate = self.packages.${system}.yazelix_zellij_bar_generate;
        yazelix_zellij_bar_widget = self.packages.${system}.yazelix_zellij_bar_widget;
      });
    };
}
