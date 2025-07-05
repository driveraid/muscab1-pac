#[doc = "Register `TIMER1RIS` reader"]
pub type R = crate::R<Timer1risSpec>;
#[doc = "Field `RIS` reader - Raw Timer Interrupt"]
pub type RisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw Timer Interrupt"]
    #[inline(always)]
    pub fn ris(&self) -> RisR {
        RisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 1 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1risSpec;
impl crate::RegisterSpec for Timer1risSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1ris::R`](R) reader structure"]
impl crate::Readable for Timer1risSpec {}
#[doc = "`reset()` method sets TIMER1RIS to value 0"]
impl crate::Resettable for Timer1risSpec {
    const RESET_VALUE: u32 = 0;
}
