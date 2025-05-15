#[doc = "Register `PIDR3` reader"]
pub type R = crate::R<Pidr3Spec>;
#[doc = "Field `bit[3_0]` reader - Customer modification number"]
pub type Bit3_0R = crate::FieldReader;
#[doc = "Field `bit[7_4]` reader - ECO revision number"]
pub type Bit7_4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer modification number"]
    #[inline(always)]
    pub fn bit3_0(&self) -> Bit3_0R {
        Bit3_0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ECO revision number"]
    #[inline(always)]
    pub fn bit7_4(&self) -> Bit7_4R {
        Bit7_4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr3Spec;
impl crate::RegisterSpec for Pidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr3::R`](R) reader structure"]
impl crate::Readable for Pidr3Spec {}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for Pidr3Spec {}
