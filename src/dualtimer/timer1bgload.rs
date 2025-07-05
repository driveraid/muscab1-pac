#[doc = "Register `TIMER1BGLOAD` reader"]
pub type R = crate::R<Timer1bgloadSpec>;
#[doc = "Register `TIMER1BGLOAD` writer"]
pub type W = crate::W<Timer1bgloadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer 1 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1bgload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1bgload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1bgloadSpec;
impl crate::RegisterSpec for Timer1bgloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1bgload::R`](R) reader structure"]
impl crate::Readable for Timer1bgloadSpec {}
#[doc = "`write(|w| ..)` method takes [`timer1bgload::W`](W) writer structure"]
impl crate::Writable for Timer1bgloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1BGLOAD to value 0"]
impl crate::Resettable for Timer1bgloadSpec {
    const RESET_VALUE: u32 = 0;
}
