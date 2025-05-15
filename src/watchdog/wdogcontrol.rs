#[doc = "Register `WDOGCONTROL` reader"]
pub type R = crate::R<WdogcontrolSpec>;
#[doc = "Register `WDOGCONTROL` writer"]
pub type W = crate::W<WdogcontrolSpec>;
#[doc = "Enable the interrupt event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: Disable Watchdog interrupt"]
    Disable = 0,
    #[doc = "1: Enable Watchdog interrupt."]
    Enable = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Enable the interrupt event"]
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
    #[doc = "Disable Watchdog interrupt"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inten::Disable
    }
    #[doc = "Enable Watchdog interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inten::Enable
    }
}
#[doc = "Field `INTEN` writer - Enable the interrupt event"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Watchdog interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Disable)
    }
    #[doc = "Enable Watchdog interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Enable)
    }
}
#[doc = "Enable watchdog reset output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resen {
    #[doc = "0: Disable Watchdog reset"]
    Disable = 0,
    #[doc = "1: Enable Watchdog reset"]
    Enable = 1,
}
impl From<Resen> for bool {
    #[inline(always)]
    fn from(variant: Resen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESEN` reader - Enable watchdog reset output"]
pub type ResenR = crate::BitReader<Resen>;
impl ResenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resen {
        match self.bits {
            false => Resen::Disable,
            true => Resen::Enable,
        }
    }
    #[doc = "Disable Watchdog reset"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Resen::Disable
    }
    #[doc = "Enable Watchdog reset"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Resen::Enable
    }
}
#[doc = "Field `RESEN` writer - Enable watchdog reset output"]
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG, Resen>;
impl<'a, REG> ResenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Watchdog reset"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Resen::Disable)
    }
    #[doc = "Enable Watchdog reset"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Resen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the interrupt event"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt event"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<WdogcontrolSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&mut self) -> ResenW<WdogcontrolSpec> {
        ResenW::new(self, 1)
    }
}
#[doc = "Watchdog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogcontrolSpec;
impl crate::RegisterSpec for WdogcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcontrol::R`](R) reader structure"]
impl crate::Readable for WdogcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogcontrol::W`](W) writer structure"]
impl crate::Writable for WdogcontrolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGCONTROL to value 0"]
impl crate::Resettable for WdogcontrolSpec {}
