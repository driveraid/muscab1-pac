#[doc = "Register `PIDR5` reader"]
pub type R = crate::R<Pidr5Spec>;
#[doc = "Field `bit[3_0]` reader - Part number"]
pub type Bit3_0R = crate::FieldReader;
#[doc = "Field `bit[7_4]` reader - jep106_id_3_0"]
pub type Bit7_4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Part number"]
    #[inline(always)]
    pub fn bit3_0(&self) -> Bit3_0R {
        Bit3_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - jep106_id_3_0"]
    #[inline(always)]
    pub fn bit7_4(&self) -> Bit7_4R {
        Bit7_4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr5Spec;
impl crate::RegisterSpec for Pidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr5::R`](R) reader structure"]
impl crate::Readable for Pidr5Spec {}
#[doc = "`reset()` method sets PIDR5 to value 0"]
impl crate::Resettable for Pidr5Spec {
    const RESET_VALUE: u32 = 0;
}
