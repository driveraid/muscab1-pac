#[doc = "Register `UARTFBRD` reader"]
pub type R = crate::R<UartfbrdSpec>;
#[doc = "Register `UARTFBRD` writer"]
pub type W = crate::W<UartfbrdSpec>;
#[doc = "Field `BAUD_DIVINT` reader - The integer baud rate divisor"]
pub type BaudDivintR = crate::FieldReader;
#[doc = "Field `BAUD_DIVINT` writer - The integer baud rate divisor"]
pub type BaudDivintW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The integer baud rate divisor"]
    #[inline(always)]
    pub fn baud_divint(&self) -> BaudDivintR {
        BaudDivintR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The integer baud rate divisor"]
    #[inline(always)]
    pub fn baud_divint(&mut self) -> BaudDivintW<UartfbrdSpec> {
        BaudDivintW::new(self, 0)
    }
}
#[doc = "Fractional baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartfbrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartfbrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartfbrdSpec;
impl crate::RegisterSpec for UartfbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartfbrd::R`](R) reader structure"]
impl crate::Readable for UartfbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`uartfbrd::W`](W) writer structure"]
impl crate::Writable for UartfbrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTFBRD to value 0"]
impl crate::Resettable for UartfbrdSpec {
    const RESET_VALUE: u32 = 0;
}
