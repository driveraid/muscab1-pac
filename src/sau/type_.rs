#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Field `SREGION` reader - Number of implemented SAU regions"]
pub type SregionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of implemented SAU regions"]
    #[inline(always)]
    pub fn sregion(&self) -> SregionR {
        SregionR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`reset()` method sets TYPE to value 0"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u32 = 0;
}
