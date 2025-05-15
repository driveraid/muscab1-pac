#[doc = "Register `WDOGINTCLR` writer"]
pub type W = crate::W<WdogintclrSpec>;
#[doc = "Field `INT` writer - Interrupt"]
pub type IntW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<WdogintclrSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Watchdog Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogintclrSpec;
impl crate::RegisterSpec for WdogintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdogintclr::W`](W) writer structure"]
impl crate::Writable for WdogintclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets WDOGINTCLR to value 0"]
impl crate::Resettable for WdogintclrSpec {}
