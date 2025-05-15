#[doc = "Register `RNR` reader"]
pub type R = crate::R<RnrSpec>;
#[doc = "Register `RNR` writer"]
pub type W = crate::W<RnrSpec>;
#[doc = "Currently selected SAU region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Region {
    #[doc = "0: Select SAU Region 0"]
    SauRegion0 = 0,
    #[doc = "1: Select SAU Region 1"]
    SauRegion1 = 1,
    #[doc = "2: Select SAU Region 2"]
    SauRegion2 = 2,
    #[doc = "3: Select SAU Region 3"]
    SauRegion3 = 3,
}
impl From<Region> for u8 {
    #[inline(always)]
    fn from(variant: Region) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Region {
    type Ux = u8;
}
impl crate::IsEnum for Region {}
#[doc = "Field `REGION` reader - Currently selected SAU region"]
pub type RegionR = crate::FieldReader<Region>;
impl RegionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Region> {
        match self.bits {
            0 => Some(Region::SauRegion0),
            1 => Some(Region::SauRegion1),
            2 => Some(Region::SauRegion2),
            3 => Some(Region::SauRegion3),
            _ => None,
        }
    }
    #[doc = "Select SAU Region 0"]
    #[inline(always)]
    pub fn is_sau_region_0(&self) -> bool {
        *self == Region::SauRegion0
    }
    #[doc = "Select SAU Region 1"]
    #[inline(always)]
    pub fn is_sau_region_1(&self) -> bool {
        *self == Region::SauRegion1
    }
    #[doc = "Select SAU Region 2"]
    #[inline(always)]
    pub fn is_sau_region_2(&self) -> bool {
        *self == Region::SauRegion2
    }
    #[doc = "Select SAU Region 3"]
    #[inline(always)]
    pub fn is_sau_region_3(&self) -> bool {
        *self == Region::SauRegion3
    }
}
#[doc = "Field `REGION` writer - Currently selected SAU region"]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 8, Region>;
impl<'a, REG> RegionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select SAU Region 0"]
    #[inline(always)]
    pub fn sau_region_0(self) -> &'a mut crate::W<REG> {
        self.variant(Region::SauRegion0)
    }
    #[doc = "Select SAU Region 1"]
    #[inline(always)]
    pub fn sau_region_1(self) -> &'a mut crate::W<REG> {
        self.variant(Region::SauRegion1)
    }
    #[doc = "Select SAU Region 2"]
    #[inline(always)]
    pub fn sau_region_2(self) -> &'a mut crate::W<REG> {
        self.variant(Region::SauRegion2)
    }
    #[doc = "Select SAU Region 3"]
    #[inline(always)]
    pub fn sau_region_3(self) -> &'a mut crate::W<REG> {
        self.variant(Region::SauRegion3)
    }
}
impl R {
    #[doc = "Bits 0:7 - Currently selected SAU region"]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Currently selected SAU region"]
    #[inline(always)]
    pub fn region(&mut self) -> RegionW<RnrSpec> {
        RegionW::new(self, 0)
    }
}
#[doc = "Region Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RnrSpec;
impl crate::RegisterSpec for RnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnr::R`](R) reader structure"]
impl crate::Readable for RnrSpec {}
#[doc = "`write(|w| ..)` method takes [`rnr::W`](W) writer structure"]
impl crate::Writable for RnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNR to value 0"]
impl crate::Resettable for RnrSpec {
    const RESET_VALUE: u32 = 0;
}
