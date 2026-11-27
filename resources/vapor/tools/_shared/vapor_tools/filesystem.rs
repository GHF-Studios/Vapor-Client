fn reset_dir(vapor_home: &Path, path: &Path) -> Result<(), String> {
    ensure_contained(vapor_home, path)?;
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

fn user_data_root() -> Result<PathBuf, String> {
    vapor_core::user_data_root()
        .ok_or_else(|| "cannot locate Vapor User Data; set the platform home directory".to_owned())
}

fn write_receipt(_app_root: &Path, name: &str, state: &str) -> Result<(), String> {
    let path = user_data_root()?
        .join("state/installer")
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

fn app_control_runner(vapor_home: &Path) -> PathBuf {
    let host_target = host_runtime_target().unwrap_or("x86_64-unknown-linux-gnu");
    vapor_home
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
    let candidate = canonicalize_allow_missing(path)?;
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

/// Resolve every existing path component before checking containment.
///
/// The final leaf often does not exist yet (for example, a tool being
/// installed). Canonicalizing its nearest existing parent gives Windows paths
/// the same extended-length prefix as the canonical root, while still
/// resolving existing symlinks before the untrusted suffix is appended.
fn canonicalize_allow_missing(path: &Path) -> Result<PathBuf, String> {
    let absolute = normalized_absolute(path)?;
    let mut ancestor = absolute.as_path();
    let mut missing = Vec::new();

    loop {
        match fs::canonicalize(ancestor) {
            Ok(mut resolved) => {
                for component in missing.iter().rev() {
                    resolved.push(component);
                }
                return Ok(resolved);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if fs::symlink_metadata(ancestor).is_ok() {
                    return Err(format!(
                        "cannot resolve existing path component '{}' safely",
                        ancestor.display()
                    ));
                }
                let component = ancestor.file_name().ok_or_else(|| {
                    format!("cannot find an existing parent for '{}': {error}", path.display())
                })?;
                missing.push(component.to_os_string());
                ancestor = ancestor.parent().ok_or_else(|| {
                    format!("cannot find an existing parent for '{}': {error}", path.display())
                })?;
            }
            Err(error) => {
                return Err(format!(
                    "failed to resolve existing path component '{}': {error}",
                    ancestor.display()
                ));
            }
        }
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
    // `canonicalize` produces Windows verbatim paths (`\\?\C:\...`).
    // Windows PowerShell 5.1's filesystem provider rejects those in commands
    // such as Invoke-WebRequest -OutFile even though Rust's filesystem APIs
    // accept them. Keep the long-path form for Rust I/O, but normalize paths
    // whenever they cross this PowerShell boundary.
    value
        .strip_prefix(r"\\?\")
        .unwrap_or(value)
        .replace('\'', "''")
}

fn powershell_compatible_path_text(value: &str) -> String {
    if let Some(unc_path) = value.strip_prefix(r"\\?\UNC\") {
        return format!(r"\\{unc_path}");
    }

    let bytes = value.as_bytes();
    if bytes.len() >= 7
        && value.starts_with(r"\\?\")
        && bytes[4].is_ascii_alphabetic()
        && bytes[5] == b':'
        && matches!(bytes[6], b'\\' | b'/')
    {
        return value[4..].to_owned();
    }

    value.to_owned()
}

#[cfg(test)]
mod tests {
    use super::powershell_compatible_path_text;

    #[test]
    fn powershell_paths_drop_extended_drive_prefixes() {
        assert_eq!(
            powershell_compatible_path_text(r"\\?\C:\Users\Leslie\Vapor\data\downloads\tool.zip"),
            r"C:\Users\Leslie\Vapor\data\downloads\tool.zip"
        );
    }

    #[test]
    fn powershell_paths_convert_extended_unc_prefixes() {
        assert_eq!(
            powershell_compatible_path_text(r"\\?\UNC\server\share\Vapor\tool.zip"),
            r"\\server\share\Vapor\tool.zip"
        );
    }
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
