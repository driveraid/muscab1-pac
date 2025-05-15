#[doc = "Register `UARTILPR` reader"]
pub type R = crate::R<UartilprSpec>;
#[doc = "Register `UARTILPR` writer"]
pub type W = crate::W<UartilprSpec>;
#[doc = "Field `ILPDVSR` reader - 8-bit low-power divisor value"]
pub type IlpdvsrR = crate::FieldReader;
#[doc = "Field `ILPDVSR` writer - 8-bit low-power divisor value"]
pub type IlpdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value"]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> IlpdvsrR {
        IlpdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value"]
    #[inline(always)]
    pub fn ilpdvsr(&mut self) -> IlpdvsrW<UartilprSpec> {
        IlpdvsrW::new(self, 0)
    }
}
#[doc = "IrDA low-power counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartilpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartilpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartilprSpec;
impl crate::RegisterSpec for UartilprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartilpr::R`](R) reader structure"]
impl crate::Readable for UartilprSpec {}
#[doc = "`write(|w| ..)` method takes [`uartilpr::W`](W) writer structure"]
impl crate::Writable for UartilprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTILPR to value 0"]
impl crate::Resettable for UartilprSpec {}
