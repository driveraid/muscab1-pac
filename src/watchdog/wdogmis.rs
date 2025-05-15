#[doc = "Register `WDOGMIS` reader"]
pub type R = crate::R<WdogmisSpec>;
#[doc = "Field `MIS` reader - Masked Watchdog Interrupt"]
pub type MisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked Watchdog Interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Mask Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogmisSpec;
impl crate::RegisterSpec for WdogmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogmis::R`](R) reader structure"]
impl crate::Readable for WdogmisSpec {}
#[doc = "`reset()` method sets WDOGMIS to value 0"]
impl crate::Resettable for WdogmisSpec {}
