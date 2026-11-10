fn print_component(status: &ComponentStatus) {
    println!(
        "{}: {}",
        status.label,
        if status.ready { "ready" } else { "missing" }
    );
    println!("  path: {}", status.path.display());
    for missing in &status.missing {
        println!("  missing: {missing}");
    }
}

fn super_workspace_root() -> Result<PathBuf, String> {
    if let Some(path) = arg_path("--super-workspace") {
        return canonical_dir(path);
    }
    if let Some(path) = env::var_os("VAPOR_SUPER_WORKSPACE_ROOT") {
        return canonical_dir(PathBuf::from(path));
    }
    let current =
        env::current_dir().map_err(|error| format!("failed to read current dir: {error}"))?;
    find_upward(&current, SUPER_MANIFEST).ok_or_else(|| {
        format!(
            "could not find {SUPER_MANIFEST}; run from inside the Loo-Cast/Vapor super-workspace or set VAPOR_SUPER_WORKSPACE_ROOT"
        )
    })
}

fn resolve_super_workspace_root(explicit: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(path) = explicit {
        return canonical_dir(path.to_path_buf());
    }
    super_workspace_root()
}

fn find_upward(start: &Path, marker: &str) -> Option<PathBuf> {
    let mut current = start;
    for _ in 0..12 {
        if current.join(marker).is_file() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
    None
}

fn resolve_vapor_home(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    let mut candidates = Vec::<(String, PathBuf)>::new();
    if let Some(path) = explicit {
        let path = canonical_dir(path).map_err(|error| {
            format!("the explicit --vapor-home path is invalid: {error}")
        })?;
        require_vapor_home(&path).map_err(|error| {
            format!("the explicit --vapor-home path is not a Vapor home: {error}")
        })?;
        return Ok(path);
    }
    if let Ok(value) = env::var(VAPOR_HOME_ENV) {
        if !value.trim().is_empty() {
            let path = canonical_dir(PathBuf::from(value.trim())).map_err(|error| {
                format!("{VAPOR_HOME_ENV} points to an invalid Vapor home: {error}")
            })?;
            require_vapor_home(&path).map_err(|error| {
                format!("{VAPOR_HOME_ENV} does not identify a Vapor home: {error}")
            })?;
            return Ok(path);
        }
    }
    let anchor = vapor_home_anchor_file();
    if let Ok(value) = fs::read_to_string(&anchor) {
        if !value.trim().is_empty() {
            candidates.push((anchor.display().to_string(), PathBuf::from(value.trim())));
        }
    }
    for path in candidate_vapor_homes_from_tool_source() {
        candidates.push(("deployed setup script".to_owned(), path));
    }
    if let Some(path) = candidate_vapor_home_from_exe() {
        candidates.push(("running executable".to_owned(), path));
    }
    if let Some(home) = env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
    {
        candidates.push(("default Vapor home".to_owned(), home.join("Vapor")));
    }
    if let Ok(current) = env::current_dir() {
        candidates.push(("current directory".to_owned(), current.clone()));
        if let Some(path) = find_upward(&current, "Vapor-Home.toml") {
            candidates.push((
                "nearest Vapor home above current directory".to_owned(),
                path,
            ));
        }
    }

    let mut rejected = Vec::new();
    for (source, candidate) in dedupe_candidates(candidates) {
        match canonical_dir(candidate.clone()) {
            Ok(path) => match require_vapor_home(&path) {
                Ok(()) => return Ok(path),
                Err(error) => rejected.push(format!("{source}: {} ({error})", candidate.display())),
            },
            Err(error) => rejected.push(format!("{source}: {} ({error})", candidate.display())),
        }
    }
    let checked = if rejected.is_empty() {
        "(no candidates)".to_owned()
    } else {
        rejected.join("\n  - ")
    };

    Err(format!(
        "could not resolve Vapor home\nchecked:\n  - {}\npass --vapor-home /path/to/Vapor or install Vapor",
        checked
    ))
}

fn candidate_vapor_homes_from_tool_source() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    for path in tool_source_path_candidates() {
        let Some(path) = existing_absolute_path(&path) else {
            continue;
        };
        candidates.extend(
            path.ancestors()
                .filter(|ancestor| ancestor.join("Vapor-Home.toml").is_file())
                .map(Path::to_path_buf),
        );
    }
    dedupe_paths(candidates)
}

