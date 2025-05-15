#[doc = "Register `SECDBGCLR` writer"]
pub type W = crate::W<SecdbgclrSpec>;
#[doc = "Debug enable clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgenIClr {
    #[doc = "1: debug enable clear control"]
    Enable = 1,
    #[doc = "0: debug disable clear control"]
    Disable = 0,
}
impl From<DbgenIClr> for bool {
    #[inline(always)]
    fn from(variant: DbgenIClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN_I_CLR` writer - Debug enable clear control"]
pub type DbgenIClrW<'a, REG> = crate::BitWriter<'a, REG, DbgenIClr>;
impl<'a, REG> DbgenIClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DbgenIClr::Enable)
    }
    #[doc = "debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DbgenIClr::Disable)
    }
}
#[doc = "Debug enable selector clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbgenSelClr {
    #[doc = "1: debug enable selector clear control"]
    Enable = 1,
    #[doc = "0: debug disable selector clear control"]
    Disable = 0,
}
impl From<DbgenSelClr> for bool {
    #[inline(always)]
    fn from(variant: DbgenSelClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN_SEL_CLR` writer - Debug enable selector clear control"]
pub type DbgenSelClrW<'a, REG> = crate::BitWriter<'a, REG, DbgenSelClr>;
impl<'a, REG> DbgenSelClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DbgenSelClr::Enable)
    }
    #[doc = "debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DbgenSelClr::Disable)
    }
}
#[doc = "Non-invasive debug enable clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NidenIClr {
    #[doc = "1: non-invasive debug enable clear control"]
    Enable = 1,
    #[doc = "0: non-invasive debug disable clear control"]
    Disable = 0,
}
impl From<NidenIClr> for bool {
    #[inline(always)]
    fn from(variant: NidenIClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN_I_CLR` writer - Non-invasive debug enable clear control"]
pub type NidenIClrW<'a, REG> = crate::BitWriter<'a, REG, NidenIClr>;
impl<'a, REG> NidenIClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenIClr::Enable)
    }
    #[doc = "non-invasive debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenIClr::Disable)
    }
}
#[doc = "Non-invasive debug enable selector clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NidenSelClr {
    #[doc = "1: non-invasive debug enable selector clear control"]
    Enable = 1,
    #[doc = "0: non-invasive debug disable selector clear control"]
    Disable = 0,
}
impl From<NidenSelClr> for bool {
    #[inline(always)]
    fn from(variant: NidenSelClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN_SEL_CLR` writer - Non-invasive debug enable selector clear control"]
pub type NidenSelClrW<'a, REG> = crate::BitWriter<'a, REG, NidenSelClr>;
impl<'a, REG> NidenSelClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenSelClr::Enable)
    }
    #[doc = "non-invasive debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(NidenSelClr::Disable)
    }
}
#[doc = "Secure privilege invasive debug enable clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpidenIClr {
    #[doc = "1: Secure privilege invasive debug enable clear control"]
    Enable = 1,
    #[doc = "0: Secure privilege invasive debug disable clear control"]
    Disable = 0,
}
impl From<SpidenIClr> for bool {
    #[inline(always)]
    fn from(variant: SpidenIClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN_I_CLR` writer - Secure privilege invasive debug enable clear control"]
pub type SpidenIClrW<'a, REG> = crate::BitWriter<'a, REG, SpidenIClr>;
impl<'a, REG> SpidenIClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege invasive debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenIClr::Enable)
    }
    #[doc = "Secure privilege invasive debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenIClr::Disable)
    }
}
#[doc = "Secure privilege invasive debug enable selector clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpidenSelClr {
    #[doc = "1: Secure privilege invasive debug enable selector clear control"]
    Enable = 1,
    #[doc = "0: Secure privilege invasive debug disable selector clear control"]
    Disable = 0,
}
impl From<SpidenSelClr> for bool {
    #[inline(always)]
    fn from(variant: SpidenSelClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN_SEL_CLR` writer - Secure privilege invasive debug enable selector clear control"]
pub type SpidenSelClrW<'a, REG> = crate::BitWriter<'a, REG, SpidenSelClr>;
impl<'a, REG> SpidenSelClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenSelClr::Enable)
    }
    #[doc = "Secure privilege invasive debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpidenSelClr::Disable)
    }
}
#[doc = "Secure privilege non-invasive debug enable clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpnidenIClr {
    #[doc = "1: Secure privilege non-invasive debug enable clear control"]
    Enable = 1,
    #[doc = "0: Secure privilege non-invasive debug disable clear control"]
    Disable = 0,
}
impl From<SpnidenIClr> for bool {
    #[inline(always)]
    fn from(variant: SpnidenIClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN_I_CLR` writer - Secure privilege non-invasive debug enable clear control"]
pub type SpnidenIClrW<'a, REG> = crate::BitWriter<'a, REG, SpnidenIClr>;
impl<'a, REG> SpnidenIClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenIClr::Enable)
    }
    #[doc = "Secure privilege non-invasive debug disable clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenIClr::Disable)
    }
}
#[doc = "Secure privilege non-invasive debug enable selector clear control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpnidenSelClr {
    #[doc = "1: Secure privilege non-invasive debug enable selector clear control"]
    Enable = 1,
    #[doc = "0: Secure privilege non-invasive debug disable selector clear control"]
    Disable = 0,
}
impl From<SpnidenSelClr> for bool {
    #[inline(always)]
    fn from(variant: SpnidenSelClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN_SEL_CLR` writer - Secure privilege non-invasive debug enable selector clear control"]
pub type SpnidenSelClrW<'a, REG> = crate::BitWriter<'a, REG, SpnidenSelClr>;
impl<'a, REG> SpnidenSelClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure privilege non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenSelClr::Enable)
    }
    #[doc = "Secure privilege non-invasive debug disable selector clear control"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpnidenSelClr::Disable)
    }
}
impl W {
    #[doc = "Bit 0 - Debug enable clear control"]
    #[inline(always)]
    pub fn dbgen_i_clr(&mut self) -> DbgenIClrW<SecdbgclrSpec> {
        DbgenIClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug enable selector clear control"]
    #[inline(always)]
    pub fn dbgen_sel_clr(&mut self) -> DbgenSelClrW<SecdbgclrSpec> {
        DbgenSelClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn niden_i_clr(&mut self) -> NidenIClrW<SecdbgclrSpec> {
        NidenIClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn niden_sel_clr(&mut self) -> NidenSelClrW<SecdbgclrSpec> {
        NidenSelClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Secure privilege invasive debug enable clear control"]
    #[inline(always)]
    pub fn spiden_i_clr(&mut self) -> SpidenIClrW<SecdbgclrSpec> {
        SpidenIClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Secure privilege invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn spiden_sel_clr(&mut self) -> SpidenSelClrW<SecdbgclrSpec> {
        SpidenSelClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Secure privilege non-invasive debug enable clear control"]
    #[inline(always)]
    pub fn spniden_i_clr(&mut self) -> SpnidenIClrW<SecdbgclrSpec> {
        SpnidenIClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Secure privilege non-invasive debug enable selector clear control"]
    #[inline(always)]
    pub fn spniden_sel_clr(&mut self) -> SpnidenSelClrW<SecdbgclrSpec> {
        SpnidenSelClrW::new(self, 7)
    }
}
#[doc = "Secure Debug Configuration Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secdbgclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecdbgclrSpec;
impl crate::RegisterSpec for SecdbgclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`secdbgclr::W`](W) writer structure"]
impl crate::Writable for SecdbgclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECDBGCLR to value 0"]
impl crate::Resettable for SecdbgclrSpec {
    const RESET_VALUE: u32 = 0;
}
