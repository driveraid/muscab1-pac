#[doc = "Register `IOPAD_PS_0` reader"]
pub type R = crate::R<IopadPs0Spec>;
#[doc = "Register `IOPAD_PS_0` writer"]
pub type W = crate::W<IopadPs0Spec>;
#[doc = "Field `pull_select` reader - Enables pull resistors of test chip I/O PA31-PA0"]
pub type PullSelectR = crate::FieldReader<u32>;
#[doc = "Field `pull_select` writer - Enables pull resistors of test chip I/O PA31-PA0"]
pub type PullSelectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enables pull resistors of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn pull_select(&self) -> PullSelectR {
        PullSelectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enables pull resistors of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn pull_select(&mut self) -> PullSelectW<IopadPs0Spec> {
        PullSelectW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ps_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ps_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadPs0Spec;
impl crate::RegisterSpec for IopadPs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_ps_0::R`](R) reader structure"]
impl crate::Readable for IopadPs0Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_ps_0::W`](W) writer structure"]
impl crate::Writable for IopadPs0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_PS_0 to value 0xffff_ffff"]
impl crate::Resettable for IopadPs0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
