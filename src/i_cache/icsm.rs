#[doc = "Register `ICSM` reader"]
pub type R = crate::R<IcsmSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Instruction Cache Statistic Miss Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsmSpec;
impl crate::RegisterSpec for IcsmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsm::R`](R) reader structure"]
impl crate::Readable for IcsmSpec {}
#[doc = "`reset()` method sets ICSM to value 0"]
impl crate::Resettable for IcsmSpec {
    const RESET_VALUE: u32 = 0;
}
