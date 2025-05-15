#[doc = "Register `PWMEI` writer"]
pub type W = crate::W<PwmeiSpec>;
#[doc = "Determines whether the write accesses the Interrupt Enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableBit {
    #[doc = "1: Enable the Interrupt generation"]
    Enabled = 1,
}
impl From<EnableBit> for bool {
    #[inline(always)]
    fn from(variant: EnableBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_BIT` writer - Determines whether the write accesses the Interrupt Enable register"]
pub type EnableBitW<'a, REG> = crate::BitWriter<'a, REG, EnableBit>;
impl<'a, REG> EnableBitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Interrupt generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EnableBit::Enabled)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether the write accesses the Interrupt Enable register"]
    #[inline(always)]
    pub fn enable_bit(&mut self) -> EnableBitW<PwmeiSpec> {
        EnableBitW::new(self, 0)
    }
}
#[doc = "PWM Enable Interrupt Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmei::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmeiSpec;
impl crate::RegisterSpec for PwmeiSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwmei::W`](W) writer structure"]
impl crate::Writable for PwmeiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMEI to value 0"]
impl crate::Resettable for PwmeiSpec {
    const RESET_VALUE: u32 = 0;
}
