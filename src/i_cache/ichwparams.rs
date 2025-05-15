#[doc = "Register `ICHWPARAMS` reader"]
pub type R = crate::R<IchwparamsSpec>;
#[doc = "Field `CSIZE` reader - Cache size: Defines the size of the instruction cache"]
pub type CsizeR = crate::FieldReader;
#[doc = "Field `STATS` reader - Presence of Statistic Functionality"]
pub type StatsR = crate::BitReader;
#[doc = "Presence of DMA Engine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "1: The Instruction cache supports pre-fetch and locking"]
    Support = 1,
    #[doc = "0: The Instruction cache does not support pre-fetch and locking"]
    Unsupport = 0,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Presence of DMA Engine"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            true => Dma::Support,
            false => Dma::Unsupport,
        }
    }
    #[doc = "The Instruction cache supports pre-fetch and locking"]
    #[inline(always)]
    pub fn is_support(&self) -> bool {
        *self == Dma::Support
    }
    #[doc = "The Instruction cache does not support pre-fetch and locking"]
    #[inline(always)]
    pub fn is_unsupport(&self) -> bool {
        *self == Dma::Unsupport
    }
}
#[doc = "Indicates whether invalidate cache line on write match is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invmat {
    #[doc = "1: Indicates Invalidate Cache Line on Write Match is enabled"]
    Enabled = 1,
}
impl From<Invmat> for bool {
    #[inline(always)]
    fn from(variant: Invmat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVMAT` reader - Indicates whether invalidate cache line on write match is enabled"]
pub type InvmatR = crate::BitReader<Invmat>;
impl InvmatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Invmat> {
        match self.bits {
            true => Some(Invmat::Enabled),
            _ => None,
        }
    }
    #[doc = "Indicates Invalidate Cache Line on Write Match is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Invmat::Enabled
    }
}
#[doc = "Field `COFFSIZE` reader - Cacheable Block Size"]
pub type CoffsizeR = crate::FieldReader;
#[doc = "Field `COFFSET` reader - Cacheable Offset Address"]
pub type CoffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Cache size: Defines the size of the instruction cache"]
    #[inline(always)]
    pub fn csize(&self) -> CsizeR {
        CsizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Presence of Statistic Functionality"]
    #[inline(always)]
    pub fn stats(&self) -> StatsR {
        StatsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Presence of DMA Engine"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates whether invalidate cache line on write match is enabled"]
    #[inline(always)]
    pub fn invmat(&self) -> InvmatR {
        InvmatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Cacheable Block Size"]
    #[inline(always)]
    pub fn coffsize(&self) -> CoffsizeR {
        CoffsizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Cacheable Offset Address"]
    #[inline(always)]
    pub fn coffset(&self) -> CoffsetR {
        CoffsetR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Hardware Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ichwparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IchwparamsSpec;
impl crate::RegisterSpec for IchwparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ichwparams::R`](R) reader structure"]
impl crate::Readable for IchwparamsSpec {}
#[doc = "`reset()` method sets ICHWPARAMS to value 0"]
impl crate::Resettable for IchwparamsSpec {}
