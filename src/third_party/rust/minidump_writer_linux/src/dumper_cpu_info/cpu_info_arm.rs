use crate::cpu_set::*;
use crate::errors::CpuInfoError;
use crate::minidump_format::*;
use crate::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path;

type Result<T> = std::result::Result<T, CpuInfoError>;

struct CpuInfoEntry {
    field: &'static str,
    format: char,
    bit_lshift: u8,
    bit_length: u8,
}

impl CpuInfoEntry {
    fn new(field: &'static str, format: char, bit_lshift: u8, bit_length: u8) -> Self {
        CpuInfoEntry {
            field,
            format,
            bit_lshift,
            bit_length,
        }
    }
}

struct CpuFeaturesEntry {
    tag: &'static str,
    hwcaps: u32,
}

impl CpuFeaturesEntry {
    fn new(tag: &'static str, hwcaps: u32) -> Self {
        CpuFeaturesEntry { tag, hwcaps }
    }
}

pub fn write_cpu_information(sys_info: &mut MDRawSystemInfo) -> Result<()> {
    // The CPUID value is broken up in several entries in /proc/cpuinfo.
    // This table is used to rebuild it from the entries.
    let cpu_id_entries = [
        CpuInfoEntry::new("CPU implementer", 'x', 24, 8),
        CpuInfoEntry::new("CPU variant", 'x', 20, 4),
        CpuInfoEntry::new("CPU part", 'x', 4, 12),
        CpuInfoEntry::new("CPU revision", 'd', 0, 4),
    ];

    // The ELF hwcaps are listed in the "Features" entry as textual tags.
    // This table is used to rebuild them.
    let cpu_features_entries;
    #[cfg(target_arch = "arm")]
    {
        cpu_features_entries = [
            CpuFeaturesEntry::new("swp", MDCPUInformationARMElfHwCaps::Swp as u32),
            CpuFeaturesEntry::new("half", MDCPUInformationARMElfHwCaps::Half as u32),
            CpuFeaturesEntry::new("thumb", MDCPUInformationARMElfHwCaps::Thumb as u32),
            CpuFeaturesEntry::new("bit26", MDCPUInformationARMElfHwCaps::Bit26 as u32),
            CpuFeaturesEntry::new("fastmult", MDCPUInformationARMElfHwCaps::FastMult as u32),
            CpuFeaturesEntry::new("fpa", MDCPUInformationARMElfHwCaps::Fpa as u32),
            CpuFeaturesEntry::new("vfp", MDCPUInformationARMElfHwCaps::Vfp as u32),
            CpuFeaturesEntry::new("edsp", MDCPUInformationARMElfHwCaps::Edsp as u32),
            CpuFeaturesEntry::new("java", MDCPUInformationARMElfHwCaps::Java as u32),
            CpuFeaturesEntry::new("iwmmxt", MDCPUInformationARMElfHwCaps::Iwmmxt as u32),
            CpuFeaturesEntry::new("crunch", MDCPUInformationARMElfHwCaps::Crunch as u32),
            CpuFeaturesEntry::new("thumbee", MDCPUInformationARMElfHwCaps::Thumbee as u32),
            CpuFeaturesEntry::new("neon", MDCPUInformationARMElfHwCaps::Neon as u32),
            CpuFeaturesEntry::new("vfpv3", MDCPUInformationARMElfHwCaps::Vfpv3 as u32),
            CpuFeaturesEntry::new("vfpv3d16", MDCPUInformationARMElfHwCaps::Vfpv3d16 as u32),
            CpuFeaturesEntry::new("tls", MDCPUInformationARMElfHwCaps::Tls as u32),
            CpuFeaturesEntry::new("vfpv4", MDCPUInformationARMElfHwCaps::Vfpv4 as u32),
            CpuFeaturesEntry::new("idiva", MDCPUInformationARMElfHwCaps::Idiva as u32),
            CpuFeaturesEntry::new("idivt", MDCPUInformationARMElfHwCaps::Idivt as u32),
            CpuFeaturesEntry::new(
                "idiv",
                MDCPUInformationARMElfHwCaps::Idiva as u32
                    | MDCPUInformationARMElfHwCaps::Idivt as u32,
            ),
        ];
    }
    #[cfg(target_arch = "aarch64")]
    {
        // No hwcaps on aarch64.
        cpu_features_entries = [];
    }

    // processor_architecture should always be set, do this first
    if cfg!(target_arch = "aarch64") {
        sys_info.processor_architecture = MDCPUArchitecture::Arm64Old as u16;
    } else {
        sys_info.processor_architecture = MDCPUArchitecture::Arm as u16;
    }

    // /proc/cpuinfo is not readable under various sandboxed environments
    // (e.g. Android services with the android:isolatedProcess attribute)
    // prepare for this by setting default values now, which will be
    // returned when this happens.
    //
    // Note: Bogus values are used to distinguish between failures (to
    //       read /sys and /proc files) and really badly configured kernels.
    sys_info.number_of_processors = 0;
    sys_info.processor_level = 1; // There is no ARMv1
    sys_info.processor_revision = 42;
    sys_info.cpu.cpuid = 0;
    sys_info.cpu.elf_hwcaps = 0;

    // Counting the number of CPUs involves parsing two sysfs files,
    // because the content of /proc/cpuinfo will only mirror the number
    // of 'online' cores, and thus will vary with time.
    // See http://www.kernel.org/doc/Documentation/cputopology.txt
    let mut cpus_present = CpuSet::new();
    let mut cpus_possible = CpuSet::new();

    if let Ok(mut present_file) = File::open("/sys/devices/system/cpu/present") {
        // Ignore unparsable content
        let _ = cpus_present.parse_sys_file(&mut present_file);

        if let Ok(mut possible_file) = File::open("/sys/devices/system/cpu/possible") {
            // Ignore unparsable content
            let _ = cpus_possible.parse_sys_file(&mut possible_file);
            cpus_present.intersect_with(&cpus_possible);
            let cpu_count = std::cmp::min(255, cpus_present.get_count()) as u8;
            sys_info.number_of_processors = cpu_count;
        }
    }

    // Parse /proc/cpuinfo to reconstruct the CPUID value, as well
    // as the ELF hwcaps field. For the latter, it would be easier to
    // read /proc/self/auxv but unfortunately, this file is not always
    // readable from regular Android applications on later versions
    // (>= 4.1) of the Android platform.

    let cpuinfo_file = match File::open(path::PathBuf::from("/proc/cpuinfo")) {
        Ok(x) => x,
        Err(_) => {
            // Do not return Error here to allow the minidump generation
            // to happen properly.
            return Ok(());
        }
    };

    for line in BufReader::new(cpuinfo_file).lines() {
        let line = line?;
        // Expected format: <field-name> <space>+ ':' <space> <value>
        // Note that:
        //   - empty lines happen.
        //   - <field-name> can contain spaces.
        //   - some fields have an empty <value>
        if line.trim().is_empty() {
            continue;
        }

        let split: Vec<_> = line.split(":").map(|x| x.trim()).collect();
        let field = split[0];
        let value = split.get(1); // Option, might be missing
        for entry in &cpu_id_entries {
            if value.is_none() {
                break;
            }
            if field != entry.field {
                continue;
            }
            let mut result;
            let val = value.unwrap();

            let rr = if val.starts_with("0x") || entry.format == 'x' {
                usize::from_str_radix(val.trim_start_matches("0x"), 16)
            } else {
                usize::from_str_radix(val, 10)
            };
            result = match rr {
                Ok(x) => x,
                Err(_) => {
                    continue;
                }
            };
            result &= (1 << entry.bit_length) - 1;
            result <<= entry.bit_lshift;
            sys_info.cpu.cpuid |= result as u32;
        }

        if cfg!(target_arch = "arm") {
            // Get the architecture version from the "Processor" field.
            // Note that it is also available in the "CPU architecture" field,
            // however, some existing kernels are misconfigured and will report
            // invalid values here (e.g. 6, while the CPU is ARMv7-A based).
            // The "Processor" field doesn't have this issue.
            if field == "Processor" {
                // Expected format: <text> (v<level><endian>)
                // Where <text> is some text like "ARMv7 Processor rev 2"
                // and <level> is a decimal corresponding to the ARM
                // architecture number. <endian> is either 'l' or 'b'
                // and corresponds to the endianess, it is ignored here.
                if let Some(val) = value.and_then(|v| v.split_whitespace().last()) {
                    // val is now something like "(v7l)"
                    sys_info.processor_level = val[2..val.len() - 2].parse::<u16>().unwrap_or(5);
                }
            }
        } else {
            // aarch64
            // The aarch64 architecture does not provide the architecture level
            // in the Processor field, so we instead check the "CPU architecture"
            // field.
            if field == "CPU architecture" {
                sys_info.processor_level = match value.and_then(|v| v.parse::<u16>().ok()) {
                    Some(v) => v,
                    None => {
                        continue;
                    }
                };
            }
        }

        // Rebuild the ELF hwcaps from the 'Features' field.
        if field == "Features" {
            if let Some(val) = value {
                // Parse each space-separated tag.
                for tag in val.split_whitespace() {
                    for entry in &cpu_features_entries {
                        if entry.tag == tag {
                            sys_info.cpu.elf_hwcaps |= entry.hwcaps;
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
