#[doc = "Register `AHBSPPPC0` reader"]
pub type R = crate::R<Ahbspppc0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Secure Unprivileged Access AHB slave Peripheral Protection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbspppc0Spec;
impl crate::RegisterSpec for Ahbspppc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbspppc0::R`](R) reader structure"]
impl crate::Readable for Ahbspppc0Spec {}
#[doc = "`reset()` method sets AHBSPPPC0 to value 0"]
impl crate::Resettable for Ahbspppc0Spec {
    const RESET_VALUE: u32 = 0;
}
