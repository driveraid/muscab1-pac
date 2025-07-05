#[doc = "Register `UARTIBRD` reader"]
pub type R = crate::R<UartibrdSpec>;
#[doc = "Register `UARTIBRD` writer"]
pub type W = crate::W<UartibrdSpec>;
#[doc = "Field `BAUD_DIVINT` reader - The integer baud rate divisor"]
pub type BaudDivintR = crate::FieldReader<u16>;
#[doc = "Field `BAUD_DIVINT` writer - The integer baud rate divisor"]
pub type BaudDivintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor"]
    #[inline(always)]
    pub fn baud_divint(&self) -> BaudDivintR {
        BaudDivintR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor"]
    #[inline(always)]
    pub fn baud_divint(&mut self) -> BaudDivintW<UartibrdSpec> {
        BaudDivintW::new(self, 0)
    }
}
#[doc = "Integer baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartibrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartibrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartibrdSpec;
impl crate::RegisterSpec for UartibrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartibrd::R`](R) reader structure"]
impl crate::Readable for UartibrdSpec {}
#[doc = "`write(|w| ..)` method takes [`uartibrd::W`](W) writer structure"]
impl crate::Writable for UartibrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTIBRD to value 0"]
impl crate::Resettable for UartibrdSpec {
    const RESET_VALUE: u32 = 0;
}
