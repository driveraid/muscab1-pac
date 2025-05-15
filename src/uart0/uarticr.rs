#[doc = "Register `UARTICR` writer"]
pub type W = crate::W<UarticrSpec>;
#[doc = "Field `RIMIC` writer - nUARTRI modem interrupt clear, write 1 to clear, write 0 has no effect"]
pub type RimicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIC` writer - nUARTCTS modem interrupt clear, write 1 to clear, write 0 has no effect"]
pub type CtsmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIC` writer - nUARTDCD modem interrupt clear, write 1 to clear, write 0 has no effect"]
pub type DcdmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRIC` writer - nUARTDSR modem interrupt clear, write 1 to clear, write 0 has no effect"]
pub type DsricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIC` writer - Receive interrupt clear, write 1 to clear, write 0 has no effect"]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` writer - Transmit interrupt clear, write 1 to clear, write 0 has no effect"]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - Receive timeout interrupt clear, write 1 to clear, write 0 has no effect"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` writer - Framing error interrupt clear, write 1 to clear, write 0 has no effect"]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` writer - Parity error interrupt clear, write 1 to clear, write 0 has no effect"]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` writer - Break error interrupt clear, write 1 to clear, write 0 has no effect"]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` writer - Overrun error interrupt clear, write 1 to clear, write 0 has no effect"]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn rimic(&mut self) -> RimicW<UarticrSpec> {
        RimicW::new(self, 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CtsmicW<UarticrSpec> {
        CtsmicW::new(self, 1)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DcdmicW<UarticrSpec> {
        DcdmicW::new(self, 2)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn dsric(&mut self) -> DsricW<UarticrSpec> {
        DsricW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn rxic(&mut self) -> RxicW<UarticrSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn txic(&mut self) -> TxicW<UarticrSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<UarticrSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn feic(&mut self) -> FeicW<UarticrSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn peic(&mut self) -> PeicW<UarticrSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn beic(&mut self) -> BeicW<UarticrSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear, write 1 to clear, write 0 has no effect"]
    #[inline(always)]
    pub fn oeic(&mut self) -> OeicW<UarticrSpec> {
        OeicW::new(self, 10)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uarticr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UarticrSpec;
impl crate::RegisterSpec for UarticrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uarticr::W`](W) writer structure"]
impl crate::Writable for UarticrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTICR to value 0"]
impl crate::Resettable for UarticrSpec {}
