#[doc = "Register `EWCTRL` reader"]
pub type R = crate::R<EwctrlSpec>;
#[doc = "Register `EWCTRL` writer"]
pub type W = crate::W<EwctrlSpec>;
#[doc = "External Wakeup Controller 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewc0enStatus {
    #[doc = "1: External Wakeup Controller 0 Enabled"]
    Enable = 1,
    #[doc = "0: External Wakeup Controller 0 Disabled"]
    Disabled = 0,
}
impl From<Ewc0enStatus> for bool {
    #[inline(always)]
    fn from(variant: Ewc0enStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWC0EN_STATUS` reader - External Wakeup Controller 0 Enable"]
pub type Ewc0enStatusR = crate::BitReader<Ewc0enStatus>;
impl Ewc0enStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewc0enStatus {
        match self.bits {
            true => Ewc0enStatus::Enable,
            false => Ewc0enStatus::Disabled,
        }
    }
    #[doc = "External Wakeup Controller 0 Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ewc0enStatus::Enable
    }
    #[doc = "External Wakeup Controller 0 Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ewc0enStatus::Disabled
    }
}
#[doc = "External Wakeup Controller 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ewc1enStatus {
    #[doc = "1: External Wakeup Controller 1 Enabled"]
    Enable = 1,
    #[doc = "0: External Wakeup Controller 1 Disabled"]
    Disabled = 0,
}
impl From<Ewc1enStatus> for bool {
    #[inline(always)]
    fn from(variant: Ewc1enStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWC1EN_STATUS` reader - External Wakeup Controller 1 Enable"]
pub type Ewc1enStatusR = crate::BitReader<Ewc1enStatus>;
impl Ewc1enStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ewc1enStatus {
        match self.bits {
            true => Ewc1enStatus::Enable,
            false => Ewc1enStatus::Disabled,
        }
    }
    #[doc = "External Wakeup Controller 1 Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ewc1enStatus::Enable
    }
    #[doc = "External Wakeup Controller 1 Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ewc1enStatus::Disabled
    }
}
#[doc = "Field `EWC0EN_SET` writer - EHigh Active External Wakeup Controller 0 Set"]
pub type Ewc0enSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWC1EN_SET` writer - High Active External Wakeup Controller 1 Set"]
pub type Ewc1enSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWC0EN_CLR` writer - High Active External Wakeup Controller 0 Clear"]
pub type Ewc0enClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWC1EN_CLR` writer - High Active External Wakeup Controller 1 Clear"]
pub type Ewc1enClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Wakeup Controller 0 Enable"]
    #[inline(always)]
    pub fn ewc0en_status(&self) -> Ewc0enStatusR {
        Ewc0enStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Wakeup Controller 1 Enable"]
    #[inline(always)]
    pub fn ewc1en_status(&self) -> Ewc1enStatusR {
        Ewc1enStatusR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - EHigh Active External Wakeup Controller 0 Set"]
    #[inline(always)]
    pub fn ewc0en_set(&mut self) -> Ewc0enSetW<EwctrlSpec> {
        Ewc0enSetW::new(self, 4)
    }
    #[doc = "Bit 5 - High Active External Wakeup Controller 1 Set"]
    #[inline(always)]
    pub fn ewc1en_set(&mut self) -> Ewc1enSetW<EwctrlSpec> {
        Ewc1enSetW::new(self, 5)
    }
    #[doc = "Bit 8 - High Active External Wakeup Controller 0 Clear"]
    #[inline(always)]
    pub fn ewc0en_clr(&mut self) -> Ewc0enClrW<EwctrlSpec> {
        Ewc0enClrW::new(self, 8)
    }
    #[doc = "Bit 9 - High Active External Wakeup Controller 1 Clear"]
    #[inline(always)]
    pub fn ewc1en_clr(&mut self) -> Ewc1enClrW<EwctrlSpec> {
        Ewc1enClrW::new(self, 9)
    }
}
#[doc = "External Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ewctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwctrlSpec;
impl crate::RegisterSpec for EwctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewctrl::R`](R) reader structure"]
impl crate::Readable for EwctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ewctrl::W`](W) writer structure"]
impl crate::Writable for EwctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EWCTRL to value 0"]
impl crate::Resettable for EwctrlSpec {
    const RESET_VALUE: u32 = 0;
}