fn tool_source_path_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    candidates.push(PathBuf::from(file!()));
    for arg in env::args_os().take(3) {
        candidates.push(PathBuf::from(arg));
    }
    let mut expanded = Vec::new();
    for candidate in candidates {
        expanded.push(candidate.clone());
        if candidate.is_relative() {
            if let Ok(current) = env::current_dir() {
                expanded.push(current.join(candidate));
            }
        }
    }
    dedupe_paths(expanded)
}

fn existing_absolute_path(path: &Path) -> Option<PathBuf> {
    if let Ok(path) = fs::canonicalize(path) {
        return Some(path);
    }
    if path.is_relative() {
        if let Ok(current) = env::current_dir() {
            if let Ok(path) = fs::canonicalize(current.join(path)) {
                return Some(path);
            }
        }
    }
    None
}

fn candidate_vapor_home_from_exe() -> Option<PathBuf> {
    let executable = env::current_exe().ok()?;
    let directory = executable.parent()?;
    if directory.file_name().is_some_and(|name| name == "bin") {
        return directory.parent().map(Path::to_path_buf);
    }
    if directory
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|name| name == "bin")
    {
        return directory
            .parent()
            .and_then(Path::parent)
            .map(Path::to_path_buf);
    }
    None
}

fn require_vapor_home(path: &Path) -> Result<(), String> {
    let marker = path.join("Vapor-Home.toml");
    if !marker.is_file() {
        return Err("missing Vapor-Home.toml".to_owned());
    }
    let source = fs::read_to_string(&marker).map_err(io("read Vapor home marker", &marker))?;
    let mut lines = source.lines().map(str::trim);
    if lines.next() == Some("schema = 1")
        && lines.next() == Some("")
        && lines.next() == Some("[vapor]")
        && lines.next() == Some("id = \"ghf-studios/vapor\"")
        && lines.next().is_none()
    {
        Ok(())
    } else {
        Err(format!("{} does not identify the supported Vapor home", marker.display()))
    }
}

fn dedupe_candidates(candidates: Vec<(String, PathBuf)>) -> Vec<(String, PathBuf)> {
    let mut deduped = Vec::new();
    for (source, path) in candidates {
        let key = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !deduped.iter().any(|(_, existing): &(String, PathBuf)| {
            fs::canonicalize(existing).unwrap_or_else(|_| existing.clone()) == key
        }) {
            deduped.push((source, path));
        }
    }
    deduped
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths {
        let key = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !deduped.iter().any(|existing: &PathBuf| {
            fs::canonicalize(existing).unwrap_or_else(|_| existing.clone()) == key
        }) {
            deduped.push(path);
        }
    }
    deduped
}

fn write_vapor_home_anchor(vapor_home: &Path) -> Result<(), String> {
    let anchor = vapor_home_anchor_file();
    if let Some(parent) = anchor.parent() {
        fs::create_dir_all(parent).map_err(io("create Vapor home anchor directory", parent))?;
    }
    fs::write(&anchor, vapor_home.to_string_lossy().as_bytes())
        .map_err(io("write Vapor home anchor", &anchor))?;
    if cfg!(windows) {
        let _ = Command::new("setx")
            .arg(VAPOR_HOME_ENV)
            .arg(vapor_home.to_string_lossy().as_ref())
            .status();
    }
    Ok(())
}

fn remove_vapor_home_anchor() -> Result<bool, String> {
    let anchor = vapor_home_anchor_file();
    if anchor.exists() {
        fs::remove_file(&anchor).map_err(io("remove Vapor-home anchor", &anchor))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn vapor_home_anchor_file() -> PathBuf {
    if cfg!(windows) {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("USERPROFILE").map(|home| PathBuf::from(home).join("AppData/Roaming"))
            })
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Vapor/home.path")
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("Vapor/home.path")
    }
}

fn arg_path(flag: &str) -> Option<PathBuf> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next().map(PathBuf::from);
        }
    }
    None
}

fn arg_value(flag: &str) -> Option<String> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next();
        }
    }
    None
}

fn has_flag(flag: &str) -> bool {
    env::args().skip(1).any(|arg| arg == flag)
}

fn positional_path() -> Option<PathBuf> {
    positional_values().first().map(PathBuf::from)
}

