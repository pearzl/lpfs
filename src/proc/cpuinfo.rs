//! /proc/cpuinfo
//!
//! > This virtual file identifies the type of processor used by your system. The following is an example of the output typical of `/proc/cpuinfo`:
//! > 
//! > <pre class="screen">processor	: 0
//! > vendor_id	: GenuineIntel
//! > cpu family	: 15
//! > model		: 2
//! > model name	: Intel(R) Xeon(TM) CPU 2.40GHz
//! > stepping	: 7 cpu
//! > MHz		: 2392.371
//! > cache size	: 512 KB
//! > physical id	: 0
//! > siblings	: 2
//! > runqueue	: 0
//! > fdiv_bug	: no
//! > hlt_bug		: no
//! > f00f_bug	: no
//! > coma_bug	: no
//! > fpu		: yes
//! > fpu_exception	: yes
//! > cpuid level	: 2
//! > wp		: yes
//! > flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca  cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm
//! > bogomips	: 4771.02
//! > </pre>
//! > 
//! > *   `processor` — Provides each processor with an identifying number. On systems that have one processor, only a `0` is present.
//! > 
//! > *   `cpu family` — Authoritatively identifies the type of processor in the system. For an Intel-based system, place the number in front of "86" to determine the value. This is particularly helpful for those attempting to identify the architecture of an older system such as a 586, 486, or 386\. Because some RPM packages are compiled for each of these particular architectures, this value also helps users determine which packages to install.
//! > 
//! > *   `model name` — Displays the common name of the processor, including its project name.
//! > 
//! > *   `cpu MHz` — Shows the precise speed in megahertz for the processor to the thousandths decimal place.
//! > 
//! > *   `cache size` — Displays the amount of level 2 memory cache available to the processor.
//! > 
//! > *   `siblings` — Displays the total number of sibling CPUs on the same physical CPU for architectures which use hyper-threading.
//! > 
//! > *   `flags` — Defines a number of different qualities about the processor, such as the presence of a floating point unit (FPU) and the ability to process MMX instructions.
//! >
//! > -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/s2-proc-cpuinfo
//! >
//! 
//! 
//!
//! > This is a collection of CPU and system architecture dependent items, for each supported architecture a different list.
//! > Two common entries are _processor_ which gives CPU number and _bogomips_; a system constant that is calculated during kernel initialization. 
//! > SMP machines have information for each CPU. The [lscpu(1)](https://manpages.debian.org/testing/util-linux/lscpu.1.en.html) command gathers its information from this file.
//! >
//! > -- https://manpages.debian.org/testing/manpages/procfs.5.en.html#Files_and_directories
//! 

define_struct!{
    /// Each instance of this struct represent a block in /proc/cpuinfo.
    ///
    /// Fields of this struct reference to 
    /// [proc.c](https://github.com/torvalds/linux/blob/master/arch/x86/kernel/cpu/proc.c#L57).
    /// 
    /// *Note: this is for x86_64, other architecture may not work.*
    #[cfg_attr(test, derive(Default))]
    pub struct Processor {
        processor: u8,
        vendor_id: String,
        cpu_family: u8,
        model: u8,
        model_name: String,
        /// Return None if `stepping` is not number or the fields doesn't appear.
        /// In first case, it should be "unknonw", but it doesn't actually be checked.
        stepping: Option<u8>,
        microcode: Option<u32>,
        cpu_mhz: Option<f32>,
        /// Unit is KB(KiB).
        cache_size: Option<u32>,
        
        // SMP fileds
        physical_id: Option<u16>,
        siblings: Option<u32>,
        core_id: Option<u16>,
        cpu_cores: Option<u16>,
        apicid: Option<u16>,
        initial_apicid: Option<u16>,

        // MISC fileds
        fdiv_bug: Option<bool>,
        f00f_bug: Option<bool>,
        coma_bug: Option<bool>,
        /// always true if appears.
        fpu: Option<bool>,
        /// always true if appears.
        fpu_exception: Option<bool>,
        cpuid_level: Option<i32>,
        /// always true if appears.
        wp: Option<bool>,

        flags: Vec<String>,
        bugs: Vec<String>,
        bogomips: f32,
        
        tlb_size: Option<i32>,

        clflush_size: u16,
        cache_alignment: i32,
        /// (bits physical, bits virtual)
        address_sizes: (u8, u8),
        power_management: Vec<String>,
    }
}

