#[doc = "Register `NMI_ENABLE` reader"]
pub type R = crate::R<NmiEnableSpec>;
#[doc = "Register `NMI_ENABLE` writer"]
pub type W = crate::W<NmiEnableSpec>;
#[doc = "CPU0 Internally Sourced NMI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0IntnmiEnable {
    #[doc = "1: CPU0 Internally Sourced NMI Enabled"]
    Enable = 1,
    #[doc = "0: CPU0 Internally Sourced NMI Disabled"]
    Disabled = 0,
}
impl From<Cpu0IntnmiEnable> for bool {
    #[inline(always)]
    fn from(variant: Cpu0IntnmiEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0_INTNMI_ENABLE` reader - CPU0 Internally Sourced NMI Enable"]
pub type Cpu0IntnmiEnableR = crate::BitReader<Cpu0IntnmiEnable>;
impl Cpu0IntnmiEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0IntnmiEnable {
        match self.bits {
            true => Cpu0IntnmiEnable::Enable,
            false => Cpu0IntnmiEnable::Disabled,
        }
    }
    #[doc = "CPU0 Internally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0IntnmiEnable::Enable
    }
    #[doc = "CPU0 Internally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu0IntnmiEnable::Disabled
    }
}
#[doc = "Field `CPU0_INTNMI_ENABLE` writer - CPU0 Internally Sourced NMI Enable"]
pub type Cpu0IntnmiEnableW<'a, REG> = crate::BitWriter<'a, REG, Cpu0IntnmiEnable>;
impl<'a, REG> Cpu0IntnmiEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU0 Internally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0IntnmiEnable::Enable)
    }
    #[doc = "CPU0 Internally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0IntnmiEnable::Disabled)
    }
}
#[doc = "CPU1 Internally Sourced NMI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1IntnmiEnable {
    #[doc = "1: CPU1 Internally Sourced NMI Enabled"]
    Enable = 1,
    #[doc = "0: CPU1 Internally Sourced NMI Disabled"]
    Disabled = 0,
}
impl From<Cpu1IntnmiEnable> for bool {
    #[inline(always)]
    fn from(variant: Cpu1IntnmiEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_INTNMI_ENABLE` reader - CPU1 Internally Sourced NMI Enable"]
pub type Cpu1IntnmiEnableR = crate::BitReader<Cpu1IntnmiEnable>;
impl Cpu1IntnmiEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1IntnmiEnable {
        match self.bits {
            true => Cpu1IntnmiEnable::Enable,
            false => Cpu1IntnmiEnable::Disabled,
        }
    }
    #[doc = "CPU1 Internally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1IntnmiEnable::Enable
    }
    #[doc = "CPU1 Internally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu1IntnmiEnable::Disabled
    }
}
#[doc = "Field `CPU1_INTNMI_ENABLE` writer - CPU1 Internally Sourced NMI Enable"]
pub type Cpu1IntnmiEnableW<'a, REG> = crate::BitWriter<'a, REG, Cpu1IntnmiEnable>;
impl<'a, REG> Cpu1IntnmiEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU1 Internally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1IntnmiEnable::Enable)
    }
    #[doc = "CPU1 Internally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1IntnmiEnable::Disabled)
    }
}
#[doc = "CPU0 Externally Sourced NMI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0ExpnmiEnable {
    #[doc = "1: CPU0 Externally Sourced NMI Enabled"]
    Enable = 1,
    #[doc = "0: CPU0 Externally Sourced NMI Disabled"]
    Disabled = 0,
}
impl From<Cpu0ExpnmiEnable> for bool {
    #[inline(always)]
    fn from(variant: Cpu0ExpnmiEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0_EXPNMI_ENABLE` reader - CPU0 Externally Sourced NMI Enable"]
pub type Cpu0ExpnmiEnableR = crate::BitReader<Cpu0ExpnmiEnable>;
impl Cpu0ExpnmiEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0ExpnmiEnable {
        match self.bits {
            true => Cpu0ExpnmiEnable::Enable,
            false => Cpu0ExpnmiEnable::Disabled,
        }
    }
    #[doc = "CPU0 Externally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0ExpnmiEnable::Enable
    }
    #[doc = "CPU0 Externally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu0ExpnmiEnable::Disabled
    }
}
#[doc = "Field `CPU0_EXPNMI_ENABLE` writer - CPU0 Externally Sourced NMI Enable"]
pub type Cpu0ExpnmiEnableW<'a, REG> = crate::BitWriter<'a, REG, Cpu0ExpnmiEnable>;
impl<'a, REG> Cpu0ExpnmiEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU0 Externally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0ExpnmiEnable::Enable)
    }
    #[doc = "CPU0 Externally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0ExpnmiEnable::Disabled)
    }
}
#[doc = "CPU1 Externally Sourced NMI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1ExpnmiEnable {
    #[doc = "1: CPU1 Externally Sourced NMI Enabled"]
    Enable = 1,
    #[doc = "0: CPU1 Externally Sourced NMI Disabled"]
    Disabled = 0,
}
impl From<Cpu1ExpnmiEnable> for bool {
    #[inline(always)]
    fn from(variant: Cpu1ExpnmiEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_EXPNMI_ENABLE` reader - CPU1 Externally Sourced NMI Enable"]
pub type Cpu1ExpnmiEnableR = crate::BitReader<Cpu1ExpnmiEnable>;
impl Cpu1ExpnmiEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1ExpnmiEnable {
        match self.bits {
            true => Cpu1ExpnmiEnable::Enable,
            false => Cpu1ExpnmiEnable::Disabled,
        }
    }
    #[doc = "CPU1 Externally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1ExpnmiEnable::Enable
    }
    #[doc = "CPU1 Externally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu1ExpnmiEnable::Disabled
    }
}
#[doc = "Field `CPU1_EXPNMI_ENABLE` writer - CPU1 Externally Sourced NMI Enable"]
pub type Cpu1ExpnmiEnableW<'a, REG> = crate::BitWriter<'a, REG, Cpu1ExpnmiEnable>;
impl<'a, REG> Cpu1ExpnmiEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU1 Externally Sourced NMI Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1ExpnmiEnable::Enable)
    }
    #[doc = "CPU1 Externally Sourced NMI Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1ExpnmiEnable::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - CPU0 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_intnmi_enable(&self) -> Cpu0IntnmiEnableR {
        Cpu0IntnmiEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU1 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_intnmi_enable(&self) -> Cpu1IntnmiEnableR {
        Cpu1IntnmiEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU0 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_expnmi_enable(&self) -> Cpu0ExpnmiEnableR {
        Cpu0ExpnmiEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU1 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_expnmi_enable(&self) -> Cpu1ExpnmiEnableR {
        Cpu1ExpnmiEnableR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU0 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_intnmi_enable(&mut self) -> Cpu0IntnmiEnableW<NmiEnableSpec> {
        Cpu0IntnmiEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU1 Internally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_intnmi_enable(&mut self) -> Cpu1IntnmiEnableW<NmiEnableSpec> {
        Cpu1IntnmiEnableW::new(self, 1)
    }
    #[doc = "Bit 16 - CPU0 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu0_expnmi_enable(&mut self) -> Cpu0ExpnmiEnableW<NmiEnableSpec> {
        Cpu0ExpnmiEnableW::new(self, 16)
    }
    #[doc = "Bit 17 - CPU1 Externally Sourced NMI Enable"]
    #[inline(always)]
    pub fn cpu1_expnmi_enable(&mut self) -> Cpu1ExpnmiEnableW<NmiEnableSpec> {
        Cpu1ExpnmiEnableW::new(self, 17)
    }
}
#[doc = "NMI Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiEnableSpec;
impl crate::RegisterSpec for NmiEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmi_enable::R`](R) reader structure"]
impl crate::Readable for NmiEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`nmi_enable::W`](W) writer structure"]
impl crate::Writable for NmiEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMI_ENABLE to value 0"]
impl crate::Resettable for NmiEnableSpec {}
