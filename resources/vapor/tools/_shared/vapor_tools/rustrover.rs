fn cargo_projects_xml(projects: &[CargoProjectRegistration]) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<project version=\"4\">\n{}</project>\n",
        cargo_projects_component(projects)
    )
}

fn cargo_projects_component(projects: &[CargoProjectRegistration]) -> String {
    let mut cargo_projects = String::new();
    cargo_projects.push_str("  <component name=\"CargoProjects\">\n");
    for project in projects {
        cargo_projects.push_str("    <cargoProject FILE=\"$PROJECT_DIR$/");
        cargo_projects.push_str(&xml_escape(&project.manifest.to_string_lossy()));
        cargo_projects.push_str("\" />\n");
    }
    cargo_projects.push_str("  </component>\n");
    cargo_projects
}

fn rust_xml(rust_bin: &Path, stdlib: &Path) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<project version=\"4\">\n  <component name=\"RsProjectSettings\">\n    <option name=\"toolchainHomeDirectory\" value=\"{}\" />\n    <option name=\"explicitPathToStdlib\" value=\"{}\" />\n  </component>\n  <component name=\"RustProjectSettingsService\">\n    <option name=\"explicitSetExplicitly\" value=\"true\" />\n    <option name=\"toolchainHomeDirectory\" value=\"{}\" />\n    <option name=\"stdlibSourceDirectory\" value=\"{}\" />\n  </component>\n</project>\n",
        xml_escape(&rust_bin.to_string_lossy()),
        xml_escape(&stdlib.to_string_lossy()),
        xml_escape(&rust_bin.to_string_lossy()),
        xml_escape(&stdlib.to_string_lossy()),
    )
}

fn rust_workspace_component(rust_bin: &Path, stdlib: &Path) -> String {
    format!(
        "  <component name=\"RustProjectSettings\">\n    <option name=\"explicitPathToStdlib\" value=\"{}\" />\n    <option name=\"toolchainHomeDirectory\" value=\"{}\" />\n  </component>\n",
        xml_escape(&stdlib.to_string_lossy()),
        xml_escape(&rust_bin.to_string_lossy()),
    )
}

#[derive(Debug, Clone)]
struct RustRoverToolchainShim {
    bin: PathBuf,
    cargo: PathBuf,
    rustc: PathBuf,
    rustup: PathBuf,
}

fn install_rustrover_toolchain_shim(
    idea: &Path,
    app_root: &Path,
    rust_bin: &Path,
    rustup: &Path,
) -> Result<RustRoverToolchainShim, String> {
    #[cfg(not(unix))]
    {
        Ok(RustRoverToolchainShim {
            bin: rust_bin.to_path_buf(),
            cargo: rust_bin.join(executable("cargo")),
            rustc: rust_bin.join(executable("rustc")),
            rustup: rustup.to_path_buf(),
        })
    }

    #[cfg(unix)]
    {
        let shim_bin = idea.join("vapor-toolchain/bin");
        reset_plain_dir(&shim_bin)?;

        let wrappers = [
            ("cargo", rust_bin.join(executable("cargo"))),
            ("rustc", rust_bin.join(executable("rustc"))),
            ("rustup", rustup.to_path_buf()),
            ("rustfmt", rust_bin.join(executable("rustfmt"))),
            ("cargo-clippy", rust_bin.join(executable("cargo-clippy"))),
            ("rustdoc", rust_bin.join(executable("rustdoc"))),
        ];

        for (name, target) in wrappers {
            let path = shim_bin.join(name);
            write_toolchain_shim(&path, app_root, rust_bin, rustup, name, &target)?;
            make_executable(&path)?;
        }

        Ok(RustRoverToolchainShim {
            bin: shim_bin.clone(),
            cargo: shim_bin.join("cargo"),
            rustc: shim_bin.join("rustc"),
            rustup: shim_bin.join("rustup"),
        })
    }
}

