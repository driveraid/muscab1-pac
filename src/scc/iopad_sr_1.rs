#[doc = "Register `IOPAD_SR_1` reader"]
pub type R = crate::R<IopadSr1Spec>;
#[doc = "Register `IOPAD_SR_1` writer"]
pub type W = crate::W<IopadSr1Spec>;
#[doc = "Field `slew_rate` reader - Selects the slew rate of test chip I/O PA37-PA32"]
pub type SlewRateR = crate::FieldReader;
#[doc = "Field `slew_rate` writer - Selects the slew rate of test chip I/O PA37-PA32"]
pub type SlewRateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Selects the slew rate of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn slew_rate(&self) -> SlewRateR {
        SlewRateR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selects the slew rate of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn slew_rate(&mut self) -> SlewRateW<IopadSr1Spec> {
        SlewRateW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_sr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_sr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadSr1Spec;
impl crate::RegisterSpec for IopadSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_sr_1::R`](R) reader structure"]
impl crate::Readable for IopadSr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_sr_1::W`](W) writer structure"]
impl crate::Writable for IopadSr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_SR_1 to value 0xffff_ffff"]
impl crate::Resettable for IopadSr1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
