#[doc = "Register `INT_INFO1` reader"]
pub type R = crate::R<IntInfo1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Interrupt information 1\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntInfo1Spec;
impl crate::RegisterSpec for IntInfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_info1::R`](R) reader structure"]
impl crate::Readable for IntInfo1Spec {}
#[doc = "`reset()` method sets INT_INFO1 to value 0"]
impl crate::Resettable for IntInfo1Spec {}
