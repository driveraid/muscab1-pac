#[doc = "Register `TIMER1LOAD` reader"]
pub type R = crate::R<Timer1loadSpec>;
#[doc = "Register `TIMER1LOAD` writer"]
pub type W = crate::W<Timer1loadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer 1 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1loadSpec;
impl crate::RegisterSpec for Timer1loadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1load::R`](R) reader structure"]
impl crate::Readable for Timer1loadSpec {}
#[doc = "`write(|w| ..)` method takes [`timer1load::W`](W) writer structure"]
impl crate::Writable for Timer1loadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER1LOAD to value 0"]
impl crate::Resettable for Timer1loadSpec {}
