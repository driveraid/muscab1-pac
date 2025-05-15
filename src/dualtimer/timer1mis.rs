#[doc = "Register `TIMER1MIS` reader"]
pub type R = crate::R<Timer1misSpec>;
#[doc = "Field `MIS` reader - Masked Timer Interrupt"]
pub type MisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked Timer Interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 1 Mask Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1misSpec;
impl crate::RegisterSpec for Timer1misSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1mis::R`](R) reader structure"]
impl crate::Readable for Timer1misSpec {}
#[doc = "`reset()` method sets TIMER1MIS to value 0"]
impl crate::Resettable for Timer1misSpec {}
