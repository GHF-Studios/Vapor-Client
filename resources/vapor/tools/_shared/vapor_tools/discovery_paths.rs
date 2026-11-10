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

fn resolve_app_root(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    let mut candidates = Vec::<(String, PathBuf)>::new();
    if let Some(path) = explicit {
        candidates.push(("--app-root".to_owned(), path));
    }
    if let Ok(value) = env::var(APP_ROOT_ENV) {
        if !value.trim().is_empty() {
            candidates.push((APP_ROOT_ENV.to_owned(), PathBuf::from(value.trim())));
        }
    }
    let anchor = app_root_anchor_file();
    if let Ok(value) = fs::read_to_string(&anchor) {
        if !value.trim().is_empty() {
            candidates.push((anchor.display().to_string(), PathBuf::from(value.trim())));
        }
    }
    for path in candidate_app_roots_from_tool_source() {
        candidates.push(("deployed setup script".to_owned(), path));
    }
    if let Some(path) = candidate_app_root_from_exe() {
        candidates.push(("running executable".to_owned(), path));
    }
    if let Some(home) = env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
        .map(|home| home.join("Vapor"))
    {
        candidates.push(("default Vapor Home".to_owned(), home));
    }
    if let Ok(current) = env::current_dir() {
        candidates.push(("current directory".to_owned(), current.clone()));
        if let Some(path) = find_upward(&current, APP_MANIFEST) {
            candidates.push((
                "nearest app manifest above current directory".to_owned(),
                path,
            ));
        }
    }

    let mut rejected = Vec::new();
    for (source, candidate) in dedupe_candidates(candidates) {
        match canonical_dir(candidate.clone()) {
            Ok(path) => match require_app_root(&path) {
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
        "could not resolve Vapor Home\nchecked:\n  - {}\npass --app-root /path/to/Vapor or set VAPOR_HOME",
        checked
    ))
}

fn candidate_app_roots_from_tool_source() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    for path in tool_source_path_candidates() {
        candidates.extend(candidate_app_roots_from_path(&path));
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

fn candidate_app_roots_from_path(path: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let Some(path) = existing_absolute_path(path) else {
        return candidates;
    };
    for ancestor in path.ancestors() {
        if is_resources_tools_dir(ancestor) {
            if let Some(app_root) = ancestor
                .parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
            {
                candidates.push(app_root.to_path_buf());
            }
        }
        if ancestor.file_name().is_some_and(|name| name == "tools") {
            if let Some(parent) = ancestor.parent() {
                if parent.join(APP_MANIFEST).is_file() {
                    candidates.push(parent.to_path_buf());
                }
            }
        }
    }
    candidates
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

fn is_resources_tools_dir(path: &Path) -> bool {
    path.file_name().is_some_and(|name| name == "tools")
        && path
            .parent()
            .and_then(Path::file_name)
            .is_some_and(|name| name == "vapor")
        && path
            .parent()
            .and_then(Path::parent)
            .and_then(Path::file_name)
            .is_some_and(|name| name == "resources")
}

fn candidate_app_root_from_exe() -> Option<PathBuf> {
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

fn steam_app_manifest_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for steam_root in steam_root_candidates() {
        let steamapps = steam_root.join("steamapps");
        let mut library_roots = vec![steam_root];
        let library_folders = steamapps.join("libraryfolders.vdf");
        if let Ok(source) = fs::read_to_string(&library_folders) {
            library_roots.extend(vdf_values(&source, "path").into_iter().map(PathBuf::from));
        }
        for library_root in dedupe_paths(library_roots) {
            let steamapps = library_root.join("steamapps");
            let manifest = steamapps.join(format!("appmanifest_{STEAM_APP_ID}.acf"));
            if let Ok(source) = fs::read_to_string(&manifest) {
                if let Some(install_dir) = first_vdf_value(&source, "installdir") {
                    roots.push(steamapps.join("common").join(install_dir));
                }
            }
        }
    }
    dedupe_paths(roots)
}

fn steam_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if cfg!(windows) {
        for root in [
            env::var_os("PROGRAMFILES(X86)").map(PathBuf::from),
            env::var_os("PROGRAMFILES").map(PathBuf::from),
        ]
        .into_iter()
        .flatten()
        {
            roots.push(root.join("Steam"));
        }
    } else {
        if let Some(path) = env::var_os("XDG_DATA_HOME") {
            roots.push(PathBuf::from(path).join("Steam"));
        }
        if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
            roots.push(home.join(".local/share/Steam"));
            roots.push(home.join(".steam/steam"));
            roots.push(home.join(".steam/root"));
        }
    }
    dedupe_paths(roots)
}

fn first_vdf_value(source: &str, key: &str) -> Option<String> {
    vdf_values(source, key).into_iter().next()
}

fn vdf_values(source: &str, key: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            let tokens = quoted_tokens(line);
            if tokens.len() >= 2 && tokens[0] == key {
                Some(tokens[1].clone())
            } else {
                None
            }
        })
        .collect()
}

