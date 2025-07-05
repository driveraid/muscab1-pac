#[doc = "Register `GPTALARM0` reader"]
pub type R = crate::R<Gptalarm0Spec>;
#[doc = "Register `GPTALARM0` writer"]
pub type W = crate::W<Gptalarm0Spec>;
#[doc = "Field `GPTALARM0_DATA` reader - Value that triggers the ALARM0 interrupt when the counter reaches that value"]
pub type Gptalarm0DataR = crate::FieldReader<u32>;
#[doc = "Field `GPTALARM0_DATA` writer - Value that triggers the ALARM0 interrupt when the counter reaches that value"]
pub type Gptalarm0DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value that triggers the ALARM0 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm0_data(&self) -> Gptalarm0DataR {
        Gptalarm0DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value that triggers the ALARM0 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm0_data(&mut self) -> Gptalarm0DataW<Gptalarm0Spec> {
        Gptalarm0DataW::new(self, 0)
    }
}
#[doc = "ALARM0 data value register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptalarm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptalarm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gptalarm0Spec;
impl crate::RegisterSpec for Gptalarm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptalarm0::R`](R) reader structure"]
impl crate::Readable for Gptalarm0Spec {}
#[doc = "`write(|w| ..)` method takes [`gptalarm0::W`](W) writer structure"]
impl crate::Writable for Gptalarm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTALARM0 to value 0"]
impl crate::Resettable for Gptalarm0Spec {
    const RESET_VALUE: u32 = 0;
}
