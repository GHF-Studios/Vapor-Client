pub fn status_main() -> Result<(), String> {
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home"))?;
    println!("Vapor Home Tool Status");
    println!();
    println!("Vapor home");
    println!("  {}", vapor_home.display());
    if let Ok(super_root) = super_workspace_root() {
        println!("Super workspace");
        println!("  {}", super_root.display());
    }
    println!();
    print_component(&directories_status(&vapor_home));
    print_component(&steamcmd_status(&vapor_home));
    print_component(&rust_status(&vapor_home));
    print_component(&cross_status(&vapor_home));
    println!();
    println!("Next");
    if !directories_status(&vapor_home).ready {
        println!(
            "  {} install --vapor-home {}",
            shell_arg(&app_control_runner(&vapor_home)),
            shell_arg(&vapor_home)
        );
    } else if !steamcmd_status(&vapor_home).ready
        || !rust_status(&vapor_home).ready
        || !cross_status(&vapor_home).ready
    {
        println!(
            "  {} dev-env install --vapor-home {}",
            shell_arg(&app_control_runner(&vapor_home)),
            shell_arg(&vapor_home)
        );
    } else if let Ok(super_root) = super_workspace_root() {
        println!(
            "  {} ide patch-rustrover --super-workspace {} --vapor-home {}",
            shell_arg(&app_control_runner(&vapor_home)),
            shell_arg(&super_root),
            shell_arg(&vapor_home)
        );
    } else {
        println!(
            "  {} ide patch-rustrover --super-workspace /path/to/SuperWorkspace --vapor-home {}",
            shell_arg(&app_control_runner(&vapor_home)),
            shell_arg(&vapor_home)
        );
    }
    Ok(())
}

pub fn setup_player_main() -> Result<(), String> {
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home"))?;
    setup_player(&vapor_home)
}

pub fn setup_development_main() -> Result<(), String> {
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home"))?;
    setup_development(&vapor_home)
}

pub fn teardown_player_main() -> Result<(), String> {
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home"))?;
    teardown_player(&vapor_home)
}

pub fn teardown_development_main() -> Result<(), String> {
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home"))?;
    teardown_development(&vapor_home)
}

pub fn post_install_main() -> Result<(), String> {
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home").or_else(candidate_vapor_home_from_exe))?;
    write_vapor_home_anchor(&vapor_home)?;
    println!("Post Install");
    println!();
    println!("Vapor home anchor");
    println!("  {VAPOR_HOME_ENV}={}", vapor_home.display());
    println!("  {}", vapor_home_anchor_file().display());
    Ok(())
}

pub fn pre_uninstall_main() -> Result<(), String> {
    let removed = remove_vapor_home_anchor()?;
    println!("Pre Uninstall");
    println!();
    println!(
        "Vapor home anchor: {}",
        if removed { "removed" } else { "already absent" }
    );
    Ok(())
}

pub fn create_superworkspace_main() -> Result<(), String> {
    let root = arg_path("--path")
        .or_else(positional_path)
        .ok_or_else(|| "usage: create.rs --path /path/to/SuperWorkspace".to_owned())?;
    create_superworkspace_at(&root, arg_path("--vapor-home").as_deref())
}

pub fn create_superworkspace_at(
    root: &Path,
    explicit_app_root: Option<&Path>,
) -> Result<(), String> {
    let vapor_home = resolve_vapor_home(
        explicit_app_root
            .map(Path::to_path_buf)
            .or_else(candidate_vapor_home_from_exe),
    )?;
    let root = root.to_path_buf();
    let root = absolute_from_current(root)?;
    if root.exists() && !root.is_dir() {
        return Err(format!(
            "super-workspace target exists and is not a directory: {}",
            root.display()
        ));
    }
    fs::create_dir_all(&root).map_err(io("create super-workspace", &root))?;
    let manifest = root.join(SUPER_MANIFEST);
    if manifest.exists() {
        return Err(format!(
            "super-workspace already exists: {}\nrefusing to rewrite it",
            manifest.display()
        ));
    }
    let sources = root.join(SOURCES_DIR);
    fs::create_dir_all(&sources).map_err(io("create super-workspace sources", &sources))?;
    let name = root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "vapor-superworkspace".to_owned());
    write(&manifest, &superworkspace_manifest(&name, &vapor_home, &[]))?;
    println!("SuperWorkspace created");
    println!();
    println!("root:    {}", root.display());
    println!("manifest: {}", manifest.display());
    println!("sources: {}", sources.display());
    println!("Vapor home: {}", vapor_home.display());
    println!();
    println!("Next");
    println!(
        "  {} source clone SOURCE_NAME --super-workspace {} --vapor-home {}",
        shell_arg(&app_control_runner(&vapor_home)),
        shell_arg(&root),
        shell_arg(&vapor_home)
    );
    println!("  clone only the source you want to work on");
    println!(
        "  {} ide patch-rustrover --super-workspace {} --vapor-home {}",
        shell_arg(&app_control_runner(&vapor_home)),
        shell_arg(&root),
        shell_arg(&vapor_home)
    );
    Ok(())
}

