#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Timer is disabled"]
    Disable = 0,
    #[doc = "1: Timer is enabled"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Timer is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Timer is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Timer is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "External Input as Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extin {
    #[doc = "0: External Input as Enable is disabled"]
    Disable = 0,
    #[doc = "1: External Input as Enable is enabled"]
    Enable = 1,
}
impl From<Extin> for bool {
    #[inline(always)]
    fn from(variant: Extin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTIN` reader - External Input as Enable"]
pub type ExtinR = crate::BitReader<Extin>;
impl ExtinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extin {
        match self.bits {
            false => Extin::Disable,
            true => Extin::Enable,
        }
    }
    #[doc = "External Input as Enable is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Extin::Disable
    }
    #[doc = "External Input as Enable is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Extin::Enable
    }
}
#[doc = "Field `EXTIN` writer - External Input as Enable"]
pub type ExtinW<'a, REG> = crate::BitWriter<'a, REG, Extin>;
impl<'a, REG> ExtinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Input as Enable is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Extin::Disable)
    }
    #[doc = "External Input as Enable is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Extin::Enable)
    }
}
#[doc = "External Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extclk {
    #[doc = "0: External Clock is disabled"]
    Disable = 0,
    #[doc = "1: External Clock is enabled"]
    Enable = 1,
}
impl From<Extclk> for bool {
    #[inline(always)]
    fn from(variant: Extclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCLK` reader - External Clock Enable"]
pub type ExtclkR = crate::BitReader<Extclk>;
impl ExtclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extclk {
        match self.bits {
            false => Extclk::Disable,
            true => Extclk::Enable,
        }
    }
    #[doc = "External Clock is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Extclk::Disable
    }
    #[doc = "External Clock is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Extclk::Enable
    }
}
#[doc = "Field `EXTCLK` writer - External Clock Enable"]
pub type ExtclkW<'a, REG> = crate::BitWriter<'a, REG, Extclk>;
impl<'a, REG> ExtclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External Clock is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Extclk::Disable)
    }
    #[doc = "External Clock is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Extclk::Enable)
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: Interrupt is disabled"]
    Disable = 0,
    #[doc = "1: Interrupt is enabled"]
    Enable = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Interrupt Enable"]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::Disable,
            true => Inten::Enable,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inten::Disable
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inten::Enable
    }
}
#[doc = "Field `INTEN` writer - Interrupt Enable"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Disable)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Input as Enable"]
    #[inline(always)]
    pub fn extin(&self) -> ExtinR {
        ExtinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Clock Enable"]
    #[inline(always)]
    pub fn extclk(&self) -> ExtclkR {
        ExtclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - External Input as Enable"]
    #[inline(always)]
    pub fn extin(&mut self) -> ExtinW<CtrlSpec> {
        ExtinW::new(self, 1)
    }
    #[doc = "Bit 2 - External Clock Enable"]
    #[inline(always)]
    pub fn extclk(&mut self) -> ExtclkW<CtrlSpec> {
        ExtclkW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 3)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
