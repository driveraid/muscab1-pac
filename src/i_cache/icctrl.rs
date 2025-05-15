#[doc = "Register `ICCTRL` reader"]
pub type R = crate::R<IcctrlSpec>;
#[doc = "Register `ICCTRL` writer"]
pub type W = crate::W<IcctrlSpec>;
#[doc = "Enable Cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheen {
    #[doc = "1: Caching is enabled"]
    Enabled = 1,
    #[doc = "0: All accesses bypass the cache"]
    Disabled = 0,
}
impl From<Cacheen> for bool {
    #[inline(always)]
    fn from(variant: Cacheen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEEN` reader - Enable Cache"]
pub type CacheenR = crate::BitReader<Cacheen>;
impl CacheenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cacheen {
        match self.bits {
            true => Cacheen::Enabled,
            false => Cacheen::Disabled,
        }
    }
    #[doc = "Caching is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cacheen::Enabled
    }
    #[doc = "All accesses bypass the cache"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cacheen::Disabled
    }
}
#[doc = "Field `CACHEEN` writer - Enable Cache"]
pub type CacheenW<'a, REG> = crate::BitWriter<'a, REG, Cacheen>;
impl<'a, REG> CacheenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Caching is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheen::Enabled)
    }
    #[doc = "All accesses bypass the cache"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheen::Disabled)
    }
}
#[doc = "Full Cache Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finv {
    #[doc = "1: Triggers the instruction cache to start invalidating all cache lines"]
    Invalidate = 1,
}
impl From<Finv> for bool {
    #[inline(always)]
    fn from(variant: Finv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINV` writer - Full Cache Invalidate"]
pub type FinvW<'a, REG> = crate::BitWriter<'a, REG, Finv>;
impl<'a, REG> FinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Triggers the instruction cache to start invalidating all cache lines"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut crate::W<REG> {
        self.variant(Finv::Invalidate)
    }
}
#[doc = "Enable Statistic function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Staten {
    #[doc = "1: Cache statistic counters are enabled"]
    Enabled = 1,
    #[doc = "0: Cache statistic counters are disabled"]
    Disabled = 0,
}
impl From<Staten> for bool {
    #[inline(always)]
    fn from(variant: Staten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATEN` reader - Enable Statistic function"]
pub type StatenR = crate::BitReader<Staten>;
impl StatenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Staten {
        match self.bits {
            true => Staten::Enabled,
            false => Staten::Disabled,
        }
    }
    #[doc = "Cache statistic counters are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Staten::Enabled
    }
    #[doc = "Cache statistic counters are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Staten::Disabled
    }
}
#[doc = "Field `STATEN` writer - Enable Statistic function"]
pub type StatenW<'a, REG> = crate::BitWriter<'a, REG, Staten>;
impl<'a, REG> StatenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache statistic counters are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Staten::Enabled)
    }
    #[doc = "Cache statistic counters are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Staten::Disabled)
    }
}
#[doc = "Clear Statistic values\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Statc {
    #[doc = "1: Triggers the instruction cache to start clear all cache statistic counters"]
    Clear = 1,
}
impl From<Statc> for bool {
    #[inline(always)]
    fn from(variant: Statc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATC` writer - Clear Statistic values"]
pub type StatcW<'a, REG> = crate::BitWriter<'a, REG, Statc>;
impl<'a, REG> StatcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Triggers the instruction cache to start clear all cache statistic counters"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Statc::Clear)
    }
}
#[doc = "Enable Handler Allocation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halloc {
    #[doc = "0: All incoming handler code fetches are not allocated a cache line if a miss occurs"]
    Low = 0,
    #[doc = "1: Handler code access is treated like any other code access arriving at its interface"]
    High = 1,
}
impl From<Halloc> for bool {
    #[inline(always)]
    fn from(variant: Halloc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALLOC` reader - Enable Handler Allocation"]
pub type HallocR = crate::BitReader<Halloc>;
impl HallocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halloc {
        match self.bits {
            false => Halloc::Low,
            true => Halloc::High,
        }
    }
    #[doc = "All incoming handler code fetches are not allocated a cache line if a miss occurs"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Halloc::Low
    }
    #[doc = "Handler code access is treated like any other code access arriving at its interface"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Halloc::High
    }
}
#[doc = "Field `HALLOC` writer - Enable Handler Allocation"]
pub type HallocW<'a, REG> = crate::BitWriter<'a, REG, Halloc>;
impl<'a, REG> HallocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All incoming handler code fetches are not allocated a cache line if a miss occurs"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Halloc::Low)
    }
    #[doc = "Handler code access is treated like any other code access arriving at its interface"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Halloc::High)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Cache"]
    #[inline(always)]
    pub fn cacheen(&self) -> CacheenR {
        CacheenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Statistic function"]
    #[inline(always)]
    pub fn staten(&self) -> StatenR {
        StatenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Handler Allocation"]
    #[inline(always)]
    pub fn halloc(&self) -> HallocR {
        HallocR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Cache"]
    #[inline(always)]
    pub fn cacheen(&mut self) -> CacheenW<IcctrlSpec> {
        CacheenW::new(self, 0)
    }
    #[doc = "Bit 2 - Full Cache Invalidate"]
    #[inline(always)]
    pub fn finv(&mut self) -> FinvW<IcctrlSpec> {
        FinvW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Statistic function"]
    #[inline(always)]
    pub fn staten(&mut self) -> StatenW<IcctrlSpec> {
        StatenW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Statistic values"]
    #[inline(always)]
    pub fn statc(&mut self) -> StatcW<IcctrlSpec> {
        StatcW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Handler Allocation"]
    #[inline(always)]
    pub fn halloc(&mut self) -> HallocW<IcctrlSpec> {
        HallocW::new(self, 5)
    }
}
#[doc = "Instruction Cache Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcctrlSpec;
impl crate::RegisterSpec for IcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icctrl::R`](R) reader structure"]
impl crate::Readable for IcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`icctrl::W`](W) writer structure"]
impl crate::Writable for IcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCTRL to value 0"]
impl crate::Resettable for IcctrlSpec {}
