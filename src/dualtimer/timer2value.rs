#[doc = "Register `TIMER2VALUE` reader"]
pub type R = crate::R<Timer2valueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Timer 2 Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2valueSpec;
impl crate::RegisterSpec for Timer2valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2value::R`](R) reader structure"]
impl crate::Readable for Timer2valueSpec {}
#[doc = "`reset()` method sets TIMER2VALUE to value 0xffff_ffff"]
impl crate::Resettable for Timer2valueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
