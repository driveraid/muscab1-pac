#[doc = "Register `FLASH1_DOUT_3` reader"]
pub type R = crate::R<Flash1Dout3Spec>;
#[doc = "Field `scc_flash1_dout3` reader - eFlash 1 data output\\[127:96\\]"]
pub type SccFlash1Dout3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 1 data output\\[127:96\\]"]
    #[inline(always)]
    pub fn scc_flash1_dout3(&self) -> SccFlash1Dout3R {
        SccFlash1Dout3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash1Dout3Spec;
impl crate::RegisterSpec for Flash1Dout3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash1_dout_3::R`](R) reader structure"]
impl crate::Readable for Flash1Dout3Spec {}
#[doc = "`reset()` method sets FLASH1_DOUT_3 to value 0xffff_ffff"]
impl crate::Resettable for Flash1Dout3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