pub fn clone_source_main() -> Result<(), String> {
    if has_flag("--all") {
        return Err(
            "bulk source cloning is intentionally unsupported; clone exactly one source by name"
                .to_owned(),
        );
    }
    let requested = positional_values();
    if requested.len() != 1 {
        return Err(format!(
            "usage: clone.rs --super-workspace /path/to/SuperWorkspace SOURCE_NAME\nknown sources: {}",
            SOURCE_PRESETS
                .iter()
                .map(|source| source.name)
                .collect::<Vec<_>>()
                .join("|")
        ));
    }
    clone_source_into_superworkspace(None, arg_path("--vapor-home").as_deref(), &requested[0])
}

pub fn clone_source_into_superworkspace(
    explicit_super_root: Option<&Path>,
    explicit_app_root: Option<&Path>,
    source_name: &str,
) -> Result<(), String> {
    let vapor_home = resolve_vapor_home(
        explicit_app_root
            .map(Path::to_path_buf)
            .or_else(candidate_vapor_home_from_exe),
    )?;
    let super_root = resolve_super_workspace_root(explicit_super_root)?;
    let source = source_preset(source_name)?;
    let sources_root = super_root.join(SOURCES_DIR);
    fs::create_dir_all(&sources_root)
        .map_err(io("create source clone directory", &sources_root))?;
    clone_source(&sources_root, source)?;
    record_source_clone(&super_root, &vapor_home, source)?;
    println!();
    println!("Source clone ready under {}", sources_root.display());
    Ok(())
}

