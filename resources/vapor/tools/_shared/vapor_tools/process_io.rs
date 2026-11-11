fn download(url: &str, destination: &Path) -> Result<(), String> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(io("create download directory", parent))?;
    }
    println!("download {url}");
    if cfg!(windows) {
        run(Command::new("powershell")
            .args(["-NoProfile", "-Command"])
            .arg(format!(
                "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                powershell_quote(url),
                powershell_quote(&destination.to_string_lossy())
            )))
    } else {
        run(Command::new("curl")
            .args(["--fail", "--location", "--show-error", "--output"])
            .arg(destination)
            .arg(url))
    }
}

fn verify_sha256(path: &Path, expected: &str) -> Result<(), String> {
    if cfg!(windows) {
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command"])
            .arg(format!(
                "(Get-FileHash -Algorithm SHA256 '{}').Hash",
                powershell_quote(&path.to_string_lossy())
            ))
            .output()
            .map_err(|error| format!("failed to start PowerShell sha256 check: {error}"))?;
        let actual = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_ascii_lowercase();
        if output.status.success() && actual == expected {
            return Ok(());
        }
    } else {
        let output = Command::new("sha256sum")
            .arg(path)
            .output()
            .map_err(|error| format!("failed to start sha256sum: {error}"))?;
        let actual = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if output.status.success() && actual == expected {
            return Ok(());
        }
    }
    Err(format!(
        "checksum mismatch for {}; expected {expected}",
        path.display()
    ))
}

fn extract_tar_xz_strip(archive: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target).map_err(io("create extract target", target))?;
    run(Command::new("tar")
        .arg("-xJf")
        .arg(archive)
        .arg("-C")
        .arg(target)
        .arg("--strip-components=1"))
}

fn extract_zip(archive: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target).map_err(io("create extract target", target))?;
    run(Command::new("powershell")
        .args(["-NoProfile", "-Command"])
        .arg(format!(
            "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
            powershell_quote(&archive.to_string_lossy()),
            powershell_quote(&target.to_string_lossy())
        )))
}

fn extract_zip_single_root(archive: &Path, target: &Path) -> Result<(), String> {
    let staging = target.with_extension("extracting");
    reset_plain_dir(&staging)?;
    extract_zip(archive, &staging)?;
    let mut entries = fs::read_dir(&staging)
        .map_err(io("read zip staging", &staging))?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.path());
    if entries.len() == 1 && entries[0].file_type().is_ok_and(|kind| kind.is_dir()) {
        let single_root = entries[0].path();
        if target.exists() {
            remove_path(target).map_err(io("remove old extract target", target))?;
        }
        fs::rename(&single_root, target).map_err(io("promote zip single root", target))?;
        remove_path(&staging).map_err(io("remove zip staging", &staging))?;
        Ok(())
    } else {
        if target.exists() {
            remove_path(target).map_err(io("remove old extract target", target))?;
        }
        fs::rename(&staging, target).map_err(io("promote zip staging", target))
    }
}

fn run(command: &mut Command) -> Result<(), String> {
    let rendered = format!("{command:?}");
    let status = command
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("failed to start {rendered}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{rendered} exited with {status}"))
    }
}

fn run_status(command: &mut Command) -> Result<(), String> {
    let rendered = format!("{command:?}");
    let status = command
        .status()
        .map_err(|error| format!("failed to start {rendered}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{rendered} exited with {status}"))
    }
}

fn require_executable(path: &Path, label: &str) -> Result<(), String> {
    if is_executable(path) {
        Ok(())
    } else {
        Err(format!(
            "missing {label}: {}\nrun the app build/publish-local workflow to refresh installed binaries",
            path.display()
        ))
    }
}

fn command_exists(name: &str) -> bool {
    env::var_os("PATH").is_some_and(|paths| {
        env::split_paths(&paths).any(|path| is_executable(&path.join(executable(name))))
    })
}

fn prepend_path(paths: &[PathBuf]) {
    let mut all = paths
        .iter()
        .filter(|path| path.exists())
        .cloned()
        .collect::<Vec<_>>();
    if let Some(existing) = env::var_os("PATH") {
        all.extend(env::split_paths(&existing));
    }
    if let Ok(joined) = env::join_paths(all) {
        set_env_var("PATH", joined);
    }
}

fn set_env_var<K, V>(key: K, value: V)
where
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    // These tools are single-threaded command-line entrypoints. Environment
    // mutation happens before child commands are spawned so app-root tools and
    // toolchains are resolved consistently.
    unsafe {
        env::set_var(key, value);
    }
}

fn command_stdout(command: &mut Command) -> Result<String, String> {
    let rendered = format!("{command:?}");
    let output = command
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("failed to start {rendered}: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{rendered} exited with {}\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr
        ))
    }
}
