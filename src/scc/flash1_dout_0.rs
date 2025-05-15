#[doc = "Register `FLASH1_DOUT_0` reader"]
pub type R = crate::R<Flash1Dout0Spec>;
#[doc = "Field `scc_flash1_dout0` reader - eFlash 1 data output\\[31:0\\]"]
pub type SccFlash1Dout0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 1 data output\\[31:0\\]"]
    #[inline(always)]
    pub fn scc_flash1_dout0(&self) -> SccFlash1Dout0R {
        SccFlash1Dout0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash1Dout0Spec;
impl crate::RegisterSpec for Flash1Dout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash1_dout_0::R`](R) reader structure"]
impl crate::Readable for Flash1Dout0Spec {}
#[doc = "`reset()` method sets FLASH1_DOUT_0 to value 0xffff_ffff"]
impl crate::Resettable for Flash1Dout0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
