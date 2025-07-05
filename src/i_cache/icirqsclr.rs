#[doc = "Register `ICIRQSCLR` writer"]
pub type W = crate::W<IcirqsclrSpec>;
#[doc = "Invalidate Complete IRQ Status Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IcClr {
    #[doc = "1: Clear the Invalidate Complete IRQ Status"]
    Clear = 1,
}
impl From<IcClr> for bool {
    #[inline(always)]
    fn from(variant: IcClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC_CLR` writer - Invalidate Complete IRQ Status Clear"]
pub type IcClrW<'a, REG> = crate::BitWriter<'a, REG, IcClr>;
impl<'a, REG> IcClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Invalidate Complete IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IcClr::Clear)
    }
}
#[doc = "Cache Disable Complete IRQ Status Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CdcClr {
    #[doc = "1: Clear Cache Disable Complete IRQ Status"]
    Clear = 1,
}
impl From<CdcClr> for bool {
    #[inline(always)]
    fn from(variant: CdcClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDC_CLR` writer - Cache Disable Complete IRQ Status Clear"]
pub type CdcClrW<'a, REG> = crate::BitWriter<'a, REG, CdcClr>;
impl<'a, REG> CdcClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear Cache Disable Complete IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CdcClr::Clear)
    }
}
#[doc = "Cache Enable Complete IRQ Status Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CecClr {
    #[doc = "1: Clear the Cache Enable Complete IRQ Status"]
    Clear = 1,
}
impl From<CecClr> for bool {
    #[inline(always)]
    fn from(variant: CecClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEC_CLR` writer - Cache Enable Complete IRQ Status Clear"]
pub type CecClrW<'a, REG> = crate::BitWriter<'a, REG, CecClr>;
impl<'a, REG> CecClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Cache Enable Complete IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CecClr::Clear)
    }
}
#[doc = "Cache Fill Error IRQ Status Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfeClr {
    #[doc = "1: Clear the Cache Fill Error IRQ Status"]
    Clear = 1,
}
impl From<CfeClr> for bool {
    #[inline(always)]
    fn from(variant: CfeClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFE_CLR` writer - Cache Fill Error IRQ Status Clear"]
pub type CfeClrW<'a, REG> = crate::BitWriter<'a, REG, CfeClr>;
impl<'a, REG> CfeClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Cache Fill Error IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CfeClr::Clear)
    }
}
#[doc = "Security violation IRQ Status Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvClr {
    #[doc = "1: Clear the Security violation IRQ Status"]
    Clear = 1,
}
impl From<SvClr> for bool {
    #[inline(always)]
    fn from(variant: SvClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SV_CLR` writer - Security violation IRQ Status Clear"]
pub type SvClrW<'a, REG> = crate::BitWriter<'a, REG, SvClr>;
impl<'a, REG> SvClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Security violation IRQ Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SvClr::Clear)
    }
}
#[doc = "Statistics Saturated Status Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsClr {
    #[doc = "1: Clear the Statistics Saturated Status"]
    Clear = 1,
}
impl From<SsClr> for bool {
    #[inline(always)]
    fn from(variant: SsClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS_CLR` writer - Statistics Saturated Status Clear"]
pub type SsClrW<'a, REG> = crate::BitWriter<'a, REG, SsClr>;
impl<'a, REG> SsClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the Statistics Saturated Status"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SsClr::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate Complete IRQ Status Clear"]
    #[inline(always)]
    pub fn ic_clr(&mut self) -> IcClrW<IcirqsclrSpec> {
        IcClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Cache Disable Complete IRQ Status Clear"]
    #[inline(always)]
    pub fn cdc_clr(&mut self) -> CdcClrW<IcirqsclrSpec> {
        CdcClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Cache Enable Complete IRQ Status Clear"]
    #[inline(always)]
    pub fn cec_clr(&mut self) -> CecClrW<IcirqsclrSpec> {
        CecClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Cache Fill Error IRQ Status Clear"]
    #[inline(always)]
    pub fn cfe_clr(&mut self) -> CfeClrW<IcirqsclrSpec> {
        CfeClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Security violation IRQ Status Clear"]
    #[inline(always)]
    pub fn sv_clr(&mut self) -> SvClrW<IcirqsclrSpec> {
        SvClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Statistics Saturated Status Clear"]
    #[inline(always)]
    pub fn ss_clr(&mut self) -> SsClrW<IcirqsclrSpec> {
        SsClrW::new(self, 5)
    }
}
#[doc = "Interrupt Status Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icirqsclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcirqsclrSpec;
impl crate::RegisterSpec for IcirqsclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icirqsclr::W`](W) writer structure"]
impl crate::Writable for IcirqsclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICIRQSCLR to value 0"]
impl crate::Resettable for IcirqsclrSpec {
    const RESET_VALUE: u32 = 0;
}
