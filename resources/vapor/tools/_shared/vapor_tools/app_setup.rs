fn setup_player(vapor_home: &Path) -> Result<(), String> {
    require_vapor_home(vapor_home)?;
    println!("Setup Player");
    println!();
    println!("Vapor home: {}", vapor_home.display());
    for relative in PLAYER_DIRS {
        let path = vapor_home.join(relative);
        ensure_contained(vapor_home, &path)?;
        fs::create_dir_all(&path).map_err(io("create Vapor-home directory", &path))?;
        println!("ensured {relative}");
    }
    write_vapor_home_anchor(vapor_home)?;
    write_receipt(vapor_home, "player", "ready")?;
    println!();
    println!("Status: player setup ready");
    Ok(())
}

/// Prepare the standalone Vapor home for Player use.
pub fn setup_player_for(vapor_home: &Path) -> Result<(), String> {
    setup_player(vapor_home)
}

fn setup_development(vapor_home: &Path) -> Result<(), String> {
    setup_player(vapor_home)?;
    println!();
    println!("Setup Development");
    if !steamcmd_status(vapor_home).ready {
        install_steamcmd(vapor_home)?;
    } else {
        println!("kept Vapor-managed SteamCMD");
    }
    if !rust_toolchain_installed(vapor_home) {
        install_rust(vapor_home)?;
    } else {
        install_rust_targets_and_components(vapor_home)?;
        println!("kept app-local Rust/Cargo");
    }
    if !cross_status(vapor_home).ready {
        install_zig(vapor_home)?;
        install_llvm_mingw(vapor_home)?;
        write_cross_wrappers(vapor_home)?;
    } else {
        println!("kept app-local cross-build tools");
    }
    write_receipt(vapor_home, "dev-env", "ready")?;
    println!();
    println!("Status: development setup ready");
    Ok(())
}

fn teardown_player(vapor_home: &Path) -> Result<(), String> {
    require_vapor_home(vapor_home)?;
    println!("Teardown Player");
    println!();
    println!("Vapor home: {}", vapor_home.display());
    println!();
    teardown_development(vapor_home)?;
    println!();
    println!("Player-mode state");
    for relative in PLAYER_TEARDOWN_PATHS {
        let path = vapor_home.join(relative);
        ensure_contained(vapor_home, &path)?;
        if path.exists() || fs::symlink_metadata(&path).is_ok() {
            remove_path(&path).map_err(io("remove player path", &path))?;
            println!("removed {relative}");
        } else {
            println!("skipped absent {relative}");
        }
    }
    for relative in PLAYER_EMPTY_PARENT_DIRS {
        let path = vapor_home.join(relative);
        remove_empty_dir(vapor_home, &path)?;
    }
    println!();
    println!("Status: player setup removed");
    Ok(())
}