pub fn ide_run_main() -> Result<(), String> {
    let workflow = arg_value("--workflow")
        .ok_or_else(|| "usage: run.rs --workflow WORKFLOW --super-workspace PATH".to_owned())?;
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home").or_else(candidate_vapor_home_from_exe))?;
    let super_root = super_workspace_root()?;
    let workdir = arg_path("--workdir").unwrap_or_else(|| vapor_home.clone());
    let config_name = arg_value("--config-name").unwrap_or_else(|| workflow.clone());
    let open_konsole = arg_value("--open-konsole").is_some_and(|value| value == "true");

    if open_konsole && env::var_os("VAPOR_RUN_INSIDE_KONSOLE").is_none() {
        if command_exists("konsole") {
            let runner = ide_runner_executable()?;
            let mut command = Command::new("konsole");
            command
                .arg("--workdir")
                .arg(&workdir)
                .arg("-e")
                .arg(runner)
                .args(env::args_os().skip(1))
                .env("VAPOR_RUN_INSIDE_KONSOLE", "1");
            let status = command
                .status()
                .map_err(|error| format!("failed to open Konsole: {error}"))?;
            return if status.success() {
                Ok(())
            } else {
                Err(format!("Konsole workflow exited with {status}"))
            };
        }
        println!("Konsole is not available; running inside the current terminal.");
    }

    let host_target = host_rust_target().unwrap_or("x86_64-unknown-linux-gnu");
    let app_bin_dir = vapor_home.join("bin").join(host_target);
    let vapor = app_bin_dir.join(executable("vapor"));
    let installer = app_control_runner(&vapor_home);
    let entrypoint = app_bin_dir.join(executable("vapor-entrypoint"));

    set_env_var(VAPOR_HOME_ENV, &vapor_home);
    set_env_var("RUSTUP_HOME", vapor_home.join("tools/rustup/home"));
    set_env_var("CARGO_HOME", vapor_home.join("tools/cargo/home"));
    prepend_path(&[
        app_bin_dir.clone(),
        vapor_home
            .join("tools/rustup/home/toolchains")
            .join(format!("{RUST_TOOLCHAIN}-{host_target}"))
            .join("bin"),
        vapor_home.join("tools/rustup/bin"),
        vapor_home.join("tools/zig"),
        vapor_home.join("tools/llvm-mingw/bin"),
    ]);

    println!("== {config_name} ==");
    println!("workspace: {}", super_root.display());
    println!("Vapor home:  {}", vapor_home.display());
    println!("app bin:   {}", app_bin_dir.display());
    println!();

    env::set_current_dir(&workdir)
        .map_err(|error| format!("failed to enter {}: {error}", workdir.display()))?;

    match workflow.as_str() {
        "app-status" => status_main(),
        "setup-development" => {
            println!(
                "This installs/reconciles app-local Rust/Cargo, Zig, llvm-mingw, and Vapor-home tools."
            );
            let answer = prompt("Proceed with setup_development.rs? [y/N] ")?;
            if !is_yes(&answer) {
                println!("cancelled");
                return Ok(());
            }
            setup_development(&vapor_home)
        }
        "source-status" => source_status_main(),
        workflow if workflow.starts_with("source-clone:") => {
            let source_name = workflow
                .strip_prefix("source-clone:")
                .ok_or_else(|| format!("invalid source clone workflow: {workflow}"))?;
            let source = source_preset(source_name)?;
            println!("Clone one source into the SuperWorkspace.");
            println!("  source: {}", source.name);
            println!("  remote: {}", source.remote);
            println!(
                "  target: {}",
                super_root.join(SOURCES_DIR).join(source.name).display()
            );
            println!();
            let answer = prompt("Clone this source now? [y/N] ")?;
            if !is_yes(&answer) {
                println!("cancelled");
                return Ok(());
            }
            clone_source_into_superworkspace(Some(&super_root), Some(&vapor_home), source.name)
        }
        "source-init-basic-content" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Create a new source workspace from the basic content template.");
            println!("This writes only to the target path you provide.");
            let target_path = prompt("Target path: ")?;
            if target_path.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let organization = prompt("Organization slug [ghf-studios]: ")?;
            let organization = if organization.trim().is_empty() {
                "ghf-studios".to_owned()
            } else {
                organization
            };
            let workspace_name = prompt("Workspace name: ")?;
            if workspace_name.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            run_status(Command::new(&vapor).args([
                "source",
                "init",
                "basic-content",
                target_path.trim(),
                "--organization",
                organization.trim(),
                "--name",
                workspace_name.trim(),
            ]))
        }
        "vapor-shell-interactive" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!(
                "Opening the app-local Vapor shell. Use source commands to choose the active source workspace."
            );
            run_status(&mut Command::new(&vapor))
        }
        "app-binaries-status" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            require_executable(&installer, "app-local Vapor installer")?;
            require_executable(&entrypoint, "app-local Vapor entrypoint")?;
            let gui = app_bin_dir.join(executable("vapor-gui"));
            require_executable(&gui, "app-local Vapor GUI")?;
            println!("App-local binaries:");
            println!("  vapor:           {}", vapor.display());
            println!("  vapor-gui:       {}", gui.display());
            println!("  vapor-installer: {}", installer.display());
            println!("  vapor-entrypoint: {}", entrypoint.display());
            println!();
            run_status(Command::new(&vapor).arg("installation"))?;
            run_status(Command::new(&vapor).arg("binaries"))
        }
        "content-status" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            run_status(Command::new(&vapor).args(["content", "status"]))
        }
        "content-build-host" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Build the active content workspace into Vapor-home build outputs.");
            run_status(Command::new(&vapor).args(["content", "build", "--host-only"]))
        }
        "content-deploy-artifact" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Build and publish content locally into the installed Vapor home.");
            run_status(Command::new(&vapor).args(["content", "list"]))?;
            let artifact = prompt("\nArtifact to publish locally: ")?;
            if artifact.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let select_answer = prompt("Select deployed packagepack if applicable? [y/N] ")?;
            let mut command = Command::new(&vapor);
            command.args(["content", "deploy", artifact.trim(), "--host-only"]);
            if is_yes(&select_answer) {
                command.arg("--select");
            }
            run_status(&mut command)
        }
        "root-build-host" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Build the app/root host target into Vapor-home build outputs.");
            run_status(Command::new(&vapor).args(["root", "build", "--host-only"]))
        }
        "root-deploy-host" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!(
                "Publish already-built app/root host-target outputs locally into the installed Vapor home."
            );
            run_status(Command::new(&vapor).args(["root", "deploy", "--host-only", "--skip-docs"]))
        }
        "root-publish" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Publish the Steam app/depot through the app-local Vapor shell.");
            let account = prompt("Steam build account: ")?;
            if account.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let branch = prompt("Steam beta branch [manifest default]: ")?;
            let description = prompt("Build description [Vapor development build]: ")?;
            let description = if description.trim().is_empty() {
                "Vapor development build".to_owned()
            } else {
                description
            };
            let confirmation = prompt("\nType publish-app to upload the app/depot now: ")?;
            if confirmation.trim() != "publish-app" {
                println!("cancelled");
                return Ok(());
            }
            let mut command = Command::new(&vapor);
            command.args([
                "root",
                "publish",
                "--account",
                account.trim(),
                "--description",
                description.trim(),
            ]);
            if !branch.trim().is_empty() {
                command.args(["--branch", branch.trim()]);
            }
            run_status(command.arg("--yes"))
        }
        "content-publish" => {
            require_executable(&vapor, "app-local Vapor shell")?;
            println!("Publish one or more content artifacts through the app-local Vapor shell.");
            run_status(Command::new(&vapor).args(["content", "list"]))?;
            let artifact_line = prompt("\nArtifact(s) to publish, separated by spaces: ")?;
            let artifacts = artifact_line
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if artifacts.is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let account = prompt("Steam account: ")?;
            if account.trim().is_empty() {
                println!("cancelled");
                return Ok(());
            }
            let change_note = prompt("Change note [Vapor content update]: ")?;
            let change_note = if change_note.trim().is_empty() {
                "Vapor content update".to_owned()
            } else {
                change_note
            };
            let confirmation = prompt("\nType publish-content to upload Workshop content now: ")?;
            if confirmation.trim() != "publish-content" {
                println!("cancelled");
                return Ok(());
            }
            let mut command = Command::new(&vapor);
            command.arg("content").arg("publish");
            for artifact in artifacts {
                command.arg(artifact);
            }
            command.args([
                "--account",
                account.trim(),
                "--change-note",
                change_note.trim(),
                "--yes",
            ]);
            run_status(&mut command)
        }
        other => Err(format!("unknown Vapor-home IDE workflow: {other}")),
    }
}

