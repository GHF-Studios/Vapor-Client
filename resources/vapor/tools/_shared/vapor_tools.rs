#![allow(dead_code, unused_variables, clippy::collapsible_if)]

use std::{
    env, fs,
    io::{Write, stdin, stdout},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub const APP_ROOT_ENV: &str = "LOO_CAST_APP_ROOT";
pub const SUPER_MANIFEST: &str = "SuperWorkspace.vapor.toml";
pub const APP_MANIFEST: &str = "App.vapor.toml";
pub const APP_SOURCE_MANIFEST: &str = "App-Source.vapor.toml";
pub const STEAM_APP_ID: &str = "2122620";
pub const STEAM_APP_DIR_NAME: &str = "Loo Cast";
pub const RUST_TOOLCHAIN: &str = "1.97.0";
pub const ZIG_VERSION: &str = "0.16.0";
pub const LLVM_MINGW_VERSION: &str = "20260616";
pub const SOURCES_DIR: &str = "sources";

const STEAMCMD_LINUX: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";
const STEAMCMD_WINDOWS: &str = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
const RUSTUP_INIT_X86_64_LINUX: &str =
    "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init";
const RUSTUP_INIT_AARCH64_LINUX: &str =
    "https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnu/rustup-init";
const RUSTUP_INIT_X86_64_WINDOWS: &str =
    "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe";
const ZIG_X86_64_LINUX: &str = "https://ziglang.org/download/0.16.0/zig-x86_64-linux-0.16.0.tar.xz";
const ZIG_X86_64_WINDOWS: &str =
    "https://ziglang.org/download/0.16.0/zig-x86_64-windows-0.16.0.zip";
const LLVM_MINGW_X86_64_LINUX: &str = "https://github.com/mstorsjo/llvm-mingw/releases/download/20260616/llvm-mingw-20260616-msvcrt-ubuntu-22.04-x86_64.tar.xz";
const LLVM_MINGW_X86_64_LINUX_SHA256: &str =
    "a1f7968b48ba8d949194d6dee6c76f3cd0f61cba91658599af2c2c834a55ab87";
const LLVM_MINGW_X86_64_WINDOWS: &str = "https://github.com/mstorsjo/llvm-mingw/releases/download/20260616/llvm-mingw-20260616-msvcrt-x86_64.zip";
const LLVM_MINGW_X86_64_WINDOWS_SHA256: &str =
    "744809646fdefe24a357399788d68fb07ecc65fa0be71baa2406793ce25c9813";

const RUST_TARGETS: &[&str] = &["x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnullvm"];
const RUST_COMPONENTS: &[&str] = &["rustfmt", "clippy", "rust-src"];
const PLAYER_DIRS: &[&str] = &[
    ".vapor/state",
    ".vapor/state/installer",
    ".vapor/logs",
    ".vapor/diagnostics/runs",
    ".vapor/downloads",
    ".vapor/extract",
    "content/cache/packages",
    "content/installed",
    "content/workshop/downloads",
    "tools",
];

const PLAYER_TEARDOWN_PATHS: &[&str] = &[
    "tools/git",
    "tools/steamcmd",
    ".vapor/registry",
    ".vapor/downloads",
    ".vapor/extract",
    ".vapor/state",
    ".vapor/diagnostics",
    ".vapor/logs",
    "content/cache",
    "content/installed",
    "content/workshop/downloads",
    "output",
];

const PLAYER_EMPTY_PARENT_DIRS: &[&str] = &["content/workshop", "content", "tools", ".vapor"];

#[derive(Debug, Clone)]
struct ComponentStatus {
    label: &'static str,
    ready: bool,
    path: PathBuf,
    missing: Vec<String>,
}

#[derive(Debug, Clone)]
struct CargoProjectRegistration {
    manifest: PathBuf,
}

#[derive(Debug, Clone)]
struct RunConfiguration {
    name: &'static str,
    folder: &'static str,
    file_stem: &'static str,
    kind: RunConfigurationKind,
    command: &'static str,
    working_directory: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct SourcePreset {
    name: &'static str,
    role: &'static str,
    remote: &'static str,
    branch: &'static str,
}

const SOURCE_PRESETS: &[SourcePreset] = &[
    SourcePreset {
        name: "Vapor-Client",
        role: "app-source-root",
        remote: "https://github.com/GHF-Studios/Vapor-Client.git",
        branch: "main",
    },
    SourcePreset {
        name: "Loo-Cast",
        role: "first-party-content-workspace",
        remote: "https://github.com/GHF-Studios/Loo-Cast.git",
        branch: "main",
    },
    SourcePreset {
        name: "Vapor-Registry",
        role: "registry-authority",
        remote: "https://github.com/GHF-Studios/Vapor-Registry.git",
        branch: "main",
    },
    SourcePreset {
        name: "Vapor-Platform-Server",
        role: "server-orchestration",
        remote: "https://github.com/GHF-Studios/Vapor-Platform-Server.git",
        branch: "main",
    },
];

#[derive(Debug, Clone, Copy)]
enum RunConfigurationKind {
    Cargo,
    Shell,
    Konsole,
}

const GENERATED_RUN_CONFIGURATION_STEMS: &[&str] = &[
    "00_Setup_Check_App_Environment",
    "00_Setup_Install_Developer_Tools",
    "10_Sources_Check_Source_Checkout",
    "10_Sources_Clone_Vapor_Root",
    "10_Sources_Clone_Vapor_Client",
    "10_Sources_Clone_Loo_Cast",
    "10_Sources_Clone_Vapor_Registry",
    "10_Sources_Clone_Vapor_Server_Root",
    "10_Sources_Clone_Vapor_Platform_Server",
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
    "00_Initial_Setup_Patch_RustRover",
    "00_Initial_Setup_Check_App_Environment",
    "00_Initial_Setup_Install_Developer_Tools",
    "10_Source_Workspaces_Check_Source_Checkout",
    "10_Source_Workspaces_Clone_Vapor_Root",
    "10_Source_Workspaces_Clone_Loo_Cast",
    "10_Source_Workspaces_Clone_Vapor_Registry",
    "10_Source_Workspaces_Clone_Vapor_Server_Root",
    "10_Source_Workspaces_Create_Basic_Content_Workspace",
    "20_Development_Workflows_Open_Vapor_Shell",
    "20_Development_Workflows_Check_App_Binaries",
    "20_Development_Workflows_Check_Launcher",
    "20_Development_Workflows_Check_Content",
    "20_Development_Workflows_Build_App",
    "20_Development_Workflows_Deploy_App",
    "20_Development_Workflows_Build_Content",
    "20_Development_Workflows_Deploy_Content",
    "30_Publishing_Publish_App",
    "30_Publishing_Publish_Content",
    "20_Development_Workflows_Deploy_Content_Locally",
    "20_Development_Workflows_Build_App_Locally",
    "30_Publish_Previews_Preview_Root_Publish",
    "30_Publish_Previews_Preview_Content_Publish",
    "00_IDE_Patch_RustRover",
    "01_Environment_Status",
    "02_Environment_Setup_Developer_Tools_Konsole",
    "10_Source_Status",
    "11_Source_Init_Basic_Content_Konsole",
    "20_Shell_Interactive_Konsole",
    "30_Launcher_Status",
    "40_Content_Status",
    "41_Content_Deploy_Artifact_Konsole",
    "50_Root_Build_Host_Konsole",
    "60_Publish_Root_Preview_Konsole",
    "61_Publish_Content_Preview_Konsole",
    "Cargo_Check_Loo_Cast",
    "Cargo_Check_Vapor_Launcher",
    "Cargo_Check_Vapor_SDK",
    "Cargo_Check_Vapor_Shell",
    "Server_Diagnostics_Local_Konsole",
    "Server_Identity_Local_Konsole",
    "Vapor_Content_Publish_Dry_Run_Konsole",
    "Vapor_Identity_Operator_Shell_Konsole",
    "Vapor_Launcher_Help",
    "Vapor_Launcher_Status",
    "Vapor_Root_Publish_Dry_Run_Konsole",
    "Vapor_Setup_Development_Setup_Konsole",
    "Vapor_Setup_Player_Setup_Konsole",
    "Vapor_Shell_Check_All",
    "Vapor_Shell_Content_Status",
    "Vapor_Shell_Interactive_Konsole",
    "Vapor_Shell_Metadata",
    "Vapor_Shell_Source_Status",
    "Vapor_Tools_App_Status",
    "Vapor_Tools_Patch_RustRover",
    "Vapor_Tools_Source_Status",
];

pub fn exit(result: Result<(), String>) {
    if let Err(error) = result {
        eprintln!("ERROR: {error}");
        std::process::exit(1);
    }
}

include!("vapor_tools/entrypoints.rs");
include!("vapor_tools/source_workspace.rs");
include!("vapor_tools/app_setup.rs");
include!("vapor_tools/discovery_paths.rs");
include!("vapor_tools/process_io.rs");
include!("vapor_tools/rust_cargo.rs");
include!("vapor_tools/rustrover.rs");
include!("vapor_tools/filesystem.rs");
