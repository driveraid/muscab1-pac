#[doc = "Register `PWMDI` writer"]
pub type W = crate::W<PwmdiSpec>;
#[doc = "Determines whether the write accesses the Interrupt Disable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableBit {
    #[doc = "1: Disable the Interrupt generation"]
    Disabled = 1,
}
impl From<DisableBit> for bool {
    #[inline(always)]
    fn from(variant: DisableBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Disable_BIT` writer - Determines whether the write accesses the Interrupt Disable register"]
pub type DisableBitW<'a, REG> = crate::BitWriter<'a, REG, DisableBit>;
impl<'a, REG> DisableBitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisableBit::Disabled)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether the write accesses the Interrupt Disable register"]
    #[inline(always)]
    pub fn disable_bit(&mut self) -> DisableBitW<PwmdiSpec> {
        DisableBitW::new(self, 0)
    }
}
#[doc = "PWM Disable Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmdiSpec;
impl crate::RegisterSpec for PwmdiSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwmdi::W`](W) writer structure"]
impl crate::Writable for PwmdiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDI to value 0"]
impl crate::Resettable for PwmdiSpec {}
