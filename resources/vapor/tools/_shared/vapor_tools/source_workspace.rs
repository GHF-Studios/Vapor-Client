static SOURCE_CLONE_SEQUENCE: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

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

fn validate_commit_revision(revision: &str) -> Result<(), String> {
    if matches!(revision.len(), 40 | 64) && revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("--revision must be a full 40- or 64-character hexadecimal commit SHA".to_owned())
    }
}

fn require_success(status: std::process::ExitStatus, operation: &str, source: &str) -> Result<(), String> {
    if status.success() {
        Ok(())
    } else {
        Err(format!("Vapor-managed Git {operation} for {source} failed with {status}"))
    }
}

fn run_git(root: &Path, args: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .status()
        .map_err(|error| format!("Vapor-managed Git could not start: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("Vapor-managed Git {:?} failed with {status}", args))
    }
}

fn checkout_revision(root: &Path, revision: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["checkout", "--detach"])
        .arg(revision)
        .status()
        .map_err(|error| format!("Vapor-managed Git could not start: {error}"))?;
    require_success(status, "checkout", revision)
}

fn fetch_revision(root: &Path, revision: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["fetch", "--no-tags", "--depth=1", "origin"])
        .arg(revision)
        .status()
        .map_err(|error| format!("Vapor-managed Git could not start: {error}"))?;
    require_success(status, "fetch", revision)
}

struct RemoveDirectoryOnDrop(PathBuf);

impl Drop for RemoveDirectoryOnDrop {
    fn drop(&mut self) {
        if self.0.exists() {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
}

fn clone_source(sources_root: &Path, source: SourcePreset, revision: Option<&str>) -> Result<(), String> {
    let target = sources_root.join(source.name);
    if fs::symlink_metadata(&target).is_ok() {
        return Err(format!(
            "source clone already exists: {}\nremove it deliberately before cloning again",
            target.display()
        ));
    }
    println!("Cloning {}", source.name);
    println!("  remote: {}", source.remote);
    println!("  branch: {}", source.branch);
    println!("  target: {}", target.display());
    let stage = reserve_clone_stage(sources_root, source.name)?;
    let _cleanup = RemoveDirectoryOnDrop(stage.clone());
    let mut clone = Command::new("git");
    clone.args(["clone", "--branch", source.branch, "--single-branch"]);
    if revision.is_some() {
        clone.arg("--no-checkout");
    } else {
        clone.arg("--recurse-submodules");
    }
    let status = clone.arg(source.remote).arg(&stage).status()
        .map_err(|error| format!("failed to run Vapor-managed source clone: {error}"))?;
    require_success(status, "clone", source.name)?;

    if let Some(revision) = revision {
        fetch_revision(&stage, revision)
            .map_err(|error| format!("could not fetch pinned source revision {revision}: {error}"))?;
        checkout_revision(&stage, revision)
            .map_err(|error| format!("could not select source revision {revision}: {error}"))?;
        run_git(&stage, &["submodule", "update", "--init", "--recursive"])
            .map_err(|error| format!("could not initialize pinned source submodules: {error}"))?;
    }

    fs::rename(&stage, &target).map_err(io("commit source checkout", &target))?;
    Ok(())
}

fn reserve_clone_stage(sources_root: &Path, name: &str) -> Result<PathBuf, String> {
    for _ in 0..64 {
        let sequence = SOURCE_CLONE_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let stage = sources_root.join(format!(".{name}.clone.{}.{}", std::process::id(), sequence));
        match fs::create_dir(&stage) {
            Ok(()) => return Ok(stage),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(format!("could not reserve temporary source checkout {}: {error}", stage.display())),
        }
    }
    Err(format!("could not reserve a unique temporary checkout for {name}"))
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
    vapor_home: &Path,
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
            vapor_home,
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

fn superworkspace_manifest(name: &str, vapor_home: &Path, mounted: &[SourcePreset]) -> String {
    let mut source = String::new();
    source.push_str("schema = 2\n\n");
    source.push_str("[super-workspace]\n");
    source.push_str(&format!("name = \"{}\"\n", toml_escape(name)));
    source.push_str("sources-directory = \"sources\"\n");
    source.push_str("tools = \"user-data\"\n\n");
    source.push_str("[app-root]\n");
    source.push_str("name = \"Vapor Home\"\n");
    source.push_str(&format!("env-var = \"{}\"\n", VAPOR_HOME_ENV));
    source.push_str(&format!(
        "path = \"{}\"\n\n",
        toml_escape(&vapor_home.to_string_lossy())
    ));
    source.push_str("[tooling]\n");
    source.push_str("location = \"user-data/resources/vapor/tools\"\n");
    source.push_str("rust-toolchain-owner = \"user-data\"\n\n");
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
