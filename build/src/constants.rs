use crate::cmd::ExternCommand;

// CLI defaults
pub const CLI_DEFAULT_PATCH_VERSION:       &str = "1";
pub const CLI_DEFAULT_PATCH_SUMMARY:       &str = "Syscare Patch";
pub const CLI_DEFAULT_WORK_DIR:            &str = ".";
pub const CLI_DEFAULT_OUTPUT_DIR:          &str = ".";
pub const CLI_DEFAULT_SKIP_COMPILER_CHECK: &str = "false";

// External commands
pub const MAKE:         ExternCommand = ExternCommand::new("/usr/bin/make");
pub const RPM:          ExternCommand = ExternCommand::new("/usr/bin/rpm");
pub const RPM_BUILD:    ExternCommand = ExternCommand::new("/usr/bin/rpmbuild");
pub const KPATCH_BUILD: ExternCommand = ExternCommand::new("/usr/bin/kpatch-build");
pub const UPATCH_BUILD: ExternCommand = ExternCommand::new("/usr/bin/upatch-build");

// Patch
pub const PATCH_FILE_PREFIX:       &str  = "syscare-patch";
pub const PATCH_FILE_EXTENSION:    &str  = "patch";
pub const PATCH_DIR_PERMISSION:    &str  = "755";
pub const PATCH_FILE_PERMISSION:   &str  = "644";
pub const PATCH_FILE_INSTALL_PATH: &str  = "/usr/lib/syscare/patches";
pub const PATCH_UNDEFINED_VALUE:   &str  = "Undefined";
pub const PATCH_INFO_FILE_NAME:    &str  = "patch_info";
pub const PATCH_VERSION_DIGITS:    usize = 8;

// Package
pub const PKG_FILE_EXTENSION:               &str = "rpm";
pub const PKG_NAME_SPLITER:                 char = '-';
pub const PKG_FLAG_SOURCE_PKG:              &str = "(none)";
pub const PKG_FLAG_PATCHED_SOURCE_PKG:      &str = "patched";
pub const PKG_PATCH_VERSION_FILE_NAME:      &str = "syscare-patch-version";
pub const PKG_PATCH_TARGET_FILE_NAME:       &str = "syscare-patch-target";
pub const PKG_SPEC_FILE_EXTENSION:          &str = "spec";
pub const PKG_SPEC_TAG_SPLITER:             char = ':';
pub const PKG_SPEC_TAG_NAME_RELEASE:        &str = "Release:";
pub const PKG_SPEC_TAG_NAME_SOURCE:         &str = "Source";
pub const PKG_SPEC_TAG_NAME_BUILD_REQUIRES: &str = "BuildRequires:";
pub const PKG_SPEC_TAG_VALUE_GROUP:         &str = "Patch";

// Kernel
pub const KERNEL_PKG_NAME:          &str = "kernel";
pub const KERNEL_SOURCE_DIR_FLAG:   &str = "Kbuild";
pub const KERNEL_SOURCE_DIR_PREFIX: &str = "linux-";
pub const KERNEL_CONFIG_NAME:       &str = ".config";
pub const KERNEL_DEFCONFIG_NAME:    &str = "openeuler_defconfig";
pub const KERNEL_FILE_NAME:         &str = "vmlinux";
