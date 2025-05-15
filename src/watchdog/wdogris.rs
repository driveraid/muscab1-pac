#[doc = "Register `WDOGRIS` reader"]
pub type R = crate::R<WdogrisSpec>;
#[doc = "Field `RIS` reader - Raw watchdog Interrupt"]
pub type RisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw watchdog Interrupt"]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogrisSpec;
impl crate::RegisterSpec for WdogrisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogris::R`](R) reader structure"]
impl crate::Readable for WdogrisSpec {}
#[doc = "`reset()` method sets WDOGRIS to value 0"]
impl crate::Resettable for WdogrisSpec {}
