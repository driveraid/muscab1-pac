#[doc = "Register `TIMER2RIS` reader"]
pub type R = crate::R<Timer2risSpec>;
#[doc = "Field `RIS` reader - Raw Timer Interrupt"]
pub type RisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw Timer Interrupt"]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 2 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2risSpec;
impl crate::RegisterSpec for Timer2risSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2ris::R`](R) reader structure"]
impl crate::Readable for Timer2risSpec {}
#[doc = "`reset()` method sets TIMER2RIS to value 0"]
impl crate::Resettable for Timer2risSpec {
    const RESET_VALUE: u32 = 0;
}