fn ide_runner_executable() -> Result<PathBuf, String> {
    if let Some(runner) = env::var_os("VAPOR_IDE_RUNNER_EXE") {
        let runner = PathBuf::from(runner);
        if is_executable(&runner) {
            return Ok(runner);
        }
    }
    let runner = env::current_exe()
        .map_err(|error| format!("failed to resolve current executable: {error}"))?;
    if is_executable(&runner) {
        Ok(runner)
    } else {
        Err(format!(
            "IDE workflow runner is not executable: {}",
            runner.display()
        ))
    }
}

pub fn patch_rustrover_main() -> Result<(), String> {
    let super_root = super_workspace_root()?;
    let vapor_home = resolve_vapor_home(arg_path("--vapor-home"))?;
    let rust = rust_status(&vapor_home);
    if !rust.ready {
        return Err(format!(
            "app-local Rust/Cargo is not ready at {}\nmissing:\n  - {}\nrun: {} dev-env install --vapor-home {}",
            rust.path.display(),
            rust.missing.join("\n  - "),
            shell_arg(&app_control_runner(&vapor_home)),
            shell_arg(&vapor_home)
        ));
    }
    let rust_bin = rust.path;
    let cargo = rust_bin.join(executable("cargo"));
    let rustc = rust_bin.join(executable("rustc"));
    let rustup = vapor_home.join("tools/rustup/bin").join(executable("rustup"));
    let stdlib = rust_stdlib_source(&rustc)?;
    let cargo_projects = discover_cargo_project_registrations(&super_root)?;
    let run_configurations = rustrover_run_configurations();
    let ide_runner = app_control_runner(&vapor_home);
    if !is_executable(&ide_runner) {
        return Err(format!(
            "Vapor-home IDE runner is missing or not executable: {}\nrefresh the installed Vapor-home binaries from Vapor Client before patching RustRover",
            ide_runner.display()
        ));
    }
    let idea = super_root.join(".idea");
    fs::create_dir_all(&idea).map_err(io("create IDE directory", &idea))?;
    let toolchain_shim = install_rustrover_toolchain_shim(&idea, &vapor_home, &rust_bin, &rustup)?;

    write(
        &idea.join("cargoProjects.xml"),
        &cargo_projects_xml(&cargo_projects),
    )?;
    write(
        &idea.join("Loo-Cast-Repos.iml"),
        &module_xml(&super_root, &cargo_projects),
    )?;
    write(
        &idea.join("rust.xml"),
        &rust_xml(&toolchain_shim.bin, &stdlib),
    )?;
    write(
        &idea.join("vapor.xml"),
        &vapor_xml(
            &super_root,
            &vapor_home,
            &cargo,
            &rustc,
            &rustup,
            &stdlib,
            &toolchain_shim,
        ),
    )?;
    patch_workspace_xml(
        &idea.join("workspace.xml"),
        &cargo_projects_component(&cargo_projects),
        &rust_workspace_component(&toolchain_shim.bin, &stdlib),
        &run_manager_component(&run_configurations),
    )?;
    write_run_configurations(&idea, &super_root, &vapor_home, &run_configurations)?;
    let verification = verify_rustrover_setup(
        &super_root,
        &idea,
        &vapor_home,
        &toolchain_shim,
        &cargo_projects,
    )?;

    println!("RustRover Setup");
    println!();
    println!("Super workspace: {}", super_root.display());
    println!("Vapor home:        {}", vapor_home.display());
    println!("Rust/Cargo bin:  {}", rust_bin.display());
    println!("RustRover shim:  {}", toolchain_shim.bin.display());
    println!("Stdlib source:   {}", stdlib.display());
    println!("Cargo projects:  {}", cargo_projects.len());
    println!("Run configs:     {}", run_configurations.len());
    println!("Wrote:");
    println!("  {}", idea.join("cargoProjects.xml").display());
    println!("  {}", idea.join("Loo-Cast-Repos.iml").display());
    println!("  {}", idea.join("rust.xml").display());
    println!("  {}", idea.join("vapor.xml").display());
    println!("  {}", toolchain_shim.bin.display());
    println!("  {}", idea.join("workspace.xml").display());
    println!("  {}", idea.join("runConfigurations").display());
    println!();
    println!("Verified:");
    for check in verification {
        println!("  - {check}");
    }
    println!();
    println!(
        "If RustRover still shows stale editor-only dependency errors, close RustRover, rerun this patcher from a terminal, then reopen and reload Cargo projects."
    );
    Ok(())
}

pub fn source_status_main() -> Result<(), String> {
    source_status_at(None, None)
}

pub fn source_status_at(
    explicit_super_root: Option<&Path>,
    explicit_app_root: Option<&Path>,
) -> Result<(), String> {
    let super_root = resolve_super_workspace_root(explicit_super_root)?;
    let vapor_home = explicit_app_root
        .map(Path::to_path_buf)
        .or_else(candidate_vapor_home_from_exe)
        .and_then(|path| resolve_vapor_home(Some(path)).ok());
    let control = vapor_home
        .as_deref()
        .map(app_control_runner)
        .map(|path| shell_arg(&path))
        .unwrap_or_else(|| "vapor-installer".to_owned());
    println!("Vapor Source Status");
    println!();
    println!("Super workspace: {}", super_root.display());
    println!(
        "Source clones:   {}",
        super_root.join(SOURCES_DIR).display()
    );
    println!();
    for source in SOURCE_PRESETS {
        let root = super_root.join(SOURCES_DIR).join(source.name);
        println!("{}", source.name);
        println!("  role: {}", source.role);
        println!("  remote: {}", source.remote);
        println!("  path: {}", root.display());
        if !root.is_dir() {
            println!("  state: missing");
            let mut command = format!(
                "{control} source clone {} --super-workspace {}",
                source.name,
                shell_arg(&super_root)
            );
            if let Some(vapor_home) = &vapor_home {
                command.push_str(&format!(" --vapor-home {}", shell_arg(vapor_home)));
            }
            println!("  clone: {command}");
            continue;
        }
        match Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["status", "--short", "--branch"])
            .output()
        {
            Ok(output) if output.status.success() => {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    println!("  {line}");
                }
            }
            Ok(output) => {
                println!("  git status failed: {}", output.status);
            }
            Err(error) => {
                println!("  git unavailable: {error}");
            }
        }
        println!();
    }
    Ok(())
}