use std::collections::HashMap;

use std::str::FromStr;
impl FromStr for Processor {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Processor, crate::ProcErr> {
        let mut map = HashMap::new();
        for line in s.trim().lines() {
            let columns: Vec<&str> = line.split(':').collect();
            if columns.len() != 2 {
                return Err(bfe!(format!("`{}` is not k:v pair", line)));
            }
            map.insert(columns[0].trim(), columns[1].trim());
        }

        macro_rules! unwrap_string {
            ($field: ident, $key: expr) => {
                let $field = if let Some(vid) = map.get($key) {
                    vid.to_string()
                }else {
                    return Err(bfe!(format!("`{}` field not found", $key)))
                };
            }
        }

        macro_rules! unwrap_number {
            ($field: ident, $key: expr, $type: ty) => {
                let $field = if let Some(p) = map.get($key) {
                    p.parse::<$type>()?
                }else {
                    return Err(bfe!(format!("`{}` field not found", $key)))
                };
            }
        }

        macro_rules! unwrap_opt_number {
            ($field: ident, $key: expr, $type: ty) => {
                let $field = match map.get($key){
                    Some(v) => {
                        let value = v.parse::<$type>()?;
                        Some(value)
                    },
                    None => None
                };
            }
        }

        macro_rules! unwrap_opt_bool {
            ($field: ident, $key: expr) => {
                let $field = {
                    match map.get($key) {
                        Some(&"yes") => Some(true),
                        Some(&"no") => Some(false),
                        Some(_) => return Err(bfe!(format!("unknow value for `{}` field", $key))),
                        None => None
                    }
                };
            }
        }


        unwrap_number!(processor, "processor", u8);
        unwrap_string!(vendor_id, "vendor_id");
        unwrap_number!(cpu_family, "cpu family", u8);
        unwrap_number!(model, "model", u8);
        unwrap_string!(model_name, "model name");
        unwrap_opt_number!(stepping, "stepping", u8);
        
        let microcode = if let Some(v) = map.get("microcode") {
            let value = u32::from_str_radix(&v[2..], 16)?;
            Some(value)
        }else {
            None
        };
        
        unwrap_opt_number!(cpu_mhz, "cpu MHz", f32);

        let cache_size = if let Some(v) = map.get("cache size") {
            let value = v[..v.len()-3].parse::<u32>()?;
            Some(value)
        }else {
            None
        };

        unwrap_opt_number!(physical_id, "physical id", u16);
        unwrap_opt_number!(siblings, "siblings", u32);
        unwrap_opt_number!(core_id, "core id", u16);
        unwrap_opt_number!(cpu_cores, "cpu cores", u16);
        unwrap_opt_number!(apicid, "apicid", u16);
        unwrap_opt_number!(initial_apicid, "initial apicid", u16);
        unwrap_opt_bool!(fdiv_bug, "fdiv_bug");
        unwrap_opt_bool!(f00f_bug, "foof_bug");
        unwrap_opt_bool!(coma_bug, "coma_bug");
        unwrap_opt_bool!(fpu, "fpu");
        unwrap_opt_bool!(fpu_exception, "fpu_exception");
        unwrap_opt_number!(cpuid_level, "cpuid level", i32);
        unwrap_opt_bool!(wp, "wp");