fn write_toolchain_shim(
    path: &Path,
    app_root: &Path,
    rust_bin: &Path,
    rustup: &Path,
    tool_name: &str,
    target: &Path,
) -> Result<(), String> {
    let source = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\n\nexport {}={}\nexport RUSTUP_HOME={}\nexport CARGO_HOME={}\nexport PATH={}:{}:{}:\"$PATH\"\n\nTOOL_NAME={}\nREAL_TOOL={}\nREAL_RUSTUP={}\n\ncase \"${{1:-}}\" in\n    +*)\n        REQUESTED_TOOLCHAIN=\"${{1#+}}\"\n        shift\n        exec \"$REAL_RUSTUP\" run \"$REQUESTED_TOOLCHAIN\" \"$TOOL_NAME\" \"$@\"\n        ;;\nesac\n\nexec \"$REAL_TOOL\" \"$@\"\n",
        APP_ROOT_ENV,
        shell_string(app_root),
        shell_string(&app_root.join("rustup-home")),
        shell_string(&app_root.join("cargo-home")),
        shell_string(rust_bin),
        shell_string(&app_root.join("rustup/bin")),
        shell_string(&app_root.join("cargo-home/bin")),
        shell_string_text(tool_name),
        shell_string(target),
        shell_string(rustup),
    );
    write(path, &source)
}

fn verify_rustrover_setup(
    super_root: &Path,
    idea: &Path,
    app_root: &Path,
    toolchain_shim: &RustRoverToolchainShim,
    cargo_projects: &[CargoProjectRegistration],
) -> Result<Vec<String>, String> {
    let mut checks = Vec::new();

    for executable_path in [
        &toolchain_shim.cargo,
        &toolchain_shim.rustc,
        &toolchain_shim.rustup,
    ] {
        if !is_executable(executable_path) {
            return Err(format!(
                "RustRover toolchain shim is missing executable: {}",
                executable_path.display()
            ));
        }
    }
    checks.push("toolchain shim executables exist".to_owned());

    let cargo_wrapper = read_required(&toolchain_shim.cargo)?;
    require_contains(
        &cargo_wrapper,
        &format!(
            "export CARGO_HOME={}",
            shell_string(&app_root.join("cargo-home"))
        ),
        "cargo shim must export app-local CARGO_HOME",
    )?;
    require_contains(
        &cargo_wrapper,
        &format!(
            "export RUSTUP_HOME={}",
            shell_string(&app_root.join("rustup-home"))
        ),
        "cargo shim must export app-local RUSTUP_HOME",
    )?;
    checks.push("toolchain shim exports app-local Cargo/Rustup homes".to_owned());

    let rust_xml_path = idea.join("rust.xml");
    let rust_xml_source = read_required(&rust_xml_path)?;
    require_contains(
        &rust_xml_source,
        &toolchain_shim.bin.to_string_lossy(),
        "rust.xml must point RustRover at the generated toolchain shim",
    )?;
    checks.push("RustRover settings point at the shim toolchain".to_owned());

    verify_app_root_centered_run_configurations(idea, app_root)?;
    checks.push("generated operational workflows use precompiled app-root binaries".to_owned());

    let iml_path = idea.join("Loo-Cast-Repos.iml");
    let iml_source = read_required(&iml_path)?;
    if iml_source.contains("<sourceFolder url=") {
        return Err(format!(
            "{} still contains explicit source roots; this can detach files from Cargo dependency context",
            iml_path.display()
        ));
    }
    checks.push("module file has no hand-written Rust source roots".to_owned());

    for project in cargo_projects {
        let manifest = super_root.join(&project.manifest);
        if !manifest.is_file() {
            return Err(format!(
                "registered Cargo manifest does not exist: {}",
                manifest.display()
            ));
        }
    }
    checks.push(format!(
        "{} registered Cargo manifests exist",
        cargo_projects.len()
    ));

    let rustc_version = command_stdout(
        Command::new(&toolchain_shim.rustc)
            .arg("-Vv")
            .current_dir(super_root),
    )?;
    require_contains(
        &rustc_version,
        &format!("release: {RUST_TOOLCHAIN}"),
        "RustRover shim rustc must be the pinned Vapor toolchain",
    )?;
    checks.push(format!("shim rustc reports Rust {RUST_TOOLCHAIN}"));

    let host_toolchain = format!("{RUST_TOOLCHAIN}-{}", host_rust_target()?);
    let cargo_plus_version = command_stdout(
        Command::new(&toolchain_shim.cargo)
            .arg(format!("+{host_toolchain}"))
            .arg("-Vv")
            .current_dir(super_root),
    )?;
    require_contains(
        &cargo_plus_version,
        &format!("release: {RUST_TOOLCHAIN}"),
        "RustRover shim cargo must accept rustup-style +toolchain directives",
    )?;
    checks.push("shim cargo accepts rustup-style +toolchain directives".to_owned());

    let launcher_manifest = "sources/Vapor-Client/Vapor-Launcher/Cargo.toml";
    if super_root.join(launcher_manifest).is_file() {
        let launcher_metadata =
            cargo_metadata_no_deps(super_root, &toolchain_shim.cargo, launcher_manifest)?;
        require_metadata_names(
            "Vapor-Launcher",
            &launcher_metadata,
            &[
                "vapor_launcher_cli",
                "vapor_launcher_core",
                "vapor_core",
                "clap",
                "dialoguer",
                "owo-colors",
            ],
        )?;
        checks.push("shim cargo metadata sees launcher registry and path dependencies".to_owned());
    } else {
        checks.push("launcher source not mounted; skipped launcher metadata check".to_owned());
    }

    let shell_manifest = "sources/Vapor-Client/Vapor-Shell/Cargo.toml";
    if super_root.join(shell_manifest).is_file() {
        let shell_metadata =
            cargo_metadata_no_deps(super_root, &toolchain_shim.cargo, shell_manifest)?;
        require_metadata_names(
            "Vapor-Shell",
            &shell_metadata,
            &["vapor_shell", "clap", "clap-repl"],
        )?;
        checks.push("shim cargo metadata sees shell workspace dependencies".to_owned());
    } else {
        checks.push("shell source not mounted; skipped shell metadata check".to_owned());
    }

    Ok(checks)
}

