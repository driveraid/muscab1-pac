#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CpuidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Unique CPU Identity Number, where N is 0 for CPU 0 and 1 for CPU 1. Set to zero for a single processor system.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuidSpec;
impl crate::RegisterSpec for CpuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CpuidSpec {}
#[doc = "`reset()` method sets CPUID to value 0"]
impl crate::Resettable for CpuidSpec {
    const RESET_VALUE: u32 = 0;
}
