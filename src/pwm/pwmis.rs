#[doc = "Register `PWMIS` reader"]
pub type R = crate::R<PwmisSpec>;
#[doc = "Reading from this address returns the current state of the PWM Interrupt output, and then sets the bit low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "1: Interrupt is active"]
    Active = 1,
    #[doc = "0: Interrupt is not active"]
    Notactive = 0,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Status` reader - Reading from this address returns the current state of the PWM Interrupt output, and then sets the bit low"]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            true => Status::Active,
            false => Status::Notactive,
        }
    }
    #[doc = "Interrupt is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Status::Active
    }
    #[doc = "Interrupt is not active"]
    #[inline(always)]
    pub fn is_notactive(&self) -> bool {
        *self == Status::Notactive
    }
}
impl R {
    #[doc = "Bit 0 - Reading from this address returns the current state of the PWM Interrupt output, and then sets the bit low"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "PWM Read Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmisSpec;
impl crate::RegisterSpec for PwmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmis::R`](R) reader structure"]
impl crate::Readable for PwmisSpec {}
#[doc = "`reset()` method sets PWMIS to value 0"]
impl crate::Resettable for PwmisSpec {}