fn positional_values() -> Vec<String> {
    let mut values = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            if matches!(
                arg.as_str(),
                "--path"
                    | "--vapor-home"
                    | "--super-workspace"
                    | "--workflow"
                    | "--workdir"
                    | "--config-name"
                    | "--open-konsole"
            ) {
                let _ = args.next();
            }
            continue;
        }
        values.push(arg);
    }
    values
}

fn prompt(label: &str) -> Result<String, String> {
    print!("{label}");
    stdout()
        .flush()
        .map_err(|error| format!("failed to flush prompt: {error}"))?;
    let mut value = String::new();
    stdin()
        .read_line(&mut value)
        .map_err(|error| format!("failed to read prompt input: {error}"))?;
    Ok(value.trim_end_matches(['\r', '\n']).to_owned())
}

fn is_yes(value: &str) -> bool {
    matches!(
        value.trim(),
        "y" | "Y" | "yes" | "YES" | "Yes" | "ye" | "YE" | "Ye"
    )
}

fn absolute_from_current(path: PathBuf) -> Result<PathBuf, String> {
    if path.is_absolute() {
        return Ok(path);
    }
    let current =
        env::current_dir().map_err(|error| format!("failed to read current dir: {error}"))?;
    Ok(current.join(path))
}

fn tool_command(relative: &str, vapor_home: Option<&Path>) -> String {
    let path = find_tool_script(relative, vapor_home)
        .unwrap_or_else(|| app_root_tool_path(vapor_home, relative));
    format!("rust-script --force {}", shell_arg(&path))
}

fn find_tool_script(relative: &str, vapor_home: Option<&Path>) -> Option<PathBuf> {
    let relative = Path::new(relative);
    tool_root_candidates(vapor_home)
        .into_iter()
        .map(|root| root.join(relative))
        .find(|path| path.is_file())
}

fn tool_root_candidates(vapor_home: Option<&Path>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(vapor_home) = vapor_home {
        candidates.push(vapor_home.join("resources/vapor/tools"));
    } else if let Ok(vapor_home) = resolve_vapor_home(None) {
        candidates.push(vapor_home.join("resources/vapor/tools"));
    }
    dedupe_paths(candidates)
}

fn app_root_tool_path(vapor_home: Option<&Path>, relative: &str) -> PathBuf {
    vapor_home
        .map(Path::to_path_buf)
        .or_else(|| resolve_vapor_home(None).ok())
        .unwrap_or_else(|| PathBuf::from("<Vapor-home>"))
        .join("resources/vapor/tools")
        .join(relative)
}

fn expected_rust_bin(vapor_home: &Path) -> Result<PathBuf, String> {
    Ok(vapor_home
        .join("tools/rustup/home/toolchains")
        .join(format!("{RUST_TOOLCHAIN}-{}", host_rust_target()?))
        .join("bin"))
}

fn steamcmd_path(vapor_home: &Path) -> PathBuf {
    let directory = vapor_home.join("tools/steamcmd");
    let candidates = if cfg!(windows) {
        vec![directory.join("steamcmd.exe")]
    } else {
        vec![directory.join("steamcmd"), directory.join("steamcmd.sh")]
    };
    candidates
        .into_iter()
        .find(|path| is_executable(path))
        .unwrap_or_else(|| directory.join(executable("steamcmd")))
}

fn rustup_init_url() -> Result<&'static str, String> {
    match (env::consts::ARCH, env::consts::OS) {
        ("x86_64", "linux") => Ok(RUSTUP_INIT_X86_64_LINUX),
        ("aarch64", "linux") => Ok(RUSTUP_INIT_AARCH64_LINUX),
        ("x86_64", "windows") => Ok(RUSTUP_INIT_X86_64_WINDOWS),
        _ => Err(format!(
            "rustup bootstrap is not configured for {}-{}",
            env::consts::ARCH,
            env::consts::OS
        )),
    }
}

fn host_rust_target() -> Result<&'static str, String> {
    match (env::consts::ARCH, env::consts::OS) {
        ("x86_64", "linux") => Ok("x86_64-unknown-linux-gnu"),
        ("aarch64", "linux") => Ok("aarch64-unknown-linux-gnu"),
        ("x86_64", "windows") => Ok("x86_64-pc-windows-msvc"),
        _ => Err(format!(
            "Rust host target is not configured for {}-{}",
            env::consts::ARCH,
            env::consts::OS
        )),
    }
}
