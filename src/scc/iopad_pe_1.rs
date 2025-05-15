#[doc = "Register `IOPAD_PE_1` reader"]
pub type R = crate::R<IopadPe1Spec>;
#[doc = "Register `IOPAD_PE_1` writer"]
pub type W = crate::W<IopadPe1Spec>;
#[doc = "Field `pull_enable` reader - Enables pull resistors of test chip I/O PA37-PA32"]
pub type PullEnableR = crate::FieldReader;
#[doc = "Field `pull_enable` writer - Enables pull resistors of test chip I/O PA37-PA32"]
pub type PullEnableW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Enables pull resistors of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PullEnableR {
        PullEnableR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Enables pull resistors of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn pull_enable(&mut self) -> PullEnableW<IopadPe1Spec> {
        PullEnableW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_pe_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_pe_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadPe1Spec;
impl crate::RegisterSpec for IopadPe1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_pe_1::R`](R) reader structure"]
impl crate::Readable for IopadPe1Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_pe_1::W`](W) writer structure"]
impl crate::Writable for IopadPe1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_PE_1 to value 0xffff_ffff"]
impl crate::Resettable for IopadPe1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
