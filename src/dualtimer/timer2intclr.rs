#[doc = "Register `TIMER2INTCLR` writer"]
pub type W = crate::W<Timer2intclrSpec>;
#[doc = "Field `INT` writer - Interrupt"]
pub type IntW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<Timer2intclrSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Timer 2 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2intclrSpec;
impl crate::RegisterSpec for Timer2intclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timer2intclr::W`](W) writer structure"]
impl crate::Writable for Timer2intclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets TIMER2INTCLR to value 0"]
impl crate::Resettable for Timer2intclrSpec {}
