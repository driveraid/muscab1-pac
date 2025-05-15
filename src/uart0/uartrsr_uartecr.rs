#[doc = "Register `UARTRSR_UARTECR` reader"]
pub type R = crate::R<UartrsrUartecrSpec>;
#[doc = "Register `UARTRSR_UARTECR` writer"]
pub type W = crate::W<UartrsrUartecrSpec>;
#[doc = "Field `FE` reader - Framing error: Indicates the received character did not had a valid stop bit"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - Framing error: Indicates the received character did not had a valid stop bit"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Parity error: Indicates that the parity of the received data character does not match the parity selected"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity error: Indicates that the parity of the received data character does not match the parity selected"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Break error: Indicates that the received data input was held LOW for longer than a full-word transmission time"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - Break error: Indicates that the received data input was held LOW for longer than a full-word transmission time"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - Overrunerror: Indicates if data is received and the receive FIFO is already full."]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Overrunerror: Indicates if data is received and the receive FIFO is already full."]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Framing error: Indicates the received character did not had a valid stop bit"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error: Indicates that the parity of the received data character does not match the parity selected"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break error: Indicates that the received data input was held LOW for longer than a full-word transmission time"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrunerror: Indicates if data is received and the receive FIFO is already full."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Framing error: Indicates the received character did not had a valid stop bit"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<UartrsrUartecrSpec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity error: Indicates that the parity of the received data character does not match the parity selected"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<UartrsrUartecrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Break error: Indicates that the received data input was held LOW for longer than a full-word transmission time"]
    #[inline(always)]
    pub fn be(&mut self) -> BeW<UartrsrUartecrSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrunerror: Indicates if data is received and the receive FIFO is already full."]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<UartrsrUartecrSpec> {
        OeW::new(self, 3)
    }
}
#[doc = "Receive status register/error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrsr_uartecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrsr_uartecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartrsrUartecrSpec;
impl crate::RegisterSpec for UartrsrUartecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartrsr_uartecr::R`](R) reader structure"]
impl crate::Readable for UartrsrUartecrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartrsr_uartecr::W`](W) writer structure"]
impl crate::Writable for UartrsrUartecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTRSR_UARTECR to value 0"]
impl crate::Resettable for UartrsrUartecrSpec {}
