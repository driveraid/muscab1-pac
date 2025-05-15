#[doc = "Register `IOPAD_PS_1` reader"]
pub type R = crate::R<IopadPs1Spec>;
#[doc = "Register `IOPAD_PS_1` writer"]
pub type W = crate::W<IopadPs1Spec>;
#[doc = "Field `pull_select` reader - Enables pull resistors of test chip I/O PA37-PA32"]
pub type PullSelectR = crate::FieldReader;
#[doc = "Field `pull_select` writer - Enables pull resistors of test chip I/O PA37-PA32"]
pub type PullSelectW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Enables pull resistors of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn pull_select(&self) -> PullSelectR {
        PullSelectR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Enables pull resistors of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn pull_select(&mut self) -> PullSelectW<IopadPs1Spec> {
        PullSelectW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ps_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ps_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadPs1Spec;
impl crate::RegisterSpec for IopadPs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_ps_1::R`](R) reader structure"]
impl crate::Readable for IopadPs1Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_ps_1::W`](W) writer structure"]
impl crate::Writable for IopadPs1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_PS_1 to value 0xffff_ffff"]
impl crate::Resettable for IopadPs1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
