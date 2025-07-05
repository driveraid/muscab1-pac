#[doc = "Register `WDOGLOCK` reader"]
pub type R = crate::R<WdoglockSpec>;
#[doc = "Register `WDOGLOCK` writer"]
pub type W = crate::W<WdoglockSpec>;
#[doc = "Register write enable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Write access to all other registers is enabled. This is the default."]
    Enabled = 0,
    #[doc = "1: Write access to all other registers is disabled."]
    Disabled = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Status` reader - Register write enable status"]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Enabled,
            true => Status::Disabled,
        }
    }
    #[doc = "Write access to all other registers is enabled. This is the default."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Status::Enabled
    }
    #[doc = "Write access to all other registers is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Status::Disabled
    }
}
#[doc = "Field `Status` writer - Register write enable status"]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG, Status>;
impl<'a, REG> StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write access to all other registers is enabled. This is the default."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Enabled)
    }
    #[doc = "Write access to all other registers is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Disabled)
    }
}
#[doc = "Field `Access` reader - Enable register writes"]
pub type AccessR = crate::FieldReader<u32>;
#[doc = "Field `Access` writer - Enable register writes"]
pub type AccessW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - Register write enable status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Enable register writes"]
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Register write enable status"]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<WdoglockSpec> {
        StatusW::new(self, 0)
    }
    #[doc = "Bits 1:31 - Enable register writes"]
    #[inline(always)]
    pub fn access(&mut self) -> AccessW<WdoglockSpec> {
        AccessW::new(self, 1)
    }
}
#[doc = "Watchdog Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdoglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdoglockSpec;
impl crate::RegisterSpec for WdoglockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdoglock::R`](R) reader structure"]
impl crate::Readable for WdoglockSpec {}
#[doc = "`write(|w| ..)` method takes [`wdoglock::W`](W) writer structure"]
impl crate::Writable for WdoglockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDOGLOCK to value 0"]
impl crate::Resettable for WdoglockSpec {
    const RESET_VALUE: u32 = 0;
}
