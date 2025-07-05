#[doc = "Register `TIMER2LOAD` reader"]
pub type R = crate::R<Timer2loadSpec>;
#[doc = "Register `TIMER2LOAD` writer"]
pub type W = crate::W<Timer2loadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer 2 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2loadSpec;
impl crate::RegisterSpec for Timer2loadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2load::R`](R) reader structure"]
impl crate::Readable for Timer2loadSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2load::W`](W) writer structure"]
impl crate::Writable for Timer2loadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2LOAD to value 0"]
impl crate::Resettable for Timer2loadSpec {
    const RESET_VALUE: u32 = 0;
}
