#[doc = "Register `UARTLCR_H` reader"]
pub type R = crate::R<UartlcrHSpec>;
#[doc = "Register `UARTLCR_H` writer"]
pub type W = crate::W<UartlcrHSpec>;
#[doc = "Field `BRK` reader - Send break"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - Send break"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - Even parity select"]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - Even parity select"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - Two stop bits select"]
pub type Stp2R = crate::BitReader;
#[doc = "Field `STP2` writer - Two stop bits select"]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - Enable FIFOs"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - Enable FIFOs"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLEN` reader - Word length"]
pub type WlenR = crate::FieldReader;
#[doc = "Field `WLEN` writer - Word length"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPS` reader - Stick parity select"]
pub type SpsR = crate::BitReader;
#[doc = "Field `SPS` writer - Stick parity select"]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Two stop bits select"]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Word length"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<UartlcrHSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<UartlcrHSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - Even parity select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<UartlcrHSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Two stop bits select"]
    #[inline(always)]
    pub fn stp2(&mut self) -> Stp2W<UartlcrHSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<UartlcrHSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Word length"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WlenW<UartlcrHSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - Stick parity select"]
    #[inline(always)]
    pub fn sps(&mut self) -> SpsW<UartlcrHSpec> {
        SpsW::new(self, 7)
    }
}
#[doc = "Line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlcr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartlcrHSpec;
impl crate::RegisterSpec for UartlcrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartlcr_h::R`](R) reader structure"]
impl crate::Readable for UartlcrHSpec {}
#[doc = "`write(|w| ..)` method takes [`uartlcr_h::W`](W) writer structure"]
impl crate::Writable for UartlcrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTLCR_H to value 0"]
impl crate::Resettable for UartlcrHSpec {}
