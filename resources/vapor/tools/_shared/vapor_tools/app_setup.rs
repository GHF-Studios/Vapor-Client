fn setup_player(app_root: &Path) -> Result<(), String> {
    require_app_root(app_root)?;
    println!("Setup Player");
    println!();
    println!("App root: {}", app_root.display());
    for relative in PLAYER_DIRS {
        let path = app_root.join(relative);
        ensure_contained(app_root, &path)?;
        fs::create_dir_all(&path).map_err(io("create app-root directory", &path))?;
        println!("ensured {relative}");
    }
    if !steamcmd_status(app_root).ready {
        install_steamcmd(app_root)?;
    } else {
        println!("kept app-local SteamCMD");
    }
    write_app_root_anchor(app_root)?;
    write_receipt(app_root, "player", "ready")?;
    println!();
    println!("Status: player setup ready");
    Ok(())
}

/// Prepare a specific Steam installation for player use.
pub fn setup_player_for(app_root: &Path) -> Result<(), String> {
    setup_player(app_root)
}

fn setup_development(app_root: &Path) -> Result<(), String> {
    setup_player(app_root)?;
    println!();
    println!("Setup Development");
    if !rust_toolchain_installed(app_root) {
        install_rust(app_root)?;
    } else {
        install_rust_targets_and_components(app_root)?;
        println!("kept app-local Rust/Cargo");
    }
    if !cross_status(app_root).ready {
        install_zig(app_root)?;
        install_llvm_mingw(app_root)?;
        write_cross_wrappers(app_root)?;
    } else {
        println!("kept app-local cross-build tools");
    }
    write_receipt(app_root, "dev-env", "ready")?;
    println!();
    println!("Status: development setup ready");
    Ok(())
}

fn teardown_player(app_root: &Path) -> Result<(), String> {
    require_app_root(app_root)?;
    println!("Teardown Player");
    println!();
    println!("App root: {}", app_root.display());
    println!();
    teardown_development(app_root)?;
    println!();
    println!("Player-mode state");
    for relative in PLAYER_TEARDOWN_PATHS {
        let path = app_root.join(relative);
        ensure_contained(app_root, &path)?;
        if path.exists() || fs::symlink_metadata(&path).is_ok() {
            remove_path(&path).map_err(io("remove player path", &path))?;
            println!("removed {relative}");
        } else {
            println!("skipped absent {relative}");
        }
    }
    for relative in PLAYER_EMPTY_PARENT_DIRS {
        let path = app_root.join(relative);
        remove_empty_dir(app_root, &path)?;
    }
    println!();
    println!("Status: player setup removed");
    Ok(())
}

fn teardown_development(app_root: &Path) -> Result<(), String> {
    require_app_root(app_root)?;
    println!("Teardown Development");
    println!();
    for relative in [
        "rustup",
        "rustup-home",
        "cargo-home",
        "tools/zig",
        "tools/llvm-mingw",
        "tools/cross",
        ".vapor/state/installer/dev-env.toml",
    ] {
        let path = app_root.join(relative);
        ensure_contained(app_root, &path)?;
        if path.exists() {
            remove_path(&path).map_err(io("remove developer tool path", &path))?;
            println!("removed {relative}");
        } else {
            println!("skipped absent {relative}");
        }
    }
    println!();
    println!("Status: developer toolchain removed");
    Ok(())
}

fn install_steamcmd(app_root: &Path) -> Result<(), String> {
    println!("installing app-local SteamCMD");
    let target = app_root.join("tools/steamcmd");
    reset_dir(app_root, &target)?;
    if cfg!(target_os = "windows") {
        let archive = app_root.join(".vapor/downloads/steamcmd.zip");
        download(STEAMCMD_WINDOWS, &archive)?;
        extract_zip(&archive, &target)?;
    } else {
        let archive = app_root.join(".vapor/downloads/steamcmd_linux.tar.gz");
        download(STEAMCMD_LINUX, &archive)?;
        run(Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&target))?;
    }
    Ok(())
}

