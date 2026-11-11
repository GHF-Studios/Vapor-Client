fn source_preset(name: &str) -> Result<SourcePreset, String> {
    SOURCE_PRESETS
        .iter()
        .copied()
        .find(|source| source.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| {
            format!(
                "unknown source '{name}'; known: {}",
                SOURCE_PRESETS
                    .iter()
                    .map(|source| source.name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

fn clone_source(sources_root: &Path, source: SourcePreset) -> Result<(), String> {
    let target = sources_root.join(source.name);
    if target.exists() {
        return Err(format!(
            "source clone already exists: {}\nremove it deliberately before cloning again",
            target.display()
        ));
    }
    println!("Cloning {}", source.name);
    println!("  remote: {}", source.remote);
    println!("  branch: {}", source.branch);
    println!("  target: {}", target.display());
    let status = Command::new("git")
        .args([
            "clone",
            "--recurse-submodules",
            "--branch",
            source.branch,
            source.remote,
        ])
        .arg(git_worktree_path(&target))
        .status()
        .map_err(|error| format!("failed to run git clone: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "git clone for {} failed with {status}",
            source.name
        ))
    }
}

fn git_worktree_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::{OsStrExt, OsStringExt};

        let wide = path.as_os_str().encode_wide().collect::<Vec<_>>();
        let extended_unc = r"\\?\UNC\".encode_utf16().collect::<Vec<_>>();
        if wide.starts_with(&extended_unc) {
            let mut compatible = r"\\".encode_utf16().collect::<Vec<_>>();
            compatible.extend_from_slice(&wide[extended_unc.len()..]);
            return PathBuf::from(std::ffi::OsString::from_wide(&compatible));
        }
        let extended = r"\\?\".encode_utf16().collect::<Vec<_>>();
        if wide.starts_with(&extended) {
            return PathBuf::from(std::ffi::OsString::from_wide(&wide[extended.len()..]));
        }
    }

    path.to_path_buf()
}

fn record_source_clone(
    super_root: &Path,
    app_root: &Path,
    source: SourcePreset,
) -> Result<(), String> {
    let manifest = super_root.join(SUPER_MANIFEST);
    let source_path = format!("{SOURCES_DIR}/{}", source.name);
    let mut mounted = mounted_sources(&manifest)?;
    if !mounted.iter().any(|name| name == source.name) {
        mounted.push(source.name.to_owned());
    }
    write(
        &manifest,
        &superworkspace_manifest(
            &superworkspace_name(super_root),
            app_root,
            &mounted
                .iter()
                .map(|name| source_preset(name))
                .collect::<Result<Vec<_>, _>>()?,
        ),
    )?;
    println!("  recorded: {source_path}");
    Ok(())
}

fn mounted_sources(manifest: &Path) -> Result<Vec<String>, String> {
    let source =
        fs::read_to_string(manifest).map_err(io("read super-workspace manifest", manifest))?;
    let mut names = Vec::new();
    let mut in_source = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "[[sources]]" {
            in_source = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_source = false;
        }
        if in_source {
            if let Some(value) = trimmed.strip_prefix("name = ") {
                if let Some(name) = parse_toml_string(value) {
                    names.push(name);
                }
            }
        }
    }
    Ok(names)
}

fn superworkspace_name(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "vapor-superworkspace".to_owned())
}

fn superworkspace_manifest(name: &str, app_root: &Path, mounted: &[SourcePreset]) -> String {
    let mut source = String::new();
    source.push_str("schema = 2\n\n");
    source.push_str("[super-workspace]\n");
    source.push_str(&format!("name = \"{}\"\n", toml_escape(name)));
    source.push_str("sources-directory = \"sources\"\n");
    source.push_str("tools = \"app-root\"\n\n");
    source.push_str("[app-root]\n");
    source.push_str("name = \"Vapor Home\"\n");
    source.push_str(&format!("env-var = \"{}\"\n", APP_ROOT_ENV));
    source.push_str(&format!(
        "path = \"{}\"\n\n",
        toml_escape(&app_root.to_string_lossy())
    ));
    source.push_str("[tooling]\n");
    source.push_str("location = \"app-root/tools\"\n");
    source.push_str("rust-toolchain-owner = \"app-root\"\n\n");
    for source_clone in mounted {
        source.push_str("[[sources]]\n");
        source.push_str(&format!("name = \"{}\"\n", source_clone.name));
        source.push_str(&format!("role = \"{}\"\n", source_clone.role));
        source.push_str(&format!("path = \"{SOURCES_DIR}/{}\"\n", source_clone.name));
        source.push_str(&format!("remote = \"{}\"\n", source_clone.remote));
        source.push_str(&format!("branch = \"{}\"\n\n", source_clone.branch));
    }
    source.push_str("[source-presets]\n");
    source.push_str("operation = \"source-clone\"\n");
    source
}

fn parse_toml_string(value: &str) -> Option<String> {
    let value = value.trim();
    let value = value.strip_prefix('"')?.strip_suffix('"')?;
    Some(value.replace("\\\"", "\"").replace("\\\\", "\\"))
}

fn toml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