        let flags = map.get("flags")
            .map(|s| s.split(' ')
                .filter(|item| *item != "")
                .map(String::from)
                .collect())
            .unwrap_or(vec![]);
        let bugs = map.get("bugs")
            .map(|s| s.split(' ')
                .filter(|item| *item != "")
                .map(String::from)
                .collect())
            .unwrap_or(vec![]);
        
        unwrap_number!(bogomips, "bogomips", f32);
        unwrap_opt_number!(tlb_size, "TLB size", i32);
        unwrap_number!(clflush_size, "clflush size", u16);
        unwrap_number!(cache_alignment, "cache_alignment", i32);
        
        unwrap_string!(address_sizes, "address sizes");
        let address_sizes = {
            let columns: Vec<&str> = address_sizes.split(' ').collect();
            if columns.len() != 6 {
                return Err(bfe!("unknow value for `address sizes`".to_string()))
            }
            (
                columns[0].parse::<u8>()?,
                columns[3].parse::<u8>()?
            )
        };

        let power_management = map.get("power management")
            .map(|s| s.split(' ')
                .filter(|item| *item != "")
                .map(String::from)
                .collect())
            .unwrap_or(vec![]);
        
        Ok(Processor{
            processor, vendor_id,  cpu_family, model, model_name,
            stepping, microcode, cpu_mhz, cache_size,
            physical_id, siblings, core_id, cpu_cores,apicid, initial_apicid,
            fdiv_bug, f00f_bug, coma_bug, fpu, fpu_exception, cpuid_level, wp,
            flags, bugs, bogomips, tlb_size, clflush_size, cache_alignment,
            address_sizes, power_management
        })
    }
}

define_struct! {
    /// represent the content of the /proc/cpuinfo
    /// 
    /// It is an Vec<(Process)[struct.Processor.html]> actually, but provides two useful method :
    /// (logical_core_num())[#method.logical_core_num] and
    /// (physical_core_num())[#method.physical_core_num]
    /// 
    /// This struct implement Index trait.
    pub struct CpuInfo {
        #[doc(hidden)]
        entry: Vec<Processor>,
    }
}

impl CpuInfo {
    /// Return logical core number
    /// 
    /// This is the numbers of the entry in /proc/cpuinfo.
    pub fn logical_core_num(&self) -> usize {
        self.entry.len()
    }

    /// Return physical core number
    /// 
    /// This is the `cpu_cores` from first Processor.
    /// 
    /// *Note: This method will panic if `cpu cores` doesn't appear.*
    /// Field `cpu cores` only appears on SMP machine. 
    /// In this case, physical core numbers equals to logical core numbers.
    pub fn physical_core_num(&self) -> usize {
        self.entry[0].cpu_cores().unwrap() as usize
    }

}

use std::ops::Index;
impl Index<usize> for CpuInfo {
    type Output = Processor;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entry[index]
    }
}

impl FromStr for CpuInfo {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<CpuInfo, crate::ProcErr> {
        let mut entry = vec![];
        for block in s.split("\n\n") {
            let p = Processor::from_str(block)?;
            entry.push(p);
        }
        Ok(CpuInfo{entry})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const S1: &'static str = {
"\
processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 26
model name      : Intel(R) Xeon(R) CPU           X5570  @ 2.93GHz
stepping        : 5
cpu MHz         : 2933.583
cache size      : 8192 KB
physical id     : 0
siblings        : 8
core id         : 0
cpu cores       : 4
apicid          : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 11
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm syscall nx rdtscp lm const ant_tsc ida pni monitor ds_cpl vmx est tm2 cx16 xtpr popcnt lahf_lm
bogomips        : 5871.08
clflush size    : 64
cache_alignment : 64
address sizes   : 40 bits physical, 48 bits virtual
power management:"
    };

