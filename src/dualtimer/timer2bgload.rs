#[doc = "Register `TIMER2BGLOAD` reader"]
pub type R = crate::R<Timer2bgloadSpec>;
#[doc = "Register `TIMER2BGLOAD` writer"]
pub type W = crate::W<Timer2bgloadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer 2 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2bgload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2bgload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2bgloadSpec;
impl crate::RegisterSpec for Timer2bgloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2bgload::R`](R) reader structure"]
impl crate::Readable for Timer2bgloadSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2bgload::W`](W) writer structure"]
impl crate::Writable for Timer2bgloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2BGLOAD to value 0"]
impl crate::Resettable for Timer2bgloadSpec {
    const RESET_VALUE: u32 = 0;
}
