#[doc = "Register `GPTALARM1` reader"]
pub type R = crate::R<Gptalarm1Spec>;
#[doc = "Register `GPTALARM1` writer"]
pub type W = crate::W<Gptalarm1Spec>;
#[doc = "Field `GPTALARM1_DATA` reader - Value that triggers the ALARM1 interrupt when the counter reaches that value"]
pub type Gptalarm1DataR = crate::FieldReader<u32>;
#[doc = "Field `GPTALARM1_DATA` writer - Value that triggers the ALARM1 interrupt when the counter reaches that value"]
pub type Gptalarm1DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value that triggers the ALARM1 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm1_data(&self) -> Gptalarm1DataR {
        Gptalarm1DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value that triggers the ALARM1 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm1_data(&mut self) -> Gptalarm1DataW<Gptalarm1Spec> {
        Gptalarm1DataW::new(self, 0)
    }
}
#[doc = "ALARM1 data value register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptalarm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptalarm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gptalarm1Spec;
impl crate::RegisterSpec for Gptalarm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptalarm1::R`](R) reader structure"]
impl crate::Readable for Gptalarm1Spec {}
#[doc = "`write(|w| ..)` method takes [`gptalarm1::W`](W) writer structure"]
impl crate::Writable for Gptalarm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTALARM1 to value 0"]
impl crate::Resettable for Gptalarm1Spec {
    const RESET_VALUE: u32 = 0;
}
