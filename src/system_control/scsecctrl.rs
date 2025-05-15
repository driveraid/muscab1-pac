#[doc = "Register `SCSECCTRL` reader"]
pub type R = crate::R<ScsecctrlSpec>;
#[doc = "Register `SCSECCTRL` writer"]
pub type W = crate::W<ScsecctrlSpec>;
#[doc = "Control to disable certification path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Certdisable {
    #[doc = "1: control to disable certification path"]
    Disable = 1,
    #[doc = "0: control to enable certification path"]
    Enable = 0,
}
impl From<Certdisable> for bool {
    #[inline(always)]
    fn from(variant: Certdisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERTDISABLE` reader - Control to disable certification path"]
pub type CertdisableR = crate::BitReader<Certdisable>;
impl CertdisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Certdisable {
        match self.bits {
            true => Certdisable::Disable,
            false => Certdisable::Enable,
        }
    }
    #[doc = "control to disable certification path"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Certdisable::Disable
    }
    #[doc = "control to enable certification path"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Certdisable::Enable
    }
}
#[doc = "Field `CERTDISABLE` writer - Control to disable certification path"]
pub type CertdisableW<'a, REG> = crate::BitWriter<'a, REG, Certdisable>;
impl<'a, REG> CertdisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "control to disable certification path"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Certdisable::Disable)
    }
    #[doc = "control to enable certification path"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Certdisable::Enable)
    }
}
#[doc = "Control to enable read access on the certification path as long as CERTDISABLE is also LOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Certreaden {
    #[doc = "1: control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    Enable = 1,
    #[doc = "0: control to disable read access on the certification path as long as CERTDISABLE is also LOW"]
    Disable = 0,
}
impl From<Certreaden> for bool {
    #[inline(always)]
    fn from(variant: Certreaden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERTREADEN` reader - Control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
pub type CertreadenR = crate::BitReader<Certreaden>;
impl CertreadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Certreaden {
        match self.bits {
            true => Certreaden::Enable,
            false => Certreaden::Disable,
        }
    }
    #[doc = "control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Certreaden::Enable
    }
    #[doc = "control to disable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Certreaden::Disable
    }
}
#[doc = "Field `CERTREADEN` writer - Control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
pub type CertreadenW<'a, REG> = crate::BitWriter<'a, REG, Certreaden>;
impl<'a, REG> CertreadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Certreaden::Enable)
    }
    #[doc = "control to disable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Certreaden::Disable)
    }
}
#[doc = "Control to disable writes to security-related control registers in this register block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scseccfglock {
    #[doc = "1: control to disable writes to security-related control registers in this register block"]
    Disable = 1,
    #[doc = "0: control to enable writes to security-related control registers in this register block"]
    Enable = 0,
}
impl From<Scseccfglock> for bool {
    #[inline(always)]
    fn from(variant: Scseccfglock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCSECCFGLOCK` reader - Control to disable writes to security-related control registers in this register block"]
pub type ScseccfglockR = crate::BitReader<Scseccfglock>;
impl ScseccfglockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scseccfglock {
        match self.bits {
            true => Scseccfglock::Disable,
            false => Scseccfglock::Enable,
        }
    }
    #[doc = "control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Scseccfglock::Disable
    }
    #[doc = "control to enable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Scseccfglock::Enable
    }
}
#[doc = "Field `SCSECCFGLOCK` writer - Control to disable writes to security-related control registers in this register block"]
pub type ScseccfglockW<'a, REG> = crate::BitWriter<'a, REG, Scseccfglock>;
impl<'a, REG> ScseccfglockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Scseccfglock::Disable)
    }
    #[doc = "control to enable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Scseccfglock::Enable)
    }
}
#[doc = "Indicates that the Certification write path has been disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Certdisabled {
    #[doc = "1: Certification write path has been disabled"]
    Disabled = 1,
    #[doc = "0: Certification write path has been enabled"]
    Enabled = 0,
}
impl From<Certdisabled> for bool {
    #[inline(always)]
    fn from(variant: Certdisabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERTDISABLED` reader - Indicates that the Certification write path has been disabled"]
pub type CertdisabledR = crate::BitReader<Certdisabled>;
impl CertdisabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Certdisabled {
        match self.bits {
            true => Certdisabled::Disabled,
            false => Certdisabled::Enabled,
        }
    }
    #[doc = "Certification write path has been disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Certdisabled::Disabled
    }
    #[doc = "Certification write path has been enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Certdisabled::Enabled
    }
}
#[doc = "Field `CERTDISABLED` writer - Indicates that the Certification write path has been disabled"]
pub type CertdisabledW<'a, REG> = crate::BitWriter<'a, REG, Certdisabled>;
impl<'a, REG> CertdisabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Certification write path has been disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Certdisabled::Disabled)
    }
    #[doc = "Certification write path has been enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Certdisabled::Enabled)
    }
}
#[doc = "Indicates whether the certification read access is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Certreadenabled {
    #[doc = "1: certification read access is enabled"]
    Enabled = 1,
    #[doc = "0: certification read access is disabled"]
    Disabled = 0,
}
impl From<Certreadenabled> for bool {
    #[inline(always)]
    fn from(variant: Certreadenabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERTREADENABLED` reader - Indicates whether the certification read access is enabled"]
pub type CertreadenabledR = crate::BitReader<Certreadenabled>;
impl CertreadenabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Certreadenabled {
        match self.bits {
            true => Certreadenabled::Enabled,
            false => Certreadenabled::Disabled,
        }
    }
    #[doc = "certification read access is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Certreadenabled::Enabled
    }
    #[doc = "certification read access is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Certreadenabled::Disabled
    }
}
#[doc = "Field `CERTREADENABLED` writer - Indicates whether the certification read access is enabled"]
pub type CertreadenabledW<'a, REG> = crate::BitWriter<'a, REG, Certreadenabled>;
impl<'a, REG> CertreadenabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "certification read access is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Certreadenabled::Enabled)
    }
    #[doc = "certification read access is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Certreadenabled::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Control to disable certification path"]
    #[inline(always)]
    pub fn certdisable(&self) -> CertdisableR {
        CertdisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn certreaden(&self) -> CertreadenR {
        CertreadenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn scseccfglock(&self) -> ScseccfglockR {
        ScseccfglockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates that the Certification write path has been disabled"]
    #[inline(always)]
    pub fn certdisabled(&self) -> CertdisabledR {
        CertdisabledR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates whether the certification read access is enabled"]
    #[inline(always)]
    pub fn certreadenabled(&self) -> CertreadenabledR {
        CertreadenabledR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control to disable certification path"]
    #[inline(always)]
    pub fn certdisable(&mut self) -> CertdisableW<ScsecctrlSpec> {
        CertdisableW::new(self, 0)
    }
    #[doc = "Bit 1 - Control to enable read access on the certification path as long as CERTDISABLE is also LOW"]
    #[inline(always)]
    pub fn certreaden(&mut self) -> CertreadenW<ScsecctrlSpec> {
        CertreadenW::new(self, 1)
    }
    #[doc = "Bit 2 - Control to disable writes to security-related control registers in this register block"]
    #[inline(always)]
    pub fn scseccfglock(&mut self) -> ScseccfglockW<ScsecctrlSpec> {
        ScseccfglockW::new(self, 2)
    }
    #[doc = "Bit 16 - Indicates that the Certification write path has been disabled"]
    #[inline(always)]
    pub fn certdisabled(&mut self) -> CertdisabledW<ScsecctrlSpec> {
        CertdisabledW::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates whether the certification read access is enabled"]
    #[inline(always)]
    pub fn certreadenabled(&mut self) -> CertreadenabledW<ScsecctrlSpec> {
        CertreadenabledW::new(self, 17)
    }
}
#[doc = "System Security Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scsecctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsecctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsecctrlSpec;
impl crate::RegisterSpec for ScsecctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsecctrl::R`](R) reader structure"]
impl crate::Readable for ScsecctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`scsecctrl::W`](W) writer structure"]
impl crate::Writable for ScsecctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCSECCTRL to value 0"]
impl crate::Resettable for ScsecctrlSpec {}
