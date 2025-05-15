#[doc = "Register `TIMER2MIS` reader"]
pub type R = crate::R<Timer2misSpec>;
#[doc = "Field `MIS` reader - Masked Timer Interrupt"]
pub type MisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked Timer Interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 2 Mask Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2misSpec;
impl crate::RegisterSpec for Timer2misSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2mis::R`](R) reader structure"]
impl crate::Readable for Timer2misSpec {}
#[doc = "`reset()` method sets TIMER2MIS to value 0"]
impl crate::Resettable for Timer2misSpec {}
