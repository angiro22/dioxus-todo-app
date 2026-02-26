{ pkgs ? import <nixpkgs> { 
    config.android_sdk.accept_license = true; 
    config.allowUnfree = true; 
  } 
}:

let
  # Definiamo le versioni dell'SDK e dell'NDK necessarie per compilare per Android
  androidComposition = pkgs.androidenv.composeAndroidPackages {
    buildToolsVersions = [ "33.0.1" "34.0.0" ];
    platformVersions = [ "33" "34" ];
    ndkVersions = [ "25.2.9519653" ]; # Versione NDK molto stabile per Rust
    includeSystemImages = true;
    includeEmulator = true;
    includeNDK = true;
  }; 
  androidSdk = androidComposition.androidsdk;

  # Dipendenze necessarie per far girare l'app Dioxus su Desktop (NixOS)
  desktopDeps = with pkgs; [
    pkg-config
    openssl
    glib
    gtk3
    libsoup_3
    webkitgtk_4_1
    xdotool
  ];

in
pkgs.mkShell {
  buildInputs = [
    # Usiamo SOLO rustup. Tolti rustc e cargo per evitare conflitti!
    pkgs.rustup 
    
    pkgs.dioxus-cli
    pkgs.jdk17
    pkgs.android-tools # <--- AGGIUNTO: Fornisce il comando 'adb' alla shell
    androidSdk
  ] ++ desktopDeps;

  shellHook = ''
    export ANDROID_HOME="${androidSdk}/libexec/android-sdk"
    export ANDROID_NDK_ROOT="$ANDROID_HOME/ndk/25.2.9519653"
    export ANDROID_NDK_HOME="$ANDROID_NDK_ROOT" 
    export JAVA_HOME="${pkgs.jdk17.home}"
    
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath desktopDeps}:$LD_LIBRARY_PATH
    export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DA
TA_DIRS

    echo "Configurando la toolchain Rust..."
    # Diciamo a rustup di prendere il controllo assoluto del compilatore
    rustup toolchain install stable --profile default
    rustup default stable
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i68
6-linux-android
    
    echo "🚀 Ambiente Dioxus pronto e fixato!"
  '';
}