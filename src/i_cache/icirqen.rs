#[doc = "Register `ICIRQEN` reader"]
pub type R = crate::R<IcirqenSpec>;
#[doc = "Register `ICIRQEN` writer"]
pub type W = crate::W<IcirqenSpec>;
#[doc = "Invalidate Complete IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IcEn {
    #[doc = "1: Enable the Invalidate Complete IRQ"]
    Enabled = 1,
    #[doc = "0: Disable the Invalidate Complete IRQ"]
    Disabled = 0,
}
impl From<IcEn> for bool {
    #[inline(always)]
    fn from(variant: IcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_EN` reader - Invalidate Complete IRQ Enable"]
pub type IcEnR = crate::BitReader<IcEn>;
impl IcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IcEn {
        match self.bits {
            true => IcEn::Enabled,
            false => IcEn::Disabled,
        }
    }
    #[doc = "Enable the Invalidate Complete IRQ"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IcEn::Enabled
    }
    #[doc = "Disable the Invalidate Complete IRQ"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IcEn::Disabled
    }
}
#[doc = "Field `IC_EN` writer - Invalidate Complete IRQ Enable"]
pub type IcEnW<'a, REG> = crate::BitWriter<'a, REG, IcEn>;
impl<'a, REG> IcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Invalidate Complete IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IcEn::Enabled)
    }
    #[doc = "Disable the Invalidate Complete IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IcEn::Disabled)
    }
}
#[doc = "Cache Disable Complete IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdcEn {
    #[doc = "1: Enable the Cache Disable Complete IRQ"]
    Enabled = 1,
    #[doc = "0: Disable the Cache Disable Complete IRQ"]
    Disabled = 0,
}
impl From<CdcEn> for bool {
    #[inline(always)]
    fn from(variant: CdcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDC_EN` reader - Cache Disable Complete IRQ Enable"]
pub type CdcEnR = crate::BitReader<CdcEn>;
impl CdcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CdcEn {
        match self.bits {
            true => CdcEn::Enabled,
            false => CdcEn::Disabled,
        }
    }
    #[doc = "Enable the Cache Disable Complete IRQ"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CdcEn::Enabled
    }
    #[doc = "Disable the Cache Disable Complete IRQ"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CdcEn::Disabled
    }
}
#[doc = "Field `CDC_EN` writer - Cache Disable Complete IRQ Enable"]
pub type CdcEnW<'a, REG> = crate::BitWriter<'a, REG, CdcEn>;
impl<'a, REG> CdcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Cache Disable Complete IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CdcEn::Enabled)
    }
    #[doc = "Disable the Cache Disable Complete IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CdcEn::Disabled)
    }
}
#[doc = "Cache Enable Complete IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CecEn {
    #[doc = "1: Enable the Cache Enable Complete IRQ"]
    Enabled = 1,
    #[doc = "0: Disable the Cache Enable Complete IRQ"]
    Disabled = 0,
}
impl From<CecEn> for bool {
    #[inline(always)]
    fn from(variant: CecEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEC_EN` reader - Cache Enable Complete IRQ Enable"]
pub type CecEnR = crate::BitReader<CecEn>;
impl CecEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CecEn {
        match self.bits {
            true => CecEn::Enabled,
            false => CecEn::Disabled,
        }
    }
    #[doc = "Enable the Cache Enable Complete IRQ"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CecEn::Enabled
    }
    #[doc = "Disable the Cache Enable Complete IRQ"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CecEn::Disabled
    }
}
#[doc = "Field `CEC_EN` writer - Cache Enable Complete IRQ Enable"]
pub type CecEnW<'a, REG> = crate::BitWriter<'a, REG, CecEn>;
impl<'a, REG> CecEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Cache Enable Complete IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CecEn::Enabled)
    }
    #[doc = "Disable the Cache Enable Complete IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CecEn::Disabled)
    }
}
#[doc = "Cache Fill Error IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfeEn {
    #[doc = "1: Enable the Cache Fill Error IRQ"]
    Enabled = 1,
    #[doc = "0: Disable the Cache Fill Error IRQ"]
    Disabled = 0,
}
impl From<CfeEn> for bool {
    #[inline(always)]
    fn from(variant: CfeEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFE_EN` reader - Cache Fill Error IRQ Enable"]
pub type CfeEnR = crate::BitReader<CfeEn>;
impl CfeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CfeEn {
        match self.bits {
            true => CfeEn::Enabled,
            false => CfeEn::Disabled,
        }
    }
    #[doc = "Enable the Cache Fill Error IRQ"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CfeEn::Enabled
    }
    #[doc = "Disable the Cache Fill Error IRQ"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CfeEn::Disabled
    }
}
#[doc = "Field `CFE_EN` writer - Cache Fill Error IRQ Enable"]
pub type CfeEnW<'a, REG> = crate::BitWriter<'a, REG, CfeEn>;
impl<'a, REG> CfeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Cache Fill Error IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CfeEn::Enabled)
    }
    #[doc = "Disable the Cache Fill Error IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CfeEn::Disabled)
    }
}
#[doc = "Security violation IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvEn {
    #[doc = "1: Enable the Security violation IRQ"]
    Enabled = 1,
    #[doc = "0: Disable the Security violation IRQ"]
    Disabled = 0,
}
impl From<SvEn> for bool {
    #[inline(always)]
    fn from(variant: SvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SV_EN` reader - Security violation IRQ Enable"]
pub type SvEnR = crate::BitReader<SvEn>;
impl SvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SvEn {
        match self.bits {
            true => SvEn::Enabled,
            false => SvEn::Disabled,
        }
    }
    #[doc = "Enable the Security violation IRQ"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SvEn::Enabled
    }
    #[doc = "Disable the Security violation IRQ"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SvEn::Disabled
    }
}
#[doc = "Field `SV_EN` writer - Security violation IRQ Enable"]
pub type SvEnW<'a, REG> = crate::BitWriter<'a, REG, SvEn>;
impl<'a, REG> SvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Security violation IRQ"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SvEn::Enabled)
    }
    #[doc = "Disable the Security violation IRQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SvEn::Disabled)
    }
}
#[doc = "Statistics Saturated Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsEn {
    #[doc = "1: Enable the Statistics Saturated"]
    Enabled = 1,
    #[doc = "0: Disable the Statistics Saturated"]
    Disabled = 0,
}
impl From<SsEn> for bool {
    #[inline(always)]
    fn from(variant: SsEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS_EN` reader - Statistics Saturated Enable"]
pub type SsEnR = crate::BitReader<SsEn>;
impl SsEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsEn {
        match self.bits {
            true => SsEn::Enabled,
            false => SsEn::Disabled,
        }
    }
    #[doc = "Enable the Statistics Saturated"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SsEn::Enabled
    }
    #[doc = "Disable the Statistics Saturated"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SsEn::Disabled
    }
}
#[doc = "Field `SS_EN` writer - Statistics Saturated Enable"]
pub type SsEnW<'a, REG> = crate::BitWriter<'a, REG, SsEn>;
impl<'a, REG> SsEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the Statistics Saturated"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SsEn::Enabled)
    }
    #[doc = "Disable the Statistics Saturated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SsEn::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Invalidate Complete IRQ Enable"]
    #[inline(always)]
    pub fn ic_en(&self) -> IcEnR {
        IcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cdc_en(&self) -> CdcEnR {
        CdcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cec_en(&self) -> CecEnR {
        CecEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Enable"]
    #[inline(always)]
    pub fn cfe_en(&self) -> CfeEnR {
        CfeEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security violation IRQ Enable"]
    #[inline(always)]
    pub fn sv_en(&self) -> SvEnR {
        SvEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Statistics Saturated Enable"]
    #[inline(always)]
    pub fn ss_en(&self) -> SsEnR {
        SsEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate Complete IRQ Enable"]
    #[inline(always)]
    pub fn ic_en(&mut self) -> IcEnW<IcirqenSpec> {
        IcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cdc_en(&mut self) -> CdcEnW<IcirqenSpec> {
        CdcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Enable"]
    #[inline(always)]
    pub fn cec_en(&mut self) -> CecEnW<IcirqenSpec> {
        CecEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Enable"]
    #[inline(always)]
    pub fn cfe_en(&mut self) -> CfeEnW<IcirqenSpec> {
        CfeEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Security violation IRQ Enable"]
    #[inline(always)]
    pub fn sv_en(&mut self) -> SvEnW<IcirqenSpec> {
        SvEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Statistics Saturated Enable"]
    #[inline(always)]
    pub fn ss_en(&mut self) -> SsEnW<IcirqenSpec> {
        SsEnW::new(self, 5)
    }
}
#[doc = "Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`icirqen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icirqen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcirqenSpec;
impl crate::RegisterSpec for IcirqenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icirqen::R`](R) reader structure"]
impl crate::Readable for IcirqenSpec {}
#[doc = "`write(|w| ..)` method takes [`icirqen::W`](W) writer structure"]
impl crate::Writable for IcirqenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICIRQEN to value 0"]
impl crate::Resettable for IcirqenSpec {}
