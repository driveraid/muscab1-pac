#[doc = "Register `FLASH1_DOUT_2` reader"]
pub type R = crate::R<Flash1Dout2Spec>;
#[doc = "Field `scc_flash1_dout2` reader - eFlash 1 data output\\[95:64\\]"]
pub type SccFlash1Dout2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 1 data output\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash1_dout2(&self) -> SccFlash1Dout2R {
        SccFlash1Dout2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash1Dout2Spec;
impl crate::RegisterSpec for Flash1Dout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash1_dout_2::R`](R) reader structure"]
impl crate::Readable for Flash1Dout2Spec {}
#[doc = "`reset()` method sets FLASH1_DOUT_2 to value 0xffff_ffff"]
impl crate::Resettable for Flash1Dout2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
