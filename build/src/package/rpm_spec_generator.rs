use std::io::{Write, LineWriter};

use crate::constants::*;
use crate::util::fs;

use crate::patch::PatchInfo;

pub struct RpmSpecGenerator;

impl RpmSpecGenerator {
    fn parse_pkg_root(patch_info: &PatchInfo) -> String {
        format!("{}/{}",
            PATCH_INSTALL_PATH,
            patch_info.get_target().get_simple_name())
    }

    fn parse_patch_name(patch_info: &PatchInfo) -> String {
        patch_info.get_name().to_owned()
    }

    fn parse_patch_root(patch_info: &PatchInfo) -> String {
        format!("{}/{}/{}",
            PATCH_INSTALL_PATH,
            patch_info.get_target().get_simple_name(),
            patch_info.get_name())
    }

    fn parse_pkg_name(patch_info: &PatchInfo) -> String {
        format!("{}-{}-{}",
            PKG_FLAG_PATCH_BINARY,
            patch_info.get_target().get_simple_name(),
            patch_info.get_name())
    }

    fn parse_pkg_version(patch_info: &PatchInfo) -> String {
        patch_info.get_version().to_owned()
    }

    fn parse_pkg_release(patch_info: &PatchInfo) -> String {
        patch_info.get_release().to_owned()
    }

    fn parse_requires(patch_info: &PatchInfo) -> String {
        let patch_target = patch_info.get_target();

        match patch_target.get_epoch() == PKG_FLAG_NONE {
            true => {
                format!("{} = {}-{}",
                    patch_target.get_name(),
                    patch_target.get_version(),
                    patch_target.get_release()
                )
            },
            false => {
                format!("{} = {}:{}-{}",
                    patch_target.get_name(),
                    patch_target.get_epoch(),
                    patch_target.get_version(),
                    patch_target.get_release()
                )
            }
        }
    }

    fn parse_license(patch_info: &PatchInfo) -> String {
        patch_info.get_license().to_owned()
    }

    fn parse_summary(patch_info: &PatchInfo) -> String {
        format!("Syscare patch '{}' for {}",
            patch_info.get_name(),
            patch_info.get_target().get_simple_name()
        )
    }

    fn write_patch_info<W>(mut writer: W, patch_info: &PatchInfo, source_dir: &str) -> std::io::Result<()>
    where
        W: Write
    {
        let pkg_file_list = fs::list_all_files(source_dir, true)?
            .into_iter()
            .map(fs::file_name)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        writeln!(writer, "%global pkg_root              {}", Self::parse_pkg_root(patch_info))?;
        writeln!(writer, "%global patch_name            {}", Self::parse_patch_name(patch_info))?;
        writeln!(writer, "%global patch_root            {}", Self::parse_patch_root(patch_info))?;
        writeln!(writer, "%global patch_dir_permission  {}", PATCH_DIR_PERMISSION)?;
        writeln!(writer, "%global patch_file_permission {}", PATCH_FILE_PERMISSION)?;

        writeln!(writer, "Name:     {}", Self::parse_pkg_name(patch_info))?;
        writeln!(writer, "Version:  {}", Self::parse_pkg_version(patch_info))?;
        writeln!(writer, "Release:  {}", Self::parse_pkg_release(patch_info))?;
        writeln!(writer, "Group:    {}", PKG_SPEC_TAG_VALUE_GROUP)?;
        writeln!(writer, "License:  {}", Self::parse_license(patch_info))?;
        writeln!(writer, "Summary:  {}", Self::parse_summary(patch_info))?;
        writeln!(writer, "Requires: {}", Self::parse_requires(patch_info))?;
        writeln!(writer, "Requires: {}", PKG_SPEC_TAG_VALUE_REQUIRES)?;
        let mut file_index = 0usize;
        for file_name in &pkg_file_list {
            writeln!(writer, "Source{}: {}", file_index, file_name)?;
            file_index += 1;
        }
        writeln!(writer)?;

        writeln!(writer, "%description")?;
        writeln!(writer, "{}", patch_info.get_description())?;
        writeln!(writer)?;

        writeln!(writer, "%prep")?;
        writeln!(writer, "cp -a %{{_sourcedir}}/* %{{_builddir}}")?;
        writeln!(writer)?;

        writeln!(writer, "%build")?;
        writeln!(writer)?;

        writeln!(writer, "%install")?;
        writeln!(writer, "install -m %{{patch_dir_permission}} -d %{{buildroot}}%{{patch_root}}")?;
        for file_name in &pkg_file_list {
            writeln!(writer, "install -m %{{patch_file_permission}} %{{_builddir}}/{} %{{buildroot}}%{{patch_root}}", file_name)?;
        }
        writeln!(writer)?;

        writeln!(writer, "%files")?;
        writeln!(writer, "%{{patch_root}}")?;
        writeln!(writer)?;

        writeln!(writer, "%preun")?;
        writeln!(writer, "if [ \"$(syscare status %{{patch_name}})\" != \"NOT-APPLIED\" ]; then")?;
        writeln!(writer, "    echo \"error: cannot remove applied patch \'%{{patch_name}}\'\" >&2")?;
        writeln!(writer, "    exit 1")?;
        writeln!(writer, "fi")?;

        writeln!(writer, "%postun")?;
        writeln!(writer, "if [ \"$1\" != 0 ]; then")?;
        writeln!(writer, "    exit 0")?;
        writeln!(writer, "fi")?;
        writeln!(writer, "if [ -d \"%{{pkg_root}}\" ] && [ -z \"$(ls -A %{{pkg_root}})\" ]; then")?;
        writeln!(writer, "    rm -rf \"%{{pkg_root}}\"")?;
        writeln!(writer, "fi")?;
        writeln!(writer)?;

        writeln!(writer, "%changelog")?;
        writeln!(writer)?;

        writer.flush()
    }

    pub fn generate_from_patch_info(patch_info: &PatchInfo, source_dir: &str, output_dir: &str) -> std::io::Result<String> {
        fs::check_dir(source_dir)?;
        fs::check_dir(output_dir)?;

        let pkg_spec_path = format!("{}/{}.spec", output_dir, Self::parse_pkg_name(patch_info));
        let writer = LineWriter::new(
            std::fs::File::create(&pkg_spec_path)?
        );

        Self::write_patch_info(writer, patch_info, source_dir)?;

        Ok(pkg_spec_path)
    }
}
