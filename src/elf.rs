use anyhow::{bail, Context, Result};
use probe_rs_target::{
    ArmCoreAccessOptions, Chip, ChipFamily, Core, CoreAccessOptions, MemoryRegion, NvmRegion,
    RamRegion, TargetDescriptionSource::BuiltIn, CoreType,
};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, Write, Cursor},
    path::Path,
};

use crate::parser::extract_flash_algo;

/// Prepare a target config based on an ELF file containing a flash algorithm, and return a JSON manifest string
pub fn cmd_elf(
    file: &[u8],
    fixed_load_address: bool,
    algo_name: Option<String>,
    family_name: Option<String>,
    chip_name: Option<String>
) -> Result<String> {
    let elf_file = Cursor::new(file);

    let mut algorithm = extract_flash_algo(elf_file, "unknown", true, fixed_load_address)?;

    if let Some(name) = algo_name {
        algorithm.name = name;
    }

    // Create a complete target specification, with place holder values
    let algorithm_name = algorithm.name.clone();
    algorithm.cores = vec!["main".to_owned()];

    let chip_family = ChipFamily {
        name: family_name.unwrap_or_default(),
        manufacturer: None,
        generated_from_pack: false,
        pack_file_release: None,
        variants: vec![Chip {
            cores: vec![Core {
                name: "main".to_owned(),
                core_type: CoreType::Armv6m,
                core_access_options: CoreAccessOptions::Arm(ArmCoreAccessOptions {
                    ap: 0,
                    psel: 0,
                    debug_base: None,
                    cti_base: None,
                }),
            }],
            part: None,
            name: chip_name.unwrap_or_default(),
            memory_map: vec![
                MemoryRegion::Nvm(NvmRegion {
                    is_boot_memory: false,
                    range: 0..0x2000,
                    cores: vec!["main".to_owned()],
                    name: None,
                }),
                MemoryRegion::Ram(RamRegion {
                    is_boot_memory: true,
                    range: 0x1_0000..0x2_0000,
                    cores: vec!["main".to_owned()],
                    name: None,
                }),
            ],
            flash_algorithms: vec![algorithm_name],
        }],
        flash_algorithms: vec![algorithm],
        source: BuiltIn,
    };

    Ok(serde_json::to_string(&chip_family).unwrap())
}
