#[doc = "Register `UARTDMACR` reader"]
pub type R = crate::R<UartdmacrSpec>;
#[doc = "Register `UARTDMACR` writer"]
pub type W = crate::W<UartdmacrSpec>;
#[doc = "Receive DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmae {
    #[doc = "0: Receive DMA is disabled"]
    Disable = 0,
    #[doc = "1: Receive DMA is enabled"]
    Enable = 1,
}
impl From<Rxdmae> for bool {
    #[inline(always)]
    fn from(variant: Rxdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable"]
pub type RxdmaeR = crate::BitReader<Rxdmae>;
impl RxdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmae {
        match self.bits {
            false => Rxdmae::Disable,
            true => Rxdmae::Enable,
        }
    }
    #[doc = "Receive DMA is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxdmae::Disable
    }
    #[doc = "Receive DMA is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxdmae::Enable
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA enable"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Rxdmae>;
impl<'a, REG> RxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive DMA is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::Disable)
    }
    #[doc = "Receive DMA is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::Enable)
    }
}
#[doc = "Transmit DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmae {
    #[doc = "0: Transmit DMA is disabled"]
    Disable = 0,
    #[doc = "1: Transmit DMA is enabled"]
    Enable = 1,
}
impl From<Txdmae> for bool {
    #[inline(always)]
    fn from(variant: Txdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA enable"]
pub type TxdmaeR = crate::BitReader<Txdmae>;
impl TxdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmae {
        match self.bits {
            false => Txdmae::Disable,
            true => Txdmae::Enable,
        }
    }
    #[doc = "Transmit DMA is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txdmae::Disable
    }
    #[doc = "Transmit DMA is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txdmae::Enable
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA enable"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Txdmae>;
impl<'a, REG> TxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit DMA is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::Disable)
    }
    #[doc = "Transmit DMA is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::Enable)
    }
}
#[doc = "DMA on error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaonerr {
    #[doc = "0: DMA receive request outputs are enabled when the UART error interrupt is asserted"]
    Disable = 0,
    #[doc = "1: DMA receive request outputs are disabled when the UART error interrupt is asserted"]
    Enable = 1,
}
impl From<Dmaonerr> for bool {
    #[inline(always)]
    fn from(variant: Dmaonerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAONERR` reader - DMA on error"]
pub type DmaonerrR = crate::BitReader<Dmaonerr>;
impl DmaonerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaonerr {
        match self.bits {
            false => Dmaonerr::Disable,
            true => Dmaonerr::Enable,
        }
    }
    #[doc = "DMA receive request outputs are enabled when the UART error interrupt is asserted"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaonerr::Disable
    }
    #[doc = "DMA receive request outputs are disabled when the UART error interrupt is asserted"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaonerr::Enable
    }
}
#[doc = "Field `DMAONERR` writer - DMA on error"]
pub type DmaonerrW<'a, REG> = crate::BitWriter<'a, REG, Dmaonerr>;
impl<'a, REG> DmaonerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA receive request outputs are enabled when the UART error interrupt is asserted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaonerr::Disable)
    }
    #[doc = "DMA receive request outputs are disabled when the UART error interrupt is asserted"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaonerr::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DmaonerrR {
        DmaonerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RxdmaeW<UartdmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TxdmaeW<UartdmacrSpec> {
        TxdmaeW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&mut self) -> DmaonerrW<UartdmacrSpec> {
        DmaonerrW::new(self, 2)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartdmacrSpec;
impl crate::RegisterSpec for UartdmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdmacr::R`](R) reader structure"]
impl crate::Readable for UartdmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdmacr::W`](W) writer structure"]
impl crate::Writable for UartdmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTDMACR to value 0"]
impl crate::Resettable for UartdmacrSpec {
    const RESET_VALUE: u32 = 0;
}
