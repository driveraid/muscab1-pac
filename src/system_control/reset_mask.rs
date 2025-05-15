#[doc = "Register `RESET_MASK` reader"]
pub type R = crate::R<ResetMaskSpec>;
#[doc = "Register `RESET_MASK` writer"]
pub type W = crate::W<ResetMaskSpec>;
#[doc = "Enable NON-SECURE WATCHDOG Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NswdEn {
    #[doc = "1: Enable NON-SECURE WATCHDOG Reset"]
    Enabled = 1,
    #[doc = "0: Disabled NON-SECURE WATCHDOG Reset"]
    Disabled = 0,
}
impl From<NswdEn> for bool {
    #[inline(always)]
    fn from(variant: NswdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSWD_EN` reader - Enable NON-SECURE WATCHDOG Reset"]
pub type NswdEnR = crate::BitReader<NswdEn>;
impl NswdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NswdEn {
        match self.bits {
            true => NswdEn::Enabled,
            false => NswdEn::Disabled,
        }
    }
    #[doc = "Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NswdEn::Enabled
    }
    #[doc = "Disabled NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NswdEn::Disabled
    }
}
#[doc = "Field `NSWD_EN` writer - Enable NON-SECURE WATCHDOG Reset"]
pub type NswdEnW<'a, REG> = crate::BitWriter<'a, REG, NswdEn>;
impl<'a, REG> NswdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NswdEn::Enabled)
    }
    #[doc = "Disabled NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NswdEn::Disabled)
    }
}
#[doc = "Enable Merging CPU 0 System Reset Request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrstreq0En {
    #[doc = "1: Enable Merging CPU 0 System Reset Request"]
    Enabled = 1,
    #[doc = "0: Disabled Merging CPU 0 System Reset Request"]
    Disabled = 0,
}
impl From<Sysrstreq0En> for bool {
    #[inline(always)]
    fn from(variant: Sysrstreq0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRSTREQ0_EN` reader - Enable Merging CPU 0 System Reset Request"]
pub type Sysrstreq0EnR = crate::BitReader<Sysrstreq0En>;
impl Sysrstreq0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrstreq0En {
        match self.bits {
            true => Sysrstreq0En::Enabled,
            false => Sysrstreq0En::Disabled,
        }
    }
    #[doc = "Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sysrstreq0En::Enabled
    }
    #[doc = "Disabled Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sysrstreq0En::Disabled
    }
}
#[doc = "Field `SYSRSTREQ0_EN` writer - Enable Merging CPU 0 System Reset Request"]
pub type Sysrstreq0EnW<'a, REG> = crate::BitWriter<'a, REG, Sysrstreq0En>;
impl<'a, REG> Sysrstreq0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstreq0En::Enabled)
    }
    #[doc = "Disabled Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstreq0En::Disabled)
    }
}
#[doc = "Enable Merging CPU 0 System Reset Request\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrstreq1En {
    #[doc = "1: Enable Merging CPU 1 System Reset Request"]
    Enabled = 1,
    #[doc = "0: Disabled Merging CPU 1 System Reset Request"]
    Disabled = 0,
}
impl From<Sysrstreq1En> for bool {
    #[inline(always)]
    fn from(variant: Sysrstreq1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRSTREQ1_EN` reader - Enable Merging CPU 0 System Reset Request"]
pub type Sysrstreq1EnR = crate::BitReader<Sysrstreq1En>;
impl Sysrstreq1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrstreq1En {
        match self.bits {
            true => Sysrstreq1En::Enabled,
            false => Sysrstreq1En::Disabled,
        }
    }
    #[doc = "Enable Merging CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sysrstreq1En::Enabled
    }
    #[doc = "Disabled Merging CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sysrstreq1En::Disabled
    }
}
#[doc = "Field `SYSRSTREQ1_EN` writer - Enable Merging CPU 0 System Reset Request"]
pub type Sysrstreq1EnW<'a, REG> = crate::BitWriter<'a, REG, Sysrstreq1En>;
impl<'a, REG> Sysrstreq1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Merging CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstreq1En::Enabled)
    }
    #[doc = "Disabled Merging CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrstreq1En::Disabled)
    }
}
impl R {
    #[doc = "Bit 1 - Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn nswd_en(&self) -> NswdEnR {
        NswdEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq0_en(&self) -> Sysrstreq0EnR {
        Sysrstreq0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq1_en(&self) -> Sysrstreq1EnR {
        Sysrstreq1EnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable NON-SECURE WATCHDOG Reset"]
    #[inline(always)]
    pub fn nswd_en(&mut self) -> NswdEnW<ResetMaskSpec> {
        NswdEnW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq0_en(&mut self) -> Sysrstreq0EnW<ResetMaskSpec> {
        Sysrstreq0EnW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Merging CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq1_en(&mut self) -> Sysrstreq1EnW<ResetMaskSpec> {
        Sysrstreq1EnW::new(self, 5)
    }
}
#[doc = "Reset Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetMaskSpec;
impl crate::RegisterSpec for ResetMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_mask::R`](R) reader structure"]
impl crate::Readable for ResetMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_mask::W`](W) writer structure"]
impl crate::Writable for ResetMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_MASK to value 0x30"]
impl crate::Resettable for ResetMaskSpec {
    const RESET_VALUE: u32 = 0x30;
}