fn install_rust(app_root: &Path) -> Result<(), String> {
    println!("installing app-local Rust/Cargo {RUST_TOOLCHAIN}");
    let rustup_init = app_root
        .join(".vapor/downloads")
        .join(executable("rustup-init"));
    download(rustup_init_url()?, &rustup_init)?;
    make_executable(&rustup_init)?;
    run(Command::new(&rustup_init)
        .args([
            "-y",
            "--no-modify-path",
            "--profile",
            "default",
            "--default-toolchain",
            RUST_TOOLCHAIN,
            "--default-host",
            host_rust_target()?,
        ])
        .env("RUSTUP_HOME", app_root.join("rustup-home"))
        .env("CARGO_HOME", app_root.join("cargo-home")))?;
    let source = app_root.join("cargo-home/bin").join(executable("rustup"));
    let target = app_root.join("rustup/bin").join(executable("rustup"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(io("create rustup bin directory", parent))?;
    }
    fs::copy(&source, &target).map_err(io("copy rustup into app root", &target))?;
    make_executable(&target)?;
    install_rust_targets_and_components(app_root)
}

fn install_rust_targets_and_components(app_root: &Path) -> Result<(), String> {
    let rustup = app_root.join("rustup/bin").join(executable("rustup"));
    if !is_executable(&rustup) {
        return Err(format!("missing app-local rustup: {}", rustup.display()));
    }
    for target in RUST_TARGETS {
        println!("installing Rust target {target}");
        run(Command::new(&rustup)
            .args(["target", "add", target])
            .env("RUSTUP_HOME", app_root.join("rustup-home"))
            .env("CARGO_HOME", app_root.join("cargo-home")))?;
    }
    for component in RUST_COMPONENTS {
        println!("installing Rust component {component}");
        run(Command::new(&rustup)
            .args(["component", "add", component])
            .env("RUSTUP_HOME", app_root.join("rustup-home"))
            .env("CARGO_HOME", app_root.join("cargo-home")))?;
    }
    Ok(())
}

fn install_zig(app_root: &Path) -> Result<(), String> {
    println!("installing Zig {ZIG_VERSION}");
    let target = app_root.join("tools/zig");
    reset_dir(app_root, &target)?;
    if cfg!(target_os = "windows") {
        let archive = app_root
            .join(".vapor/downloads")
            .join(format!("zig-x86_64-windows-{ZIG_VERSION}.zip"));
        download(ZIG_X86_64_WINDOWS, &archive)?;
        extract_zip_single_root(&archive, &target)?;
    } else {
        let archive = app_root
            .join(".vapor/downloads")
            .join(format!("zig-x86_64-linux-{ZIG_VERSION}.tar.xz"));
        download(ZIG_X86_64_LINUX, &archive)?;
        extract_tar_xz_strip(&archive, &target)?;
    }
    Ok(())
}

fn install_llvm_mingw(app_root: &Path) -> Result<(), String> {
    println!("installing llvm-mingw {LLVM_MINGW_VERSION}");
    let target = app_root.join("tools/llvm-mingw");
    reset_dir(app_root, &target)?;
    if cfg!(target_os = "windows") {
        let archive = app_root
            .join(".vapor/downloads")
            .join(format!("llvm-mingw-{LLVM_MINGW_VERSION}-ucrt-x86_64.zip"));
        download(LLVM_MINGW_X86_64_WINDOWS, &archive)?;
        verify_sha256(&archive, LLVM_MINGW_X86_64_WINDOWS_SHA256)?;
        extract_zip_single_root(&archive, &target)?;
    } else {
        let archive = app_root.join(".vapor/downloads").join(format!(
            "llvm-mingw-{LLVM_MINGW_VERSION}-ucrt-ubuntu-22.04-x86_64.tar.xz"
        ));
        download(LLVM_MINGW_X86_64_LINUX, &archive)?;
        verify_sha256(&archive, LLVM_MINGW_X86_64_LINUX_SHA256)?;
        extract_tar_xz_strip(&archive, &target)?;
    }
    write(
        &target.join("vapor-toolchain.toml"),
        &format!(
            "schema = 1\nversion = \"{LLVM_MINGW_VERSION}\"\nruntime = \"ucrt\"\ntarget = \"x86_64-pc-windows-gnullvm\"\n"
        ),
    )?;
    Ok(())
}

fn write_cross_wrappers(app_root: &Path) -> Result<(), String> {
    let bin = app_root.join("tools/cross/bin");
    fs::create_dir_all(&bin).map_err(io("create cross-wrapper bin", &bin))?;
    if cfg!(windows) {
        let path = bin.join("x86_64-unknown-linux-gnu-zig-cc.cmd");
        write(
            &path,
            "@echo off\r\n\"%~dp0\\..\\..\\zig\\zig.exe\" cc -target x86_64-linux-gnu %*\r\n",
        )?;
    } else {
        let path = bin.join("x86_64-unknown-linux-gnu-zig-cc");
        write(
            &path,
            "#!/usr/bin/env sh\nexec \"$(dirname \"$0\")/../../zig/zig\" cc -target x86_64-linux-gnu \"$@\"\n",
        )?;
        make_executable(&path)?;
    }
    println!("wrote app-local cross-linker wrappers");
    Ok(())
}

fn directories_status(app_root: &Path) -> ComponentStatus {
    let missing = PLAYER_DIRS
        .iter()
        .filter(|relative| !app_root.join(relative).is_dir())
        .map(|relative| (*relative).to_owned())
        .collect::<Vec<_>>();
    ComponentStatus {
        label: "Generated directories",
        ready: missing.is_empty(),
        path: app_root.join(".vapor"),
        missing,
    }
}

fn steamcmd_status(app_root: &Path) -> ComponentStatus {
    let path = steamcmd_path(app_root);
    let ready = is_executable(&path);
    ComponentStatus {
        label: "SteamCMD",
        ready,
        path,
        missing: if ready {
            Vec::new()
        } else {
            vec!["steamcmd".to_owned()]
        },
    }
}

fn rust_status(app_root: &Path) -> ComponentStatus {
    let rustup = app_root.join("rustup/bin").join(executable("rustup"));
    let required = [
        "cargo",
        "rustc",
        "rustfmt",
        "cargo-clippy",
        "rustdoc",
        "rust-analyzer",
    ];
    let (path, mut missing) = match expected_rust_bin(app_root) {
        Ok(bin) => {
            let missing = required
                .iter()
                .filter(|name| !is_executable(&bin.join(executable(name))))
                .map(|name| format!("{} at {}", name, bin.join(executable(name)).display()))
                .collect::<Vec<_>>();
            (bin, missing)
        }
        Err(error) => (app_root.join("rustup-home/toolchains"), vec![error]),
    };
    if !is_executable(&rustup) {
        missing.push(format!("rustup at {}", rustup.display()));
    }
    ComponentStatus {
        label: "Rust/Cargo",
        ready: missing.is_empty(),
        path,
        missing,
    }
}

fn rust_toolchain_installed(app_root: &Path) -> bool {
    let rustup = app_root.join("rustup/bin").join(executable("rustup"));
    if !is_executable(&rustup) {
        return false;
    }
    expected_rust_bin(app_root).is_ok_and(|bin| {
        ["cargo", "rustc"]
            .into_iter()
            .all(|name| is_executable(&bin.join(executable(name))))
    })
}

fn cross_status(app_root: &Path) -> ComponentStatus {
    let mut missing = Vec::new();
    let zig = app_root.join("tools/zig").join(executable("zig"));
    if !is_executable(&zig) {
        missing.push(format!("zig at {}", zig.display()));
    }
    let llvm_bin = app_root.join("tools/llvm-mingw/bin");
    if !llvm_mingw_ucrt_marker_ready(app_root) {
        missing.push(format!(
            "UCRT toolchain marker at {}",
            app_root
                .join("tools/llvm-mingw/vapor-toolchain.toml")
                .display()
        ));
    }
    for name in [
        executable("x86_64-w64-mingw32-clang"),
        executable("x86_64-w64-mingw32-dlltool"),
        executable("llvm-dlltool"),
        executable("x86_64-w64-mingw32-ar"),
    ] {
        let path = llvm_bin.join(&name);
        if !is_executable(&path) {
            missing.push(format!("{name} at {}", path.display()));
        }
    }
    let runtime = app_root.join("tools/llvm-mingw/x86_64-w64-mingw32/bin/libunwind.dll");
    if !runtime.is_file() {
        missing.push(format!("libunwind.dll at {}", runtime.display()));
    }
    let runtime = app_root.join("tools/llvm-mingw/x86_64-w64-mingw32/bin/libunwind.dll");
    if !runtime.is_file() {
        missing.push(format!("libunwind.dll at {}", runtime.display()));
    }
    let wrapper = app_root.join("tools/cross/bin").join(if cfg!(windows) {
        "x86_64-unknown-linux-gnu-zig-cc.cmd"
    } else {
        "x86_64-unknown-linux-gnu-zig-cc"
    });
    if !is_executable(&wrapper) {
        missing.push(format!(
            "Linux cross-linker wrapper at {}",
            wrapper.display()
        ));
    }
    ComponentStatus {
        label: "Zig/Cross",
        ready: missing.is_empty(),
        path: zig,
        missing,
    }
}

fn llvm_mingw_ucrt_marker_ready(app_root: &Path) -> bool {
    let marker = app_root.join("tools/llvm-mingw/vapor-toolchain.toml");
    let Ok(source) = fs::read_to_string(marker) else {
        return false;
    };
    [
        "schema = 1",
        &format!("version = \"{LLVM_MINGW_VERSION}\""),
        "runtime = \"ucrt\"",
        "target = \"x86_64-pc-windows-gnullvm\"",
    ]
    .into_iter()
    .all(|required| source.lines().any(|line| line.trim() == required))
}
