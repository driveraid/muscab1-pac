#[doc = "Register `IOPAD_SR_0` reader"]
pub type R = crate::R<IopadSr0Spec>;
#[doc = "Register `IOPAD_SR_0` writer"]
pub type W = crate::W<IopadSr0Spec>;
#[doc = "Field `slew_rate` reader - Selects the slew rate of test chip I/O PA31-PA0"]
pub type SlewRateR = crate::FieldReader<u32>;
#[doc = "Field `slew_rate` writer - Selects the slew rate of test chip I/O PA31-PA0"]
pub type SlewRateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Selects the slew rate of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn slew_rate(&self) -> SlewRateR {
        SlewRateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Selects the slew rate of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn slew_rate(&mut self) -> SlewRateW<IopadSr0Spec> {
        SlewRateW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_sr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_sr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadSr0Spec;
impl crate::RegisterSpec for IopadSr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_sr_0::R`](R) reader structure"]
impl crate::Readable for IopadSr0Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_sr_0::W`](W) writer structure"]
impl crate::Writable for IopadSr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_SR_0 to value 0xffff_ffff"]
impl crate::Resettable for IopadSr0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