fn cargo_metadata_no_deps(
    super_root: &Path,
    cargo: &Path,
    manifest: &str,
) -> Result<String, String> {
    command_stdout(
        Command::new(cargo)
            .args(["metadata", "--format-version", "1", "--manifest-path"])
            .arg(super_root.join(manifest))
            .arg("--no-deps")
            .current_dir(super_root),
    )
}

fn verify_app_root_centered_run_configurations(idea: &Path, app_root: &Path) -> Result<(), String> {
    let generated_scripts = idea.join("vapor-run");
    if generated_scripts.exists() {
        return Err(format!(
            "generated IDE workflow script directory still exists: {}",
            generated_scripts.display()
        ));
    }
    let generated_configurations = rustrover_run_configurations();
    let app_control_runner = app_control_runner(app_root);
    for configuration in &generated_configurations {
        if matches!(configuration.kind, RunConfigurationKind::Cargo) {
            continue;
        }
        let xml_path = idea
            .join("runConfigurations")
            .join(format!("{}.xml", configuration.file_stem));
        let xml = read_required(&xml_path)?;
        require_contains(
            &xml,
            " folderName=\"",
            "generated workflow run configuration must use JetBrains folderName attribute",
        )?;
        if xml.contains("<folderName") {
            return Err(format!(
                "{} still uses ignored folderName child elements",
                xml_path.display()
            ));
        }
        require_contains(
            &xml,
            &xml_escape(&app_control_runner.to_string_lossy()),
            "generated workflow must launch the precompiled app-root control runner",
        )?;
        require_contains(
            &xml,
            "ide run --workflow",
            "generated workflow must enter the compiled IDE workflow runner",
        )?;
        if xml.contains("rust-script")
            || xml.contains("development/ide_run/run.rs")
            || xml.contains("$PROJECT_DIR$/tools")
            || xml.contains(".idea/vapor-run")
            || xml.contains("cargo run --package vapor_shell")
            || xml.contains("cargo run --package vapor_launcher_cli")
        {
            return Err(format!(
                "{} still depends on source-local tools, rust-script, or generated workflow scripts",
                xml_path.display()
            ));
        }
    }

    let app_root_workflow_stems = [
        "10_Sources_Create_Basic_Content_Workspace",
        "20_Run_Open_Vapor_Shell",
        "20_Run_Check_App_Binaries",
        "20_Run_Check_Content",
        "30_Build_App",
        "30_Build_Content",
        "40_Stage_App",
        "40_Stage_Content",
        "50_Publish_App",
        "50_Publish_Content",
    ];

    for stem in app_root_workflow_stems {
        let xml_path = idea.join("runConfigurations").join(format!("{stem}.xml"));
        let xml = read_required(&xml_path)?;
        require_contains(
            &xml,
            &format!(
                "<option name=\"SCRIPT_WORKING_DIRECTORY\" value=\"{}\" />",
                xml_escape(&app_root.to_string_lossy())
            ),
            "app-root workflow run configuration must start in the app root",
        )?;
    }

    Ok(())
}

