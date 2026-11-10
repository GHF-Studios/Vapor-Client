fn reset_dir(app_root: &Path, path: &Path) -> Result<(), String> {
    ensure_contained(app_root, path)?;
    reset_plain_dir(path)
}

fn reset_plain_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        remove_path(path).map_err(io("remove directory", path))?;
    }
    fs::create_dir_all(path).map_err(io("create directory", path))
}

fn remove_path(path: &Path) -> std::io::Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn remove_empty_dir(root: &Path, path: &Path) -> Result<(), String> {
    ensure_contained(root, path)?;
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Ok(());
    }
    match fs::remove_dir(path) {
        Ok(()) => Ok(()),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
            ) =>
        {
            Ok(())
        }
        Err(error) => Err(format!(
            "failed to remove empty directory '{}': {error}",
            path.display()
        )),
    }
}

fn write_receipt(app_root: &Path, name: &str, state: &str) -> Result<(), String> {
    let path = app_root
        .join("data/state/installer")
        .join(format!("{name}.toml"));
    let body = format!(
        "schema = 1\nstate = \"{state}\"\ntool = \"resources/vapor/tools/production/app_setup/{name}\"\n"
    );
    write(&path, &body)
}

fn write(path: &Path, source: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io("create parent directory", parent))?;
    }
    let mut file = fs::File::create(path).map_err(io("create file", path))?;
    file.write_all(source.as_bytes())
        .map_err(io("write file", path))
}

fn make_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path)
            .map_err(io("read executable metadata", path))?
            .permissions();
        permissions.set_mode(permissions.mode() | 0o755);
        fs::set_permissions(path, permissions).map_err(io("set executable bit", path))?;
    }
    Ok(())
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path).is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        true
    }
}

fn executable(name: &str) -> String {
    format!("{name}{}", env::consts::EXE_SUFFIX)
}

fn app_control_runner(app_root: &Path) -> PathBuf {
    let host_target = host_rust_target().unwrap_or("x86_64-unknown-linux-gnu");
    app_root
        .join("bin")
        .join(host_target)
        .join(executable("vapor-installer"))
}

fn canonical_dir(path: PathBuf) -> Result<PathBuf, String> {
    let path = fs::canonicalize(&path)
        .map_err(|error| format!("failed to resolve '{}': {error}", path.display()))?;
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!("not a directory: {}", path.display()))
    }
}

fn ensure_contained(root: &Path, path: &Path) -> Result<(), String> {
    let root = fs::canonicalize(root).map_err(io("resolve containment root", root))?;
    let candidate = normalized_absolute(path)?;
    if candidate.starts_with(&root) {
        Ok(())
    } else {
        Err(format!(
            "path '{}' escapes root '{}'",
            candidate.display(),
            root.display()
        ))
    }
}

fn normalized_absolute(path: &Path) -> Result<PathBuf, String> {
    use std::path::Component;

    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|error| format!("failed to read current dir: {error}"))?
            .join(path)
    };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Ok(normalized)
}

fn relative_path(root: &Path, path: &Path) -> Result<PathBuf, String> {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .map_err(|_| format!("path '{}' is outside '{}'", path.display(), root.display()))
}

fn shell_arg(path: &Path) -> String {
    shell_arg_text(&path.to_string_lossy())
}

fn shell_arg_text(text: &str) -> String {
    if text
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "/._-".contains(ch))
    {
        text.to_owned()
    } else {
        format!("'{}'", text.replace('\'', "'\\''"))
    }
}

fn shell_string(path: &Path) -> String {
    shell_string_text(&path.to_string_lossy())
}

fn shell_string_text(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn powershell_quote(value: &str) -> String {
    value.replace('\'', "''")
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn io<'a>(action: &'a str, path: &'a Path) -> impl Fn(std::io::Error) -> String + 'a {
    move |error| format!("failed to {action} '{}': {error}", path.display())
}
