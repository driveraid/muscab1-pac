#[doc = "Register `ICIRQSTAT` reader"]
pub type R = crate::R<IcirqstatSpec>;
#[doc = "Invalidate Complete IRQ Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IcStatus {
    #[doc = "1: Indicates that a cache invalidation process has been completed"]
    Completed = 1,
}
impl From<IcStatus> for bool {
    #[inline(always)]
    fn from(variant: IcStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_STATUS` reader - Invalidate Complete IRQ Status"]
pub type IcStatusR = crate::BitReader<IcStatus>;
impl IcStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IcStatus> {
        match self.bits {
            true => Some(IcStatus::Completed),
            _ => None,
        }
    }
    #[doc = "Indicates that a cache invalidation process has been completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == IcStatus::Completed
    }
}
#[doc = "Cache Disable Complete IRQ Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdcStatus {
    #[doc = "1: Indicates that a request to disable the cache has been completed"]
    Completed = 1,
}
impl From<CdcStatus> for bool {
    #[inline(always)]
    fn from(variant: CdcStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDC_STATUS` reader - Cache Disable Complete IRQ Status"]
pub type CdcStatusR = crate::BitReader<CdcStatus>;
impl CdcStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CdcStatus> {
        match self.bits {
            true => Some(CdcStatus::Completed),
            _ => None,
        }
    }
    #[doc = "Indicates that a request to disable the cache has been completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CdcStatus::Completed
    }
}
#[doc = "Cache Enable Complete IRQ Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CecStatus {
    #[doc = "1: Indicates that a request to enable the cache has been completed"]
    Completed = 1,
}
impl From<CecStatus> for bool {
    #[inline(always)]
    fn from(variant: CecStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEC_STATUS` reader - Cache Enable Complete IRQ Status"]
pub type CecStatusR = crate::BitReader<CecStatus>;
impl CecStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CecStatus> {
        match self.bits {
            true => Some(CecStatus::Completed),
            _ => None,
        }
    }
    #[doc = "Indicates that a request to enable the cache has been completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CecStatus::Completed
    }
}
#[doc = "Cache Fill Error IRQ Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfeStatus {
    #[doc = "1: Indicates that a bus error occurred while filling a cache line"]
    ErrOccurred = 1,
}
impl From<CfeStatus> for bool {
    #[inline(always)]
    fn from(variant: CfeStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFE_STATUS` reader - Cache Fill Error IRQ Status"]
pub type CfeStatusR = crate::BitReader<CfeStatus>;
impl CfeStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CfeStatus> {
        match self.bits {
            true => Some(CfeStatus::ErrOccurred),
            _ => None,
        }
    }
    #[doc = "Indicates that a bus error occurred while filling a cache line"]
    #[inline(always)]
    pub fn is_err_occurred(&self) -> bool {
        *self == CfeStatus::ErrOccurred
    }
}
#[doc = "Field `SV_STATUS` reader - Security violation IRQ Status"]
pub type SvStatusR = crate::BitReader;
#[doc = "Statistics Saturated Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsStatus {
    #[doc = "1: Indicates that the internal statistic counters have saturated"]
    Saturated = 1,
}
impl From<SsStatus> for bool {
    #[inline(always)]
    fn from(variant: SsStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS_STATUS` reader - Statistics Saturated Status"]
pub type SsStatusR = crate::BitReader<SsStatus>;
impl SsStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SsStatus> {
        match self.bits {
            true => Some(SsStatus::Saturated),
            _ => None,
        }
    }
    #[doc = "Indicates that the internal statistic counters have saturated"]
    #[inline(always)]
    pub fn is_saturated(&self) -> bool {
        *self == SsStatus::Saturated
    }
}
impl R {
    #[doc = "Bit 0 - Invalidate Complete IRQ Status"]
    #[inline(always)]
    pub fn ic_status(&self) -> IcStatusR {
        IcStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Status"]
    #[inline(always)]
    pub fn cdc_status(&self) -> CdcStatusR {
        CdcStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Status"]
    #[inline(always)]
    pub fn cec_status(&self) -> CecStatusR {
        CecStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Status"]
    #[inline(always)]
    pub fn cfe_status(&self) -> CfeStatusR {
        CfeStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security violation IRQ Status"]
    #[inline(always)]
    pub fn sv_status(&self) -> SvStatusR {
        SvStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Statistics Saturated Status"]
    #[inline(always)]
    pub fn ss_status(&self) -> SsStatusR {
        SsStatusR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icirqstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcirqstatSpec;
impl crate::RegisterSpec for IcirqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icirqstat::R`](R) reader structure"]
impl crate::Readable for IcirqstatSpec {}
#[doc = "`reset()` method sets ICIRQSTAT to value 0"]
impl crate::Resettable for IcirqstatSpec {
    const RESET_VALUE: u32 = 0;
}
