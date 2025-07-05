#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<Pidr4Spec>;
#[doc = "Field `bit[3_0]` reader - jep106_c_code"]
pub type Bit3_0R = crate::FieldReader;
#[doc = "Field `bit[7_4]` reader - block count"]
pub type Bit7_4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - jep106_c_code"]
    #[inline(always)]
    pub fn bit3_0(&self) -> Bit3_0R {
        Bit3_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - block count"]
    #[inline(always)]
    pub fn bit7_4(&self) -> Bit7_4R {
        Bit7_4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr4Spec;
impl crate::RegisterSpec for Pidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for Pidr4Spec {}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for Pidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
