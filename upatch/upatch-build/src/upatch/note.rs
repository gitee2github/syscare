use std::path::Path;

use object::{write, Object, ObjectSection, SectionKind};

use super::{Error, Result};

pub fn create_note<P: AsRef<Path>, Q: AsRef<Path>>(debug_info: P, note: Q) -> Result<()> {
    let debug_info_elf = unsafe { memmap2::Mmap::map(&std::fs::File::open(debug_info)?)? };

    let in_object = match object::File::parse(&*debug_info_elf) {
        Ok(object) => object,
        Err(e) => return Err(Error::NOTES(format!("parse debug_info failed: {}", e))),
    };

    let mut out_object = write::Object::new(
        in_object.format(),
        in_object.architecture(),
        in_object.endianness(),
    );

    for section in in_object.sections() {
        if section.kind() != SectionKind::Note {
            continue;
        }

        let section_name = match section.name() {
            Ok(name) => name,
            Err(e) => return Err(Error::NOTES(format!("get note section name failed: {}", e))),
        };

        let section_id =
            out_object.add_section(vec![], section_name.as_bytes().to_vec(), section.kind());

        let out_section = out_object.section_mut(section_id);
        out_section.set_data(
            match section.data() {
                Ok(data) => data,
                Err(e) => return Err(Error::NOTES(format!("get note section data failed: {}", e))),
            },
            section.align(),
        );
        out_section.flags = section.flags();
    }

    let out_data = match out_object.write() {
        Ok(data) => data,
        Err(e) => {
            return Err(Error::NOTES(format!(
                "convert note section to data failed: {}",
                e
            )))
        }
    };

    std::fs::write(note, out_data)?;
    Ok(())
}

pub fn read_build_id<P: AsRef<Path>>(binary: P) -> std::io::Result<Option<Vec<u8>>> {
    let binary_elf = unsafe { memmap2::Mmap::map(&std::fs::File::open(binary)?)? };
    match object::File::parse(&*binary_elf) {
        Ok(object) => match object.build_id() {
            Ok(Some(id)) => Ok(Some(id.to_vec())),
            Ok(None) => Ok(None),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("read build_id failed: {}", e),
            )),
        },
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("parse binary failed: {}", e),
        )),
    }
}
