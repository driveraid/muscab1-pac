#[doc = "Register `WICCTRL` reader"]
pub type R = crate::R<WicctrlSpec>;
#[doc = "Register `WICCTRL` writer"]
pub type W = crate::W<WicctrlSpec>;
#[doc = "CPU 0 WIC Enable Request Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0wicenStatus {
    #[doc = "1: CPU 0 WIC request enabled"]
    Enable = 1,
    #[doc = "0: CPU 0 WIC request disabled"]
    Disabled = 0,
}
impl From<Cpu0wicenStatus> for bool {
    #[inline(always)]
    fn from(variant: Cpu0wicenStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0WICEN_STATUS` reader - CPU 0 WIC Enable Request Status"]
pub type Cpu0wicenStatusR = crate::BitReader<Cpu0wicenStatus>;
impl Cpu0wicenStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0wicenStatus {
        match self.bits {
            true => Cpu0wicenStatus::Enable,
            false => Cpu0wicenStatus::Disabled,
        }
    }
    #[doc = "CPU 0 WIC request enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0wicenStatus::Enable
    }
    #[doc = "CPU 0 WIC request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu0wicenStatus::Disabled
    }
}
#[doc = "CPU 1 WIC Enable Request Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1wicenStatus {
    #[doc = "1: CPU 1 WIC request enabled"]
    Enable = 1,
    #[doc = "0: CPU 1 WIC request disabled"]
    Disabled = 0,
}
impl From<Cpu1wicenStatus> for bool {
    #[inline(always)]
    fn from(variant: Cpu1wicenStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1WICEN_STATUS` reader - CPU 1 WIC Enable Request Status"]
pub type Cpu1wicenStatusR = crate::BitReader<Cpu1wicenStatus>;
impl Cpu1wicenStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1wicenStatus {
        match self.bits {
            true => Cpu1wicenStatus::Enable,
            false => Cpu1wicenStatus::Disabled,
        }
    }
    #[doc = "CPU 1 WIC request enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1wicenStatus::Enable
    }
    #[doc = "CPU 1 WIC request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu1wicenStatus::Disabled
    }
}
#[doc = "Field `CPU0WICEN_SET` writer - High Active CPU 0 WIC Enable Request Set"]
pub type Cpu0wicenSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1WICEN_SET` writer - High Active CPU 1 WIC Enable Request Set"]
pub type Cpu1wicenSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0WICEN_CLR` writer - High Active CPU 0 WIC Enable Request Clear"]
pub type Cpu0wicenClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1WICEN_CLR` writer - High Active CPU 1 WIC Enable Request Clear"]
pub type Cpu1wicenClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CPU 0 WIC Enable Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0wicrdy {
    #[doc = "1: CPU 0 WIC Enabled"]
    Enabled = 1,
    #[doc = "0: CPU 0 WIC Disabled"]
    Disabled = 0,
}
impl From<Cpu0wicrdy> for bool {
    #[inline(always)]
    fn from(variant: Cpu0wicrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0WICRDY` reader - CPU 0 WIC Enable Acknowledge"]
pub type Cpu0wicrdyR = crate::BitReader<Cpu0wicrdy>;
impl Cpu0wicrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0wicrdy {
        match self.bits {
            true => Cpu0wicrdy::Enabled,
            false => Cpu0wicrdy::Disabled,
        }
    }
    #[doc = "CPU 0 WIC Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cpu0wicrdy::Enabled
    }
    #[doc = "CPU 0 WIC Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu0wicrdy::Disabled
    }
}
#[doc = "CPU 1 WIC Enable Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1wicrdy {
    #[doc = "1: CPU 1 WIC Enabled"]
    Enabled = 1,
    #[doc = "0: CPU 1 WIC Disabled"]
    Disabled = 0,
}
impl From<Cpu1wicrdy> for bool {
    #[inline(always)]
    fn from(variant: Cpu1wicrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1WICRDY` reader - CPU 1 WIC Enable Acknowledge"]
pub type Cpu1wicrdyR = crate::BitReader<Cpu1wicrdy>;
impl Cpu1wicrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1wicrdy {
        match self.bits {
            true => Cpu1wicrdy::Enabled,
            false => Cpu1wicrdy::Disabled,
        }
    }
    #[doc = "CPU 1 WIC Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cpu1wicrdy::Enabled
    }
    #[doc = "CPU 1 WIC Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpu1wicrdy::Disabled
    }
}
impl R {
    #[doc = "Bit 0 - CPU 0 WIC Enable Request Status"]
    #[inline(always)]
    pub fn cpu0wicen_status(&self) -> Cpu0wicenStatusR {
        Cpu0wicenStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU 1 WIC Enable Request Status"]
    #[inline(always)]
    pub fn cpu1wicen_status(&self) -> Cpu1wicenStatusR {
        Cpu1wicenStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU 0 WIC Enable Acknowledge"]
    #[inline(always)]
    pub fn cpu0wicrdy(&self) -> Cpu0wicrdyR {
        Cpu0wicrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU 1 WIC Enable Acknowledge"]
    #[inline(always)]
    pub fn cpu1wicrdy(&self) -> Cpu1wicrdyR {
        Cpu1wicrdyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - High Active CPU 0 WIC Enable Request Set"]
    #[inline(always)]
    pub fn cpu0wicen_set(&mut self) -> Cpu0wicenSetW<WicctrlSpec> {
        Cpu0wicenSetW::new(self, 4)
    }
    #[doc = "Bit 5 - High Active CPU 1 WIC Enable Request Set"]
    #[inline(always)]
    pub fn cpu1wicen_set(&mut self) -> Cpu1wicenSetW<WicctrlSpec> {
        Cpu1wicenSetW::new(self, 5)
    }
    #[doc = "Bit 8 - High Active CPU 0 WIC Enable Request Clear"]
    #[inline(always)]
    pub fn cpu0wicen_clr(&mut self) -> Cpu0wicenClrW<WicctrlSpec> {
        Cpu0wicenClrW::new(self, 8)
    }
    #[doc = "Bit 9 - High Active CPU 1 WIC Enable Request Clear"]
    #[inline(always)]
    pub fn cpu1wicen_clr(&mut self) -> Cpu1wicenClrW<WicctrlSpec> {
        Cpu1wicenClrW::new(self, 9)
    }
}
#[doc = "WIC request and acknowledge handshake\n\nYou can [`read`](crate::Reg::read) this register and get [`wicctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WicctrlSpec;
impl crate::RegisterSpec for WicctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wicctrl::R`](R) reader structure"]
impl crate::Readable for WicctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wicctrl::W`](W) writer structure"]
impl crate::Writable for WicctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WICCTRL to value 0"]
impl crate::Resettable for WicctrlSpec {
    const RESET_VALUE: u32 = 0;
}