fn teardown_development(vapor_home: &Path) -> Result<(), String> {
    require_vapor_home(vapor_home)?;
    println!("Teardown Development");
    println!();
    for relative in [
        "tools/rustup",
        "tools/cargo",
        "tools/zig",
        "tools/llvm-mingw",
        "tools/cross",
        "data/state/installer/dev-env.toml",
    ] {
        let path = vapor_home.join(relative);
        ensure_contained(vapor_home, &path)?;
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

fn install_steamcmd(vapor_home: &Path) -> Result<(), String> {
    println!("installing Vapor-managed SteamCMD");
    let target = vapor_home.join("tools/steamcmd");
    reset_dir(vapor_home, &target)?;
    if cfg!(target_os = "windows") {
        let archive = vapor_home.join("data/downloads/steamcmd.zip");
        download(STEAMCMD_WINDOWS, &archive)?;
        extract_zip(&archive, &target)?;
    } else {
        let archive = vapor_home.join("data/downloads/steamcmd_linux.tar.gz");
        download(STEAMCMD_LINUX, &archive)?;
        run(Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&target))?;
    }
    Ok(())
}

fn install_rust(vapor_home: &Path) -> Result<(), String> {
    println!("installing Vapor-managed Rust/Cargo {RUST_TOOLCHAIN}");
    let rustup_init = vapor_home
        .join("data/downloads")
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
        .env("RUSTUP_HOME", vapor_home.join("tools/rustup/home"))
        .env("CARGO_HOME", vapor_home.join("tools/cargo/home")))?;
    let source = vapor_home.join("tools/cargo/home/bin").join(executable("rustup"));
    let target = vapor_home.join("tools/rustup/bin").join(executable("rustup"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(io("create rustup bin directory", parent))?;
    }
    fs::copy(&source, &target).map_err(io("copy rustup into Vapor home", &target))?;
    make_executable(&target)?;
    install_rust_targets_and_components(vapor_home)
}

fn install_rust_targets_and_components(vapor_home: &Path) -> Result<(), String> {
    let rustup = vapor_home.join("tools/rustup/bin").join(executable("rustup"));
    if !is_executable(&rustup) {
        return Err(format!("missing app-local rustup: {}", rustup.display()));
    }
    for target in RUST_TARGETS {
        println!("installing Rust target {target}");
        run(Command::new(&rustup)
            .args(["target", "add", target])
            .env("RUSTUP_HOME", vapor_home.join("tools/rustup/home"))
            .env("CARGO_HOME", vapor_home.join("tools/cargo/home")))?;
    }
    for component in RUST_COMPONENTS {
        println!("installing Rust component {component}");
        run(Command::new(&rustup)
            .args(["component", "add", component])
            .env("RUSTUP_HOME", vapor_home.join("tools/rustup/home"))
            .env("CARGO_HOME", vapor_home.join("tools/cargo/home")))?;
    }
    Ok(())
}

fn install_zig(vapor_home: &Path) -> Result<(), String> {
    println!("installing Zig {ZIG_VERSION}");
    let target = vapor_home.join("tools/zig");
    reset_dir(vapor_home, &target)?;
    if cfg!(target_os = "windows") {
        let archive = vapor_home
            .join("data/downloads")
            .join(format!("zig-x86_64-windows-{ZIG_VERSION}.zip"));
        download(ZIG_X86_64_WINDOWS, &archive)?;
        extract_zip_single_root(&archive, &target)?;
    } else {
        let archive = vapor_home
            .join("data/downloads")
            .join(format!("zig-x86_64-linux-{ZIG_VERSION}.tar.xz"));
        download(ZIG_X86_64_LINUX, &archive)?;
        extract_tar_xz_strip(&archive, &target)?;
    }
    Ok(())
}

fn install_llvm_mingw(vapor_home: &Path) -> Result<(), String> {
    println!("installing llvm-mingw {LLVM_MINGW_VERSION}");
    let target = vapor_home.join("tools/llvm-mingw");
    reset_dir(vapor_home, &target)?;
    if cfg!(target_os = "windows") {
        let archive = vapor_home
            .join("data/downloads")
            .join(format!("llvm-mingw-{LLVM_MINGW_VERSION}-ucrt-x86_64.zip"));
        download(LLVM_MINGW_X86_64_WINDOWS, &archive)?;
        verify_sha256(&archive, LLVM_MINGW_X86_64_WINDOWS_SHA256)?;
        extract_zip_single_root(&archive, &target)?;
    } else {
        let archive = vapor_home.join("data/downloads").join(format!(
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

fn write_cross_wrappers(vapor_home: &Path) -> Result<(), String> {
    let bin = vapor_home.join("tools/cross/bin");
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
    println!("wrote Vapor-managed cross-linker wrappers");
    Ok(())
}

fn directories_status(vapor_home: &Path) -> ComponentStatus {
    let missing = PLAYER_DIRS
        .iter()
        .filter(|relative| !vapor_home.join(relative).is_dir())
        .map(|relative| (*relative).to_owned())
        .collect::<Vec<_>>();
    ComponentStatus {
        label: "Generated directories",
        ready: missing.is_empty(),
        path: vapor_home.join("data"),
        missing,
    }
}

fn steamcmd_status(vapor_home: &Path) -> ComponentStatus {
    let path = steamcmd_path(vapor_home);
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

fn rust_status(vapor_home: &Path) -> ComponentStatus {
    let rustup = vapor_home.join("tools/rustup/bin").join(executable("rustup"));
    let required = [
        "cargo",
        "rustc",
        "rustfmt",
        "cargo-clippy",
        "rustdoc",
        "rust-analyzer",
    ];
    let (path, mut missing) = match expected_rust_bin(vapor_home) {
        Ok(bin) => {
            let missing = required
                .iter()
                .filter(|name| !is_executable(&bin.join(executable(name))))
                .map(|name| format!("{} at {}", name, bin.join(executable(name)).display()))
                .collect::<Vec<_>>();
            (bin, missing)
        }
        Err(error) => (vapor_home.join("tools/rustup/home/toolchains"), vec![error]),
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

fn rust_toolchain_installed(vapor_home: &Path) -> bool {
    let rustup = vapor_home.join("tools/rustup/bin").join(executable("rustup"));
    if !is_executable(&rustup) {
        return false;
    }
    expected_rust_bin(vapor_home).is_ok_and(|bin| {
        ["cargo", "rustc"]
            .into_iter()
            .all(|name| is_executable(&bin.join(executable(name))))
    })
}

fn cross_status(vapor_home: &Path) -> ComponentStatus {
    let mut missing = Vec::new();
    let zig = vapor_home.join("tools/zig").join(executable("zig"));
    if !is_executable(&zig) {
        missing.push(format!("zig at {}", zig.display()));
    }
    let llvm_bin = vapor_home.join("tools/llvm-mingw/bin");
    if !llvm_mingw_ucrt_marker_ready(vapor_home) {
        missing.push(format!(
            "UCRT toolchain marker at {}",
            vapor_home
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
    let runtime = vapor_home.join("tools/llvm-mingw/x86_64-w64-mingw32/bin/libunwind.dll");
    if !runtime.is_file() {
        missing.push(format!("libunwind.dll at {}", runtime.display()));
    }
    let wrapper = vapor_home.join("tools/cross/bin").join(if cfg!(windows) {
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

fn llvm_mingw_ucrt_marker_ready(vapor_home: &Path) -> bool {
    let marker = vapor_home.join("tools/llvm-mingw/vapor-toolchain.toml");
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
