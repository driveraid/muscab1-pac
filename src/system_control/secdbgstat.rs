#[doc = "Register `SECDBGSTAT` reader"]
pub type R = crate::R<SecdbgstatSpec>;
#[doc = "Debug enable value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgenIStatus {
    #[doc = "1: debug enable"]
    Enable = 1,
    #[doc = "0: debug disable"]
    Disable = 0,
}
impl From<DbgenIStatus> for bool {
    #[inline(always)]
    fn from(variant: DbgenIStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN_I_STATUS` reader - Debug enable value"]
pub type DbgenIStatusR = crate::BitReader<DbgenIStatus>;
impl DbgenIStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgenIStatus {
        match self.bits {
            true => DbgenIStatus::Enable,
            false => DbgenIStatus::Disable,
        }
    }
    #[doc = "debug enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DbgenIStatus::Enable
    }
    #[doc = "debug disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DbgenIStatus::Disable
    }
}
#[doc = "Debug enable selector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgenSelStatus {
    #[doc = "1: debug enable selector"]
    Enable = 1,
    #[doc = "0: debug disable selector"]
    Disable = 0,
}
impl From<DbgenSelStatus> for bool {
    #[inline(always)]
    fn from(variant: DbgenSelStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN_SEL_STATUS` reader - Debug enable selector value"]
pub type DbgenSelStatusR = crate::BitReader<DbgenSelStatus>;
impl DbgenSelStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DbgenSelStatus {
        match self.bits {
            true => DbgenSelStatus::Enable,
            false => DbgenSelStatus::Disable,
        }
    }
    #[doc = "debug enable selector"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DbgenSelStatus::Enable
    }
    #[doc = "debug disable selector"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DbgenSelStatus::Disable
    }
}
#[doc = "Non-invasive debug enable value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NidenIStatus {
    #[doc = "1: non-invasive debug enable"]
    Enable = 1,
    #[doc = "0: non-invasive debug disable"]
    Disable = 0,
}
impl From<NidenIStatus> for bool {
    #[inline(always)]
    fn from(variant: NidenIStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN_I_STATUS` reader - Non-invasive debug enable value"]
pub type NidenIStatusR = crate::BitReader<NidenIStatus>;
impl NidenIStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NidenIStatus {
        match self.bits {
            true => NidenIStatus::Enable,
            false => NidenIStatus::Disable,
        }
    }
    #[doc = "non-invasive debug enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NidenIStatus::Enable
    }
    #[doc = "non-invasive debug disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NidenIStatus::Disable
    }
}
#[doc = "Non-invasive debug enable selector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NidenSelStatus {
    #[doc = "1: non-invasive debug enable selector"]
    Enable = 1,
    #[doc = "0: non-invasive debug disable selector"]
    Disable = 0,
}
impl From<NidenSelStatus> for bool {
    #[inline(always)]
    fn from(variant: NidenSelStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN_SEL_STATUS` reader - Non-invasive debug enable selector value"]
pub type NidenSelStatusR = crate::BitReader<NidenSelStatus>;
impl NidenSelStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NidenSelStatus {
        match self.bits {
            true => NidenSelStatus::Enable,
            false => NidenSelStatus::Disable,
        }
    }
    #[doc = "non-invasive debug enable selector"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NidenSelStatus::Enable
    }
    #[doc = "non-invasive debug disable selector"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NidenSelStatus::Disable
    }
}
#[doc = "Secure privilege invasive debug enable value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpidenIStatus {
    #[doc = "1: Secure privilege invasive debug enable"]
    Enable = 1,
    #[doc = "0: Secure privilege invasive debug disable"]
    Disable = 0,
}
impl From<SpidenIStatus> for bool {
    #[inline(always)]
    fn from(variant: SpidenIStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN_I_STATUS` reader - Secure privilege invasive debug enable value"]
pub type SpidenIStatusR = crate::BitReader<SpidenIStatus>;
impl SpidenIStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpidenIStatus {
        match self.bits {
            true => SpidenIStatus::Enable,
            false => SpidenIStatus::Disable,
        }
    }
    #[doc = "Secure privilege invasive debug enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SpidenIStatus::Enable
    }
    #[doc = "Secure privilege invasive debug disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SpidenIStatus::Disable
    }
}
#[doc = "Secure privilege invasive debug enable selector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpidenSelStatus {
    #[doc = "1: Secure privilege invasive debug enable selector"]
    Enable = 1,
    #[doc = "0: Secure privilege invasive debug disable selector"]
    Disable = 0,
}
impl From<SpidenSelStatus> for bool {
    #[inline(always)]
    fn from(variant: SpidenSelStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN_SEL_STATUS` reader - Secure privilege invasive debug enable selector value"]
pub type SpidenSelStatusR = crate::BitReader<SpidenSelStatus>;
impl SpidenSelStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpidenSelStatus {
        match self.bits {
            true => SpidenSelStatus::Enable,
            false => SpidenSelStatus::Disable,
        }
    }
    #[doc = "Secure privilege invasive debug enable selector"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SpidenSelStatus::Enable
    }
    #[doc = "Secure privilege invasive debug disable selector"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SpidenSelStatus::Disable
    }
}
#[doc = "Secure privilege non-invasive debug enable value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpnidenStatus {
    #[doc = "1: Secure privilege non-invasive debug enable"]
    Enable = 1,
    #[doc = "0: Secure privilege non-invasive debug disable"]
    Disable = 0,
}
impl From<SpnidenStatus> for bool {
    #[inline(always)]
    fn from(variant: SpnidenStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN_STATUS` reader - Secure privilege non-invasive debug enable value"]
pub type SpnidenStatusR = crate::BitReader<SpnidenStatus>;
impl SpnidenStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpnidenStatus {
        match self.bits {
            true => SpnidenStatus::Enable,
            false => SpnidenStatus::Disable,
        }
    }
    #[doc = "Secure privilege non-invasive debug enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SpnidenStatus::Enable
    }
    #[doc = "Secure privilege non-invasive debug disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SpnidenStatus::Disable
    }
}
#[doc = "Secure privilege non-invasive debug enable selector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpnidenSelStatus {
    #[doc = "1: Secure privilege non-invasive debug enable selector"]
    Enable = 1,
    #[doc = "0: Secure privilege non-invasive debug disable selector"]
    Disable = 0,
}
impl From<SpnidenSelStatus> for bool {
    #[inline(always)]
    fn from(variant: SpnidenSelStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN_SEL_STATUS` reader - Secure privilege non-invasive debug enable selector value"]
pub type SpnidenSelStatusR = crate::BitReader<SpnidenSelStatus>;
impl SpnidenSelStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpnidenSelStatus {
        match self.bits {
            true => SpnidenSelStatus::Enable,
            false => SpnidenSelStatus::Disable,
        }
    }
    #[doc = "Secure privilege non-invasive debug enable selector"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SpnidenSelStatus::Enable
    }
    #[doc = "Secure privilege non-invasive debug disable selector"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SpnidenSelStatus::Disable
    }
}
impl R {
    #[doc = "Bit 0 - Debug enable value"]
    #[inline(always)]
    pub fn dbgen_i_status(&self) -> DbgenIStatusR {
        DbgenIStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug enable selector value"]
    #[inline(always)]
    pub fn dbgen_sel_status(&self) -> DbgenSelStatusR {
        DbgenSelStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-invasive debug enable value"]
    #[inline(always)]
    pub fn niden_i_status(&self) -> NidenIStatusR {
        NidenIStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-invasive debug enable selector value"]
    #[inline(always)]
    pub fn niden_sel_status(&self) -> NidenSelStatusR {
        NidenSelStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Secure privilege invasive debug enable value"]
    #[inline(always)]
    pub fn spiden_i_status(&self) -> SpidenIStatusR {
        SpidenIStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secure privilege invasive debug enable selector value"]
    #[inline(always)]
    pub fn spiden_sel_status(&self) -> SpidenSelStatusR {
        SpidenSelStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure privilege non-invasive debug enable value"]
    #[inline(always)]
    pub fn spniden_status(&self) -> SpnidenStatusR {
        SpnidenStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Secure privilege non-invasive debug enable selector value"]
    #[inline(always)]
    pub fn spniden_sel_status(&self) -> SpnidenSelStatusR {
        SpnidenSelStatusR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Secure Debug Configuration Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secdbgstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecdbgstatSpec;
impl crate::RegisterSpec for SecdbgstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secdbgstat::R`](R) reader structure"]
impl crate::Readable for SecdbgstatSpec {}
#[doc = "`reset()` method sets SECDBGSTAT to value 0"]
impl crate::Resettable for SecdbgstatSpec {
    const RESET_VALUE: u32 = 0;
}
