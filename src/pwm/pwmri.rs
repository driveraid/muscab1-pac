#[doc = "Register `PWMRI` reader"]
pub type R = crate::R<PwmriSpec>;
#[doc = "Check whether the Interrupt is Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableBit {
    #[doc = "1: Interrupt is Enabled"]
    Enabled = 1,
}
impl From<EnableBit> for bool {
    #[inline(always)]
    fn from(variant: EnableBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Enable_BIT` reader - Check whether the Interrupt is Enabled"]
pub type EnableBitR = crate::BitReader<EnableBit>;
impl EnableBitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EnableBit> {
        match self.bits {
            true => Some(EnableBit::Enabled),
            _ => None,
        }
    }
    #[doc = "Interrupt is Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EnableBit::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Check whether the Interrupt is Enabled"]
    #[inline(always)]
    pub fn enable_bit(&self) -> EnableBitR {
        EnableBitR::new((self.bits & 1) != 0)
    }
}
#[doc = "PWM Read Intr Enable Register.Reading from this address accesses the current state of the interrupt control registers\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmri::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmriSpec;
impl crate::RegisterSpec for PwmriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmri::R`](R) reader structure"]
impl crate::Readable for PwmriSpec {}
#[doc = "`reset()` method sets PWMRI to value 0"]
impl crate::Resettable for PwmriSpec {}
