#[doc = "Register `UARTDR` reader"]
pub type R = crate::R<UartdrSpec>;
#[doc = "Register `UARTDR` writer"]
pub type W = crate::W<UartdrSpec>;
#[doc = "Field `Data` reader - Receive/Transmit data"]
pub type DataR = crate::FieldReader;
#[doc = "Field `Data` writer - Receive/Transmit data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
#[doc = "Field `OE` reader - Overrun error: Indicates if data is received and the receive FIFO is already full."]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - Overrun error: Indicates if data is received and the receive FIFO is already full."]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Receive/Transmit data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Framing error: Indicates the received character did not had a valid stop bit"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity error: Indicates that the parity of the received data character does not match the parity selected"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Break error: Indicates that the received data input was held LOW for longer than a full-word transmission time"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun error: Indicates if data is received and the receive FIFO is already full."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive/Transmit data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<UartdrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - Framing error: Indicates the received character did not had a valid stop bit"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<UartdrSpec> {
        FeW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity error: Indicates that the parity of the received data character does not match the parity selected"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<UartdrSpec> {
        PeW::new(self, 9)
    }
    #[doc = "Bit 10 - Break error: Indicates that the received data input was held LOW for longer than a full-word transmission time"]
    #[inline(always)]
    pub fn be(&mut self) -> BeW<UartdrSpec> {
        BeW::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun error: Indicates if data is received and the receive FIFO is already full."]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<UartdrSpec> {
        OeW::new(self, 11)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartdrSpec;
impl crate::RegisterSpec for UartdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdr::R`](R) reader structure"]
impl crate::Readable for UartdrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdr::W`](W) writer structure"]
impl crate::Writable for UartdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTDR to value 0"]
impl crate::Resettable for UartdrSpec {
    const RESET_VALUE: u32 = 0;
}
