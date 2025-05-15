#[doc = "Register `IOPAD_PE_0` reader"]
pub type R = crate::R<IopadPe0Spec>;
#[doc = "Register `IOPAD_PE_0` writer"]
pub type W = crate::W<IopadPe0Spec>;
#[doc = "Field `pull_enable` reader - Enables pull resistors of test chip I/O PA31-PA0"]
pub type PullEnableR = crate::FieldReader<u32>;
#[doc = "Field `pull_enable` writer - Enables pull resistors of test chip I/O PA31-PA0"]
pub type PullEnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enables pull resistors of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PullEnableR {
        PullEnableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enables pull resistors of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn pull_enable(&mut self) -> PullEnableW<IopadPe0Spec> {
        PullEnableW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_pe_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_pe_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadPe0Spec;
impl crate::RegisterSpec for IopadPe0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_pe_0::R`](R) reader structure"]
impl crate::Readable for IopadPe0Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_pe_0::W`](W) writer structure"]
impl crate::Writable for IopadPe0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPAD_PE_0 to value 0xffff_ffff"]
impl crate::Resettable for IopadPe0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
