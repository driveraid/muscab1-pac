#[doc = "Register `SECDBGSET` writer"]
pub type W = crate::W<SecdbgsetSpec>;
#[doc = "Field `DBGEN_I_SET` writer - High active debug enable set control"]
pub type DbgenISetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Debug enable selector set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgenSelSet {
    #[doc = "1: debug enable selector set control"]
    Enable = 1,
    #[doc = "0: debug disable selector set control"]
    Disable = 0,
}
impl From<DbgenSelSet> for bool {
    #[inline(always)]
    fn from(variant: DbgenSelSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN_SEL_SET` writer - Debug enable selector set control"]
pub type DbgenSelSetW<'a, REG> = crate::BitWriter<'a, REG, DbgenSelSet>;
impl<'a, REG> DbgenSelSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "debug enable selector set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DbgenSelSet::Enable)
    }
    #[doc = "debug disable selector set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DbgenSelSet::Disable)
    }
}
#[doc = "Non-invasive debug enable set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NidenISet {
    #[doc = "1: non-invasive debug enable set control"]
    Enable = 1,
    #[doc = "0: non-invasive debug disable set control"]
    Disable = 0,
}
impl From<NidenISet> for bool {
    #[inline(always)]
    fn from(variant: NidenISet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN_I_SET` writer - Non-invasive debug enable set control"]
pub type NidenISetW<'a, REG> = crate::BitWriter<'a, REG, NidenISet>;
impl<'a, REG> NidenISetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-invasive debug enable set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenISet::Enable)
    }
    #[doc = "non-invasive debug disable set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenISet::Disable)
    }
}
#[doc = "Non-invasive debug enable selector set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NidenSelSet {
    #[doc = "1: non-invasive debug enable selector set control"]
    Enable = 1,
    #[doc = "0: non-invasive debug disable selector set control"]
    Disable = 0,
}
impl From<NidenSelSet> for bool {
    #[inline(always)]
    fn from(variant: NidenSelSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN_SEL_SET` writer - Non-invasive debug enable selector set control"]
pub type NidenSelSetW<'a, REG> = crate::BitWriter<'a, REG, NidenSelSet>;
impl<'a, REG> NidenSelSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-invasive debug enable selector set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenSelSet::Enable)
    }
    #[doc = "non-invasive debug disable selector set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenSelSet::Disable)
    }
}
#[doc = "Secure privilege invasive debug enable set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpidenISet {
    #[doc = "1: Secure privilege invasive debug enable set control"]
    Enable = 1,
    #[doc = "0: Secure privilege invasive debug disable set control"]
    Disable = 0,
}
impl From<SpidenISet> for bool {
    #[inline(always)]
    fn from(variant: SpidenISet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN_I_SET` writer - Secure privilege invasive debug enable set control"]
pub type SpidenISetW<'a, REG> = crate::BitWriter<'a, REG, SpidenISet>;
impl<'a, REG> SpidenISetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege invasive debug enable set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenISet::Enable)
    }
    #[doc = "Secure privilege invasive debug disable set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenISet::Disable)
    }
}
#[doc = "Secure privilege invasive debug enable selector set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpidenSelSet {
    #[doc = "1: Secure privilege invasive debug enable selector set control"]
    Enable = 1,
    #[doc = "0: Secure privilege invasive debug disable selector set control"]
    Disable = 0,
}
impl From<SpidenSelSet> for bool {
    #[inline(always)]
    fn from(variant: SpidenSelSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN_SEL_SET` writer - Secure privilege invasive debug enable selector set control"]
pub type SpidenSelSetW<'a, REG> = crate::BitWriter<'a, REG, SpidenSelSet>;
impl<'a, REG> SpidenSelSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege invasive debug enable selector set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenSelSet::Enable)
    }
    #[doc = "Secure privilege invasive debug disable selector set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenSelSet::Disable)
    }
}
#[doc = "Secure privilege non-invasive debug enable set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpnidenISet {
    #[doc = "1: Secure privilege non-invasive debug enable set control"]
    Enable = 1,
    #[doc = "0: Secure privilege non-invasive debug disable set control"]
    Disable = 0,
}
impl From<SpnidenISet> for bool {
    #[inline(always)]
    fn from(variant: SpnidenISet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN_I_SET` writer - Secure privilege non-invasive debug enable set control"]
pub type SpnidenISetW<'a, REG> = crate::BitWriter<'a, REG, SpnidenISet>;
impl<'a, REG> SpnidenISetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege non-invasive debug enable set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenISet::Enable)
    }
    #[doc = "Secure privilege non-invasive debug disable set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenISet::Disable)
    }
}
#[doc = "Secure privilege non-invasive debug enable selector set control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpnidenSelSet {
    #[doc = "1: Secure privilege non-invasive debug enable selector set control"]
    Enable = 1,
    #[doc = "0: Secure privilege non-invasive debug disable selector set control"]
    Disable = 0,
}
impl From<SpnidenSelSet> for bool {
    #[inline(always)]
    fn from(variant: SpnidenSelSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN_SEL_SET` writer - Secure privilege non-invasive debug enable selector set control"]
pub type SpnidenSelSetW<'a, REG> = crate::BitWriter<'a, REG, SpnidenSelSet>;
impl<'a, REG> SpnidenSelSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege non-invasive debug enable selector set control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenSelSet::Enable)
    }
    #[doc = "Secure privilege non-invasive debug disable selector set control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenSelSet::Disable)
    }
}
impl W {
    #[doc = "Bit 0 - High active debug enable set control"]
    #[inline(always)]
    pub fn dbgen_i_set(&mut self) -> DbgenISetW<SecdbgsetSpec> {
        DbgenISetW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug enable selector set control"]
    #[inline(always)]
    pub fn dbgen_sel_set(&mut self) -> DbgenSelSetW<SecdbgsetSpec> {
        DbgenSelSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Non-invasive debug enable set control"]
    #[inline(always)]
    pub fn niden_i_set(&mut self) -> NidenISetW<SecdbgsetSpec> {
        NidenISetW::new(self, 2)
    }
    #[doc = "Bit 3 - Non-invasive debug enable selector set control"]
    #[inline(always)]
    pub fn niden_sel_set(&mut self) -> NidenSelSetW<SecdbgsetSpec> {
        NidenSelSetW::new(self, 3)
    }
    #[doc = "Bit 4 - Secure privilege invasive debug enable set control"]
    #[inline(always)]
    pub fn spiden_i_set(&mut self) -> SpidenISetW<SecdbgsetSpec> {
        SpidenISetW::new(self, 4)
    }
    #[doc = "Bit 5 - Secure privilege invasive debug enable selector set control"]
    #[inline(always)]
    pub fn spiden_sel_set(&mut self) -> SpidenSelSetW<SecdbgsetSpec> {
        SpidenSelSetW::new(self, 5)
    }
    #[doc = "Bit 6 - Secure privilege non-invasive debug enable set control"]
    #[inline(always)]
    pub fn spniden_i_set(&mut self) -> SpnidenISetW<SecdbgsetSpec> {
        SpnidenISetW::new(self, 6)
    }
    #[doc = "Bit 7 - Secure privilege non-invasive debug enable selector set control"]
    #[inline(always)]
    pub fn spniden_sel_set(&mut self) -> SpnidenSelSetW<SecdbgsetSpec> {
        SpnidenSelSetW::new(self, 7)
    }
}
#[doc = "Secure Debug Configuration Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secdbgset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecdbgsetSpec;
impl crate::RegisterSpec for SecdbgsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`secdbgset::W`](W) writer structure"]
impl crate::Writable for SecdbgsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECDBGSET to value 0"]
impl crate::Resettable for SecdbgsetSpec {}