fn quoted_tokens(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;
    for ch in line.chars() {
        if !in_quote {
            if ch == '"' {
                in_quote = true;
                current.clear();
            }
            continue;
        }
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => {
                tokens.push(current.clone());
                current.clear();
                in_quote = false;
            }
            _ => current.push(ch),
        }
    }
    tokens
}

fn common_steam_app_root_candidates() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(path) = env::var_os("STEAM_COMPAT_INSTALL_PATH") {
        roots.push(PathBuf::from(path));
    }
    if cfg!(windows) {
        for root in [
            env::var_os("PROGRAMFILES(X86)").map(PathBuf::from),
            env::var_os("PROGRAMFILES").map(PathBuf::from),
        ]
        .into_iter()
        .flatten()
        {
            roots.push(
                root.join("Steam")
                    .join("steamapps")
                    .join("common")
                    .join(STEAM_APP_DIR_NAME),
            );
        }
    } else {
        if let Some(path) = env::var_os("XDG_DATA_HOME") {
            roots.push(
                PathBuf::from(path)
                    .join("Steam")
                    .join("steamapps")
                    .join("common")
                    .join(STEAM_APP_DIR_NAME),
            );
        }
        if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
            roots.push(
                home.join(".local/share/Steam/steamapps/common")
                    .join(STEAM_APP_DIR_NAME),
            );
            roots.push(
                home.join(".steam/steam/steamapps/common")
                    .join(STEAM_APP_DIR_NAME),
            );
        }
    }
    dedupe_paths(roots)
}

fn require_app_root(path: &Path) -> Result<(), String> {
    if path.join(APP_SOURCE_MANIFEST).is_file() {
        return Err(format!(
            "found {APP_SOURCE_MANIFEST}; this looks like the app source root, not the installed app root"
        ));
    }
    let marker = path.join(APP_MANIFEST);
    if !marker.is_file() {
        return Err(format!("missing {APP_MANIFEST}"));
    }
    let source = fs::read_to_string(&marker).map_err(io("read Vapor Home marker", &marker))?;
    if source.lines().any(|line| line.trim() == "[vapor]")
        && source.lines().any(|line| line.trim() == "schema = 1")
    {
        Ok(())
    } else {
        Err(format!("{} is not a valid Vapor Home marker", marker.display()))
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

fn write_app_root_anchor(app_root: &Path) -> Result<(), String> {
    let anchor = app_root_anchor_file();
    if let Some(parent) = anchor.parent() {
        fs::create_dir_all(parent).map_err(io("create app-root anchor directory", parent))?;
    }
    fs::write(&anchor, app_root.to_string_lossy().as_bytes())
        .map_err(io("write app-root anchor", &anchor))?;
    if cfg!(windows) {
        let _ = Command::new("setx")
            .arg(APP_ROOT_ENV)
            .arg(app_root.to_string_lossy().as_ref())
            .status();
    }
    Ok(())
}

fn remove_app_root_anchor() -> Result<bool, String> {
    let anchor = app_root_anchor_file();
    if anchor.exists() {
        fs::remove_file(&anchor).map_err(io("remove app-root anchor", &anchor))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn app_root_anchor_file() -> PathBuf {
    if cfg!(windows) {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("USERPROFILE").map(|home| PathBuf::from(home).join("AppData/Roaming"))
            })
            .unwrap_or_else(|| PathBuf::from("."))
            .join("loo_cast/app_root.path")
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("loo_cast/app_root.path")
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
                    | "--app-root"
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

fn tool_command(relative: &str, app_root: Option<&Path>) -> String {
    let path = find_tool_script(relative, app_root)
        .unwrap_or_else(|| app_root_tool_path(app_root, relative));
    format!("rust-script --force {}", shell_arg(&path))
}

fn find_tool_script(relative: &str, app_root: Option<&Path>) -> Option<PathBuf> {
    let relative = Path::new(relative);
    tool_root_candidates(app_root)
        .into_iter()
        .map(|root| root.join(relative))
        .find(|path| path.is_file())
}

fn tool_root_candidates(app_root: Option<&Path>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(app_root) = app_root {
        candidates.push(app_root.join("resources/vapor/tools"));
    } else if let Ok(app_root) = resolve_app_root(None) {
        candidates.push(app_root.join("resources/vapor/tools"));
    }
    dedupe_paths(candidates)
}

fn app_root_tool_path(app_root: Option<&Path>, relative: &str) -> PathBuf {
    app_root
        .map(Path::to_path_buf)
        .or_else(|| resolve_app_root(None).ok())
        .unwrap_or_else(|| PathBuf::from("<app-root>"))
        .join("resources/vapor/tools")
        .join(relative)
}

fn expected_rust_bin(app_root: &Path) -> Result<PathBuf, String> {
    Ok(app_root
        .join("tools/rustup/home/toolchains")
        .join(format!("{RUST_TOOLCHAIN}-{}", host_rust_target()?))
        .join("bin"))
}

fn steamcmd_path(app_root: &Path) -> PathBuf {
    let directory = app_root.join("tools/steamcmd");
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
