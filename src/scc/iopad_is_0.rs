#[doc = "Register `IOPAD_IS_0` reader"]
pub type R = crate::R<IopadIs0Spec>;
#[doc = "Register `IOPAD_IS_0` writer"]
pub type W = crate::W<IopadIs0Spec>;
#[doc = "Field `input_select` reader - Selects input mode on test chip I/O PA31-PA0"]
pub type InputSelectR = crate::FieldReader<u32>;
#[doc = "Field `input_select` writer - Selects input mode on test chip I/O PA31-PA0"]
pub type InputSelectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Selects input mode on test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn input_select(&self) -> InputSelectR {
        InputSelectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Selects input mode on test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn input_select(&mut self) -> InputSelectW<IopadIs0Spec> {
        InputSelectW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_is_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_is_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadIs0Spec;
impl crate::RegisterSpec for IopadIs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_is_0::R`](R) reader structure"]
impl crate::Readable for IopadIs0Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_is_0::W`](W) writer structure"]
impl crate::Writable for IopadIs0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPAD_IS_0 to value 0xffff_ffff"]
impl crate::Resettable for IopadIs0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