    lazy_static! {
        static ref P1: Processor = Processor {
            processor: 0,
            vendor_id: String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 26,
            model_name      : String::from("Intel(R) Xeon(R) CPU           X5570  @ 2.93GHz"),
            stepping        : Some(5),
            cpu_mhz         : Some(2933.583),
            cache_size      : Some(8192),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(0),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(11),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("syscall"), String::from("nx"), String::from("rdtscp"), String::from("lm"), String::from("const"), String::from("ant_tsc"), String::from("ida"), String::from("pni"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("est"), String::from("tm2"), String::from("cx16"), String::from("xtpr"), String::from("popcnt"), String::from("lahf_lm")],
            bogomips        : 5871.08f32,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (40, 48),
            power_management: vec![],
            ..Default::default()
        };
    }

    const S2: &'static str = {
"\
processor       : 1
vendor_id       : GenuineIntel
cpu family      : 6
model           : 26
model name      : Intel(R) Xeon(R) CPU           X5570  @ 2.93GHz
stepping        : 5
cpu MHz         : 2933.583
cache size      : 8192 KB
physical id     : 0
siblings        : 8
core id         : 1
cpu cores       : 4
apicid          : 2
fpu             : yes
fpu_exception   : yes
cpuid level     : 11
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm syscall nx rdtscp lm const ant_tsc ida pni monitor ds_cpl vmx est tm2 cx16 xtpr popcnt lahf_lm
bogomips        : 5866.74
clflush size    : 64
cache_alignment : 64
address sizes   : 40 bits physical, 48 bits virtual
power management:"      
    };

    lazy_static!{
        static ref P2: Processor = Processor {
            processor       : 1,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 26,
            model_name      : String::from("Intel(R) Xeon(R) CPU           X5570  @ 2.93GHz"),
            stepping        : Some(5),
            cpu_mhz         : Some(2933.583),
            cache_size      : Some(8192),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(1),
            cpu_cores       : Some(4),
            apicid          : Some(2),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(11),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("syscall"), String::from("nx"), String::from("rdtscp"), String::from("lm"), String::from("const"), String::from("ant_tsc"), String::from("ida"), String::from("pni"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("est"), String::from("tm2"), String::from("cx16"), String::from("xtpr"), String::from("popcnt"), String::from("lahf_lm")],
            bogomips        : 5866.74f32,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (40, 48),
            power_management: vec![],
            ..Default::default()
        };
    }

    #[test]
    fn test_parse_processor() {
        assert_eq!(*P1, Processor::from_str(S1).unwrap());
        assert_eq!(*P2, Processor::from_str(S2).unwrap());
    }

    #[test]
    fn test_index_impl() {
        let cpuinfo = CpuInfo{entry: vec![P1.clone(), P2.clone()]};
        assert_eq!(cpuinfo[0], P1.clone());
        assert_eq!(cpuinfo[1], P2.clone());
    }

    const SI0: &'static str = {"\
processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 0
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 1
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 0
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 2
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 1
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 3
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 1
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 4
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 2
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 5
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 2
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 6
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 3
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 7
vendor_id       : GenuineIntel
cpu family      : 6
model           : 94
model name      : Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz
stepping        : 3
microcode       : 0xffffffff
cpu MHz         : 3401.000
cache size      : 256 KB
physical id     : 0
siblings        : 8
core id         : 3
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 6
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave osxsave avx f16c rdrand lahf_lm abm 3dnowprefetch fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt ibrs ibpb stibp ssbd
bogomips        : 6802.00
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual    
"
    };

    lazy_static! {
        static ref SI0_P0: Processor = Processor {
            processor       : 0,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(0),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    }

    lazy_static! {
        static ref SI0_P1: Processor = Processor {
            processor       : 1,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(0),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    }

    lazy_static! {
        static ref SI0_P2: Processor = Processor {
            processor       : 2,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(1),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    }

    lazy_static! {
        static ref SI0_P3: Processor = Processor {
            processor       : 3,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(1),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    
    }

    lazy_static! {
        static ref SI0_P4: Processor = Processor {
            processor       : 4,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(2),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    
    }

    lazy_static! {
        static ref SI0_P5: Processor = Processor {
            processor       : 5,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(2),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    
    }

    lazy_static! {
        static ref SI0_P6: Processor = Processor {
            processor       : 6,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(3),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    
    }

    lazy_static! {
        static ref SI0_P7: Processor = Processor {
            processor       : 7,
            vendor_id       : String::from("GenuineIntel"),
            cpu_family      : 6,
            model           : 94,
            model_name      : String::from("Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz"),
            stepping        : Some(3),
            microcode       : Some(4294967295),
            cpu_mhz         : Some(3401.000),
            cache_size      : Some(256),
            physical_id     : Some(0),
            siblings        : Some(8),
            core_id         : Some(3),
            cpu_cores       : Some(4),
            apicid          : Some(0),
            initial_apicid  : Some(0),
            fpu             : Some(true),
            fpu_exception   : Some(true),
            cpuid_level     : Some(6),
            wp              : Some(true),
            flags           : vec![String::from("fpu"), String::from("vme"), String::from("de"), String::from("pse"), String::from("tsc"), String::from("msr"), String::from("pae"), String::from("mce"), String::from("cx8"), String::from("apic"), String::from("sep"), String::from("mtrr"), String::from("pge"), String::from("mca"), String::from("cmov"), String::from("pat"), String::from("pse36"), String::from("clflush"), String::from("dts"), String::from("acpi"), String::from("mmx"), String::from("fxsr"), String::from("sse"), String::from("sse2"), String::from("ss"), String::from("ht"), String::from("tm"), String::from("pbe"), String::from("syscall"), String::from("nx"), String::from("pdpe1gb"), String::from("rdtscp"), String::from("lm"), String::from("pni"), String::from("pclmulqdq"), String::from("dtes64"), String::from("monitor"), String::from("ds_cpl"), String::from("vmx"), String::from("smx"), String::from("est"), String::from("tm2"), String::from("ssse3"), String::from("fma"), String::from("cx16"), String::from("xtpr"), String::from("pdcm"), String::from("pcid"), String::from("sse4_1"), String::from("sse4_2"), String::from("x2apic"), String::from("movbe"), String::from("popcnt"), String::from("tsc_deadline_timer"), String::from("aes"), String::from("xsave"), String::from("osxsave"), String::from("avx"), String::from("f16c"), String::from("rdrand"), String::from("lahf_lm"), String::from("abm"), String::from("3dnowprefetch"), String::from("fsgsbase"), String::from("tsc_adjust"), String::from("bmi1"), String::from("hle"), String::from("avx2"), String::from("smep"), String::from("bmi2"), String::from("erms"), String::from("invpcid"), String::from("rtm"), String::from("mpx"), String::from("rdseed"), String::from("adx"), String::from("smap"), String::from("clflushopt"), String::from("intel_pt"), String::from("ibrs"), String::from("ibpb"), String::from("stibp"), String::from("ssbd")],
            bogomips        : 6802.00,
            clflush_size    : 64,
            cache_alignment : 64,
            address_sizes   : (36, 48),
            power_management: vec![],
            ..Default::default()
        };
    
    }

    #[test]
    fn test_parse_cpuinfo() {
        let cpuinfo = CpuInfo::from_str(SI0).unwrap();
        assert_eq!(cpuinfo, CpuInfo{entry:vec![SI0_P0.clone(),SI0_P1.clone(),SI0_P2.clone(),SI0_P3.clone(),SI0_P4.clone(),SI0_P5.clone(),SI0_P6.clone(),SI0_P7.clone()]})
    }

    #[test]
    fn test_logical_core_num() {
        let cpuinfo = CpuInfo::from_str(SI0).unwrap();
        assert_eq!(8, cpuinfo.logical_core_num());
    }

    #[test]
    fn test_physical_core_num() {
        let cpuinfo = CpuInfo::from_str(SI0).unwrap();
        assert_eq!(4, cpuinfo.physical_core_num());
    }


}