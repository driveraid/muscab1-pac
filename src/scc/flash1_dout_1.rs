#[doc = "Register `FLASH1_DOUT_1` reader"]
pub type R = crate::R<Flash1Dout1Spec>;
#[doc = "Field `scc_flash1_dout1` reader - eFlash 1 data output\\[63:32\\]"]
pub type SccFlash1Dout1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 1 data output\\[63:32\\]"]
    #[inline(always)]
    pub fn scc_flash1_dout1(&self) -> SccFlash1Dout1R {
        SccFlash1Dout1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash1Dout1Spec;
impl crate::RegisterSpec for Flash1Dout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash1_dout_1::R`](R) reader structure"]
impl crate::Readable for Flash1Dout1Spec {}
#[doc = "`reset()` method sets FLASH1_DOUT_1 to value 0xffff_ffff"]
impl crate::Resettable for Flash1Dout1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
