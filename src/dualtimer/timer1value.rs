#[doc = "Register `TIMER1VALUE` reader"]
pub type R = crate::R<Timer1valueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Timer 1 Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1valueSpec;
impl crate::RegisterSpec for Timer1valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1value::R`](R) reader structure"]
impl crate::Readable for Timer1valueSpec {}
#[doc = "`reset()` method sets TIMER1VALUE to value 0xffff_ffff"]
impl crate::Resettable for Timer1valueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
