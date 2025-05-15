#[doc = "Register `PWMCR` reader"]
pub type R = crate::R<PwmcrSpec>;
#[doc = "Register `PWMCR` writer"]
pub type W = crate::W<PwmcrSpec>;
#[doc = "Start stop bit for the pwm_output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputSet {
    #[doc = "1: Generate programmed waveform on pwm_output"]
    Enabled = 1,
    #[doc = "0: Set pwm_output continually high"]
    Disabled = 0,
}
impl From<OutputSet> for bool {
    #[inline(always)]
    fn from(variant: OutputSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTPUT_SET` reader - Start stop bit for the pwm_output"]
pub type OutputSetR = crate::BitReader<OutputSet>;
impl OutputSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutputSet {
        match self.bits {
            true => OutputSet::Enabled,
            false => OutputSet::Disabled,
        }
    }
    #[doc = "Generate programmed waveform on pwm_output"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OutputSet::Enabled
    }
    #[doc = "Set pwm_output continually high"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OutputSet::Disabled
    }
}
#[doc = "Field `OUTPUT_SET` writer - Start stop bit for the pwm_output"]
pub type OutputSetW<'a, REG> = crate::BitWriter<'a, REG, OutputSet>;
impl<'a, REG> OutputSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate programmed waveform on pwm_output"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OutputSet::Enabled)
    }
    #[doc = "Set pwm_output continually high"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OutputSet::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Start stop bit for the pwm_output"]
    #[inline(always)]
    pub fn output_set(&self) -> OutputSetR {
        OutputSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start stop bit for the pwm_output"]
    #[inline(always)]
    pub fn output_set(&mut self) -> OutputSetW<PwmcrSpec> {
        OutputSetW::new(self, 0)
    }
}
#[doc = "PWM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmcrSpec;
impl crate::RegisterSpec for PwmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcr::R`](R) reader structure"]
impl crate::Readable for PwmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmcr::W`](W) writer structure"]
impl crate::Writable for PwmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMCR to value 0"]
impl crate::Resettable for PwmcrSpec {
    const RESET_VALUE: u32 = 0;
}
