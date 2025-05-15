#[doc = "Register `PWMHR` reader"]
pub type R = crate::R<PwmhrSpec>;
#[doc = "Register `PWMHR` writer"]
pub type W = crate::W<PwmhrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PWM High Iime Register. This register contains the number of system clock cycles for during which the pwm_output should be kept high in a PWM cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmhrSpec;
impl crate::RegisterSpec for PwmhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmhr::R`](R) reader structure"]
impl crate::Readable for PwmhrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmhr::W`](W) writer structure"]
impl crate::Writable for PwmhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMHR to value 0"]
impl crate::Resettable for PwmhrSpec {
    const RESET_VALUE: u32 = 0;
}
