#[doc = "Register `PWMPR` reader"]
pub type R = crate::R<PwmprSpec>;
#[doc = "Register `PWMPR` writer"]
pub type W = crate::W<PwmprSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PWM Period Register. Number of system clock cycles indicating the period of PWM cycle.The minimum and maximum values have special significance. 0x0: pwm_output continually high 0xFFFFFFFF: pwm_output continually low\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmprSpec;
impl crate::RegisterSpec for PwmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmpr::R`](R) reader structure"]
impl crate::Readable for PwmprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmpr::W`](W) writer structure"]
impl crate::Writable for PwmprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMPR to value 0"]
impl crate::Resettable for PwmprSpec {
    const RESET_VALUE: u32 = 0;
}
