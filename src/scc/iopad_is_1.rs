#[doc = "Register `IOPAD_IS_1` reader"]
pub type R = crate::R<IopadIs1Spec>;
#[doc = "Register `IOPAD_IS_1` writer"]
pub type W = crate::W<IopadIs1Spec>;
#[doc = "Field `input_select` reader - Selects input mode on test chip I/O PA37-PA32"]
pub type InputSelectR = crate::FieldReader;
#[doc = "Field `input_select` writer - Selects input mode on test chip I/O PA37-PA32"]
pub type InputSelectW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Selects input mode on test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn input_select(&self) -> InputSelectR {
        InputSelectR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selects input mode on test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn input_select(&mut self) -> InputSelectW<IopadIs1Spec> {
        InputSelectW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_is_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_is_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadIs1Spec;
impl crate::RegisterSpec for IopadIs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_is_1::R`](R) reader structure"]
impl crate::Readable for IopadIs1Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_is_1::W`](W) writer structure"]
impl crate::Writable for IopadIs1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_IS_1 to value 0xffff_ffff"]
impl crate::Resettable for IopadIs1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
