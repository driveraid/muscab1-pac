#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpuid: Cpuid,
}
impl RegisterBlock {
    #[doc = "0x00 - Unique CPU Identity Number, where N is 0 for CPU 0 and 1 for CPU 1. Set to zero for a single processor system."]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
}
#[doc = "CPUID (r) register accessor: Unique CPU Identity Number, where N is 0 for CPU 0 and 1 for CPU 1. Set to zero for a single processor system.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`]
module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "Unique CPU Identity Number, where N is 0 for CPU 0 and 1 for CPU 1. Set to zero for a single processor system."]
pub mod cpuid;
