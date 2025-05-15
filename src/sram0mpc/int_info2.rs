#[doc = "Register `INT_INFO2` reader"]
pub type R = crate::R<IntInfo2Spec>;
#[doc = "Field `bit[15_0]` reader - hmaster"]
pub type Bit15_0R = crate::FieldReader<u16>;
#[doc = "Field `bit[16]` reader - hnonsec"]
pub type Bit16R = crate::BitReader;
#[doc = "Field `bit[17]` reader - cfg_ns"]
pub type Bit17R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - hmaster"]
    #[inline(always)]
    pub fn bit15_0(&self) -> Bit15_0R {
        Bit15_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - hnonsec"]
    #[inline(always)]
    pub fn bit16(&self) -> Bit16R {
        Bit16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - cfg_ns"]
    #[inline(always)]
    pub fn bit17(&self) -> Bit17R {
        Bit17R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt information 2\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntInfo2Spec;
impl crate::RegisterSpec for IntInfo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_info2::R`](R) reader structure"]
impl crate::Readable for IntInfo2Spec {}
#[doc = "`reset()` method sets INT_INFO2 to value 0"]
impl crate::Resettable for IntInfo2Spec {}
