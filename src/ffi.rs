use libc::{c_int, c_char, uint8_t, uint32_t, int32_t};

pub static MAX_CPUID_LEVEL: uint = 32u;
pub static MAX_EXT_CPUID_LEVEL: uint = 32u;
pub static MAX_INTELFN4_LEVEL: uint = 4u;
pub static MAX_INTELFN11_LEVEL: uint = 4u;
pub static VENDOR_STR_MAX: uint = 16u;
pub static BRAND_STR_MAX: uint = 64u;
pub static CPU_FLAGS_MAX: uint = 128u;
pub static CPU_HINTS_MAX: uint = 16u;

#[repr(C)]
pub struct cpu_raw_data_t {
    pub basic_cpuid: [[uint32_t, ..MAX_CPUID_LEVEL], ..4u],
    pub ext_cpuid: [[uint32_t, ..MAX_EXT_CPUID_LEVEL], ..4u],
    pub intel_fn4: [[uint32_t, ..MAX_INTELFN4_LEVEL], ..4u],
    pub intel_fn11: [[uint32_t, ..MAX_INTELFN11_LEVEL], ..4u],
}

#[repr(C)]
pub struct cpu_id_t {
    pub vendor_str: [c_char, ..VENDOR_STR_MAX],
    pub brand_str: [c_char, ..BRAND_STR_MAX],
    pub vendor: int32_t,
    pub flags: [uint8_t, ..CPU_FLAGS_MAX],
    pub family: int32_t,
    pub model: int32_t,
    pub stepping: int32_t,
    pub ext_family: int32_t,
    pub ext_model: int32_t,
    pub num_cores: int32_t,
    pub num_logical_cpus: int32_t,
    pub total_logical_cpus: int32_t,
    pub l1_data_cache: int32_t,
    pub l1_instruction_cache: int32_t,
    pub l2_cache: int32_t,
    pub l3_cache: int32_t,
    pub l1_assoc: int32_t,
    pub l2_assoc: int32_t,
    pub l3_assoc: int32_t,
    pub l1_cacheline: int32_t,
    pub l2_cacheline: int32_t,
    pub l3_cacheline: int32_t,
    pub cpu_codename: [c_char, ..64],
    pub sse_size: int32_t,
    pub detection_hints: [uint8_t, ..CPU_HINTS_MAX],
}

#[link(name = "cpuid")]
extern {
    pub fn cpuid_present() -> c_int;
    pub fn cpuid_lib_version() -> *const c_char;
    pub fn cpuid_error() -> *const c_char;
    pub fn cpuid_get_raw_data(raw: *mut cpu_raw_data_t) -> c_int;
    // TODO: add cpu_identify
}