fn require_metadata_names(
    label: &str,
    metadata: &str,
    required_names: &[&str],
) -> Result<(), String> {
    let missing = required_names
        .iter()
        .filter(|name| {
            let needle = format!("\"name\":\"{name}\"");
            !metadata.contains(&needle)
        })
        .copied()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{label} Cargo metadata through RustRover shim is missing dependency/package names: {}",
            missing.join(", ")
        ))
    }
}

fn read_required(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(io("read generated RustRover file", path))
}

fn require_contains(source: &str, needle: &str, message: &str) -> Result<(), String> {
    if source.contains(needle) {
        Ok(())
    } else {
        Err(format!("{message}: missing {needle:?}"))
    }
}

fn run_manager_component(configurations: &[RunConfiguration]) -> String {
    let selected = configurations
        .first()
        .map(run_configuration_item_value)
        .unwrap_or_else(|| "Shell Script.Patch RustRover".to_owned());
    format!(
        "  <component name=\"RunManager\" selected=\"{}\" />\n",
        xml_escape(&selected)
    )
}

fn run_configuration_item_value(configuration: &RunConfiguration) -> String {
    let prefix = match configuration.kind {
        RunConfigurationKind::Cargo => "Cargo",
        RunConfigurationKind::Shell | RunConfigurationKind::Konsole => "Shell Script",
    };
    format!("{prefix}.{}", configuration.name)
}

fn rustrover_run_configurations() -> Vec<RunConfiguration> {
    use RunConfigurationKind::{Konsole, Shell};
    vec![
        RunConfiguration {
            name: "Check app environment",
            folder: "00 Setup",
            file_stem: "00_Setup_Check_App_Environment",
            kind: Shell,
            working_directory: ".",
            command: "app-status",
        },
        RunConfiguration {
            name: "Install developer tools",
            folder: "00 Setup",
            file_stem: "00_Setup_Install_Developer_Tools",
            kind: Konsole,
            working_directory: ".",
            command: "setup-development",
        },
        RunConfiguration {
            name: "Check source checkout",
            folder: "10 Sources",
            file_stem: "10_Sources_Check_Source_Checkout",
            kind: Shell,
            working_directory: ".",
            command: "source-status",
        },
        RunConfiguration {
            name: "Clone source: Vapor Client",
            folder: "10 Sources",
            file_stem: "10_Sources_Clone_Vapor_Client",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Vapor-Client",
        },
        RunConfiguration {
            name: "Clone source: Loo-Cast",
            folder: "10 Sources",
            file_stem: "10_Sources_Clone_Loo_Cast",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Loo-Cast",
        },
        RunConfiguration {
            name: "Clone source: Vapor Registry",
            folder: "10 Sources",
            file_stem: "10_Sources_Clone_Vapor_Registry",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Vapor-Registry",
        },
        RunConfiguration {
            name: "Clone source: Vapor Platform Server",
            folder: "10 Sources",
            file_stem: "10_Sources_Clone_Vapor_Platform_Server",
            kind: Konsole,
            working_directory: ".",
            command: "source-clone:Vapor-Platform-Server",
        },
        RunConfiguration {
            name: "Create basic content workspace",
            folder: "10 Sources",
            file_stem: "10_Sources_Create_Basic_Content_Workspace",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "source-init-basic-content",
        },
        RunConfiguration {
            name: "Open Vapor Shell",
            folder: "20 Run",
            file_stem: "20_Run_Open_Vapor_Shell",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "vapor-shell-interactive",
        },
        RunConfiguration {
            name: "Check app binaries",
            folder: "20 Run",
            file_stem: "20_Run_Check_App_Binaries",
            kind: Shell,
            working_directory: "$APP_ROOT",
            command: "app-binaries-status",
        },
        RunConfiguration {
            name: "Check content",
            folder: "20 Run",
            file_stem: "20_Run_Check_Content",
            kind: Shell,
            working_directory: "$APP_ROOT",
            command: "content-status",
        },
        RunConfiguration {
            name: "Build app",
            folder: "30 Build",
            file_stem: "30_Build_App",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "root-build-host",
        },
        RunConfiguration {
            name: "Stage app",
            folder: "40 Stage",
            file_stem: "40_Stage_App",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "root-deploy-host",
        },
        RunConfiguration {
            name: "Build content",
            folder: "30 Build",
            file_stem: "30_Build_Content",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "content-build-host",
        },
        RunConfiguration {
            name: "Stage content",
            folder: "40 Stage",
            file_stem: "40_Stage_Content",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "content-deploy-artifact",
        },
        RunConfiguration {
            name: "Publish app",
            folder: "50 Publish",
            file_stem: "50_Publish_App",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "root-publish",
        },
        RunConfiguration {
            name: "Publish content",
            folder: "50 Publish",
            file_stem: "50_Publish_Content",
            kind: Konsole,
            working_directory: "$APP_ROOT",
            command: "content-publish",
        },
    ]
}

fn module_xml(root: &Path, projects: &[CargoProjectRegistration]) -> String {
    let mut exclude_folders = Vec::new();

    for project in projects {
        if let Some(project_dir) = project.manifest.parent() {
            exclude_folders.push(project_dir.join("target"));
        }
    }

    exclude_folders.sort();
    exclude_folders.dedup();

    let mut body = String::new();
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    body.push_str("<module type=\"EMPTY_MODULE\" version=\"4\">\n");
    body.push_str("  <component name=\"NewModuleRootManager\">\n");
    body.push_str("    <content url=\"file://$MODULE_DIR$\">\n");
    for folder in exclude_folders {
        body.push_str("      <excludeFolder url=\"file://$MODULE_DIR$/");
        body.push_str(&xml_escape(&folder.to_string_lossy()));
        body.push_str("\" />\n");
    }
    body.push_str("    </content>\n");
    body.push_str("    <orderEntry type=\"inheritedJdk\" />\n");
    body.push_str("    <orderEntry type=\"sourceFolder\" forTests=\"false\" />\n");
    body.push_str("  </component>\n");
    body.push_str("</module>\n");
    body
}

fn patch_workspace_xml(
    path: &Path,
    cargo_projects: &str,
    rust_settings: &str,
    run_manager: &str,
) -> Result<(), String> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<project version=\"4\">\n</project>\n"
                .to_owned()
        }
        Err(error) => return Err(io("read RustRover workspace file", path)(error)),
    };
    let source = upsert_project_component(&source, "CargoProjects", cargo_projects)?;
    let source = upsert_project_component(&source, "RustProjectSettings", rust_settings)?;
    let source = upsert_project_component(&source, "RunManager", run_manager)?;
    write(path, &source)
}

fn upsert_project_component(source: &str, name: &str, replacement: &str) -> Result<String, String> {
    let marker = format!("<component name=\"{name}\"");
    if let Some(component_start) = source.find(&marker) {
        let start = source[..component_start]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(component_start);
        let tag_end = source[component_start..]
            .find('>')
            .map(|index| component_start + index + 1)
            .ok_or_else(|| format!("malformed workspace.xml component {name}: missing '>'"))?;
        let mut end = if source[component_start..tag_end].trim_end().ends_with("/>") {
            tag_end
        } else {
            source[tag_end..]
                .find("</component>")
                .map(|index| tag_end + index + "</component>".len())
                .ok_or_else(|| {
                    format!("malformed workspace.xml component {name}: missing </component>")
                })?
        };
        if source[end..].starts_with('\n') {
            end += 1;
        }
        let mut updated = String::new();
        updated.push_str(&source[..start]);
        updated.push_str(replacement);
        updated.push_str(&source[end..]);
        return Ok(updated);
    }

    let project_start = source
        .find("<project version=\"4\">")
        .ok_or_else(|| "workspace.xml does not contain <project version=\"4\">".to_owned())?;
    let insert = source[project_start..]
        .find('\n')
        .map(|index| project_start + index + 1)
        .ok_or_else(|| "workspace.xml project element is not line-oriented".to_owned())?;
    let mut updated = String::new();
    updated.push_str(&source[..insert]);
    updated.push_str(replacement);
    updated.push_str(&source[insert..]);
    Ok(updated)
}

fn write_run_configurations(
    idea: &Path,
    super_root: &Path,
    app_root: &Path,
    configurations: &[RunConfiguration],
) -> Result<(), String> {
    let run_config_dir = idea.join("runConfigurations");
    fs::create_dir_all(&run_config_dir)
        .map_err(io("create run configuration directory", &run_config_dir))?;
    let generated_script_dir = idea.join("vapor-run");
    if generated_script_dir.exists() {
        fs::remove_dir_all(&generated_script_dir).map_err(io(
            "remove generated IDE workflow scripts",
            &generated_script_dir,
        ))?;
    }
    remove_stale_generated_run_configurations(&run_config_dir, configurations)?;

    for configuration in configurations {
        let xml_path = run_config_dir.join(format!("{}.xml", configuration.file_stem));
        match configuration.kind {
            RunConfigurationKind::Cargo => {
                write(
                    &xml_path,
                    &cargo_run_configuration_xml(configuration, app_root),
                )?;
            }
            RunConfigurationKind::Shell | RunConfigurationKind::Konsole => {
                write(
                    &xml_path,
                    &shell_run_configuration_xml(configuration, super_root, app_root),
                )?;
            }
        }
    }

    Ok(())
}

fn remove_stale_generated_run_configurations(
    run_config_dir: &Path,
    configurations: &[RunConfiguration],
) -> Result<(), String> {
    for stem in GENERATED_RUN_CONFIGURATION_STEMS {
        if configurations
            .iter()
            .any(|configuration| configuration.file_stem == *stem)
        {
            continue;
        }
        let xml_path = run_config_dir.join(format!("{stem}.xml"));
        if xml_path.is_file() {
            fs::remove_file(&xml_path).map_err(io("remove stale run configuration", &xml_path))?;
        }
    }
    Ok(())
}

fn cargo_run_configuration_xml(configuration: &RunConfiguration, app_root: &Path) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<component name=\"ProjectRunConfigurationManager\">\n  <configuration default=\"false\" name=\"{}\" type=\"CargoCommandRunConfiguration\" factoryName=\"Cargo Command\" folderName=\"{}\">\n    <option name=\"command\" value=\"{}\" />\n    <option name=\"workingDirectory\" value=\"file://$PROJECT_DIR$/{}\" />\n{}\n    <option name=\"emulateTerminal\" value=\"true\" />\n    <option name=\"channel\" value=\"DEFAULT\" />\n    <option name=\"requiredFeatures\" value=\"true\" />\n    <option name=\"allFeatures\" value=\"false\" />\n    <option name=\"withSudo\" value=\"false\" />\n    <option name=\"buildTarget\" value=\"REMOTE\" />\n    <option name=\"backtrace\" value=\"SHORT\" />\n    <option name=\"isRedirectInput\" value=\"false\" />\n    <option name=\"redirectInputPath\" value=\"\" />\n    <method v=\"2\">\n      <option name=\"CARGO.BUILD_TASK_PROVIDER\" enabled=\"true\" />\n    </method>\n  </configuration>\n</component>\n",
        xml_escape(configuration.name),
        xml_escape(configuration.folder),
        xml_escape(configuration.command),
        xml_escape(configuration.working_directory),
        run_config_envs_xml(app_root),
    )
}

fn shell_run_configuration_xml(
    configuration: &RunConfiguration,
    super_root: &Path,
    app_root: &Path,
) -> String {
    let working_directory = run_configuration_xml_working_directory(configuration, app_root);
    let script_path = app_control_runner(app_root);
    let options = format!(
        "ide run --workflow {} --super-workspace {} --app-root {} --workdir {} --config-name {} --open-konsole {}",
        shell_arg_text(configuration.command),
        shell_arg(super_root),
        shell_arg(app_root),
        shell_arg(Path::new(&working_directory)),
        shell_arg_text(configuration.name),
        if matches!(configuration.kind, RunConfigurationKind::Konsole) {
            "true"
        } else {
            "false"
        },
    );
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<component name=\"ProjectRunConfigurationManager\">\n  <configuration default=\"false\" name=\"{}\" type=\"ShConfigurationType\" factoryName=\"Shell Script\" folderName=\"{}\">\n    <option name=\"SCRIPT_TEXT\" value=\"\" />\n    <option name=\"INDEPENDENT_SCRIPT_PATH\" value=\"true\" />\n    <option name=\"SCRIPT_PATH\" value=\"{}\" />\n    <option name=\"SCRIPT_OPTIONS\" value=\"{}\" />\n    <option name=\"INDEPENDENT_SCRIPT_WORKING_DIRECTORY\" value=\"true\" />\n    <option name=\"SCRIPT_WORKING_DIRECTORY\" value=\"{}\" />\n    <option name=\"INDEPENDENT_INTERPRETER_PATH\" value=\"true\" />\n    <option name=\"INTERPRETER_PATH\" value=\"/usr/bin/env\" />\n    <option name=\"INTERPRETER_OPTIONS\" value=\"\" />\n    <option name=\"EXECUTE_IN_TERMINAL\" value=\"true\" />\n    <option name=\"EXECUTE_SCRIPT_FILE\" value=\"true\" />\n    <envs />\n    <method v=\"2\" />\n  </configuration>\n</component>\n",
        xml_escape(configuration.name),
        xml_escape(configuration.folder),
        xml_escape(&script_path.to_string_lossy()),
        xml_escape(&options),
        xml_escape(&working_directory),
    )
}

fn run_configuration_xml_working_directory(
    configuration: &RunConfiguration,
    app_root: &Path,
) -> String {
    if configuration.working_directory == "$APP_ROOT" {
        app_root.to_string_lossy().into_owned()
    } else if configuration.working_directory == "." {
        "$PROJECT_DIR$".to_owned()
    } else {
        format!("$PROJECT_DIR$/{}", configuration.working_directory)
    }
}

fn run_config_envs_xml(app_root: &Path) -> String {
    format!(
        "    <envs>\n      <env name=\"{}\" value=\"{}\" />\n      <env name=\"RUSTUP_HOME\" value=\"{}\" />\n      <env name=\"CARGO_HOME\" value=\"{}\" />\n    </envs>",
        APP_ROOT_ENV,
        xml_escape(&app_root.to_string_lossy()),
        xml_escape(&app_root.join("rustup-home").to_string_lossy()),
        xml_escape(&app_root.join("cargo-home").to_string_lossy()),
    )
}

fn vapor_xml(
    super_root: &Path,
    app_root: &Path,
    cargo: &Path,
    rustc: &Path,
    rustup: &Path,
    stdlib: &Path,
    toolchain_shim: &RustRoverToolchainShim,
) -> String {
    format!(
        "<project version=\"4\">\n  <component name=\"VaporSuperWorkspaceSettings\">\n    <option name=\"schemaVersion\" value=\"1\" />\n    <option name=\"superWorkspaceRoot\" value=\"$PROJECT_DIR$\" />\n    <option name=\"superWorkspaceManifest\" value=\"{}\" />\n    <option name=\"appRoot\" value=\"{}\" />\n    <option name=\"cargoHome\" value=\"{}\" />\n    <option name=\"rustupHome\" value=\"{}\" />\n    <option name=\"cargoPath\" value=\"{}\" />\n    <option name=\"rustcPath\" value=\"{}\" />\n    <option name=\"rustupPath\" value=\"{}\" />\n    <option name=\"rustStdlibSource\" value=\"{}\" />\n    <option name=\"rustRoverToolchainBin\" value=\"{}\" />\n    <option name=\"rustRoverCargoPath\" value=\"{}\" />\n    <option name=\"rustRoverRustcPath\" value=\"{}\" />\n    <option name=\"rustRoverRustupPath\" value=\"{}\" />\n  </component>\n</project>\n",
        xml_escape(&super_root.join(SUPER_MANIFEST).to_string_lossy()),
        xml_escape(&app_root.to_string_lossy()),
        xml_escape(&app_root.join("cargo-home").to_string_lossy()),
        xml_escape(&app_root.join("rustup-home").to_string_lossy()),
        xml_escape(&cargo.to_string_lossy()),
        xml_escape(&rustc.to_string_lossy()),
        xml_escape(&rustup.to_string_lossy()),
        xml_escape(&stdlib.to_string_lossy()),
        xml_escape(&toolchain_shim.bin.to_string_lossy()),
        xml_escape(&toolchain_shim.cargo.to_string_lossy()),
        xml_escape(&toolchain_shim.rustc.to_string_lossy()),
        xml_escape(&toolchain_shim.rustup.to_string_lossy()),
    )
}
