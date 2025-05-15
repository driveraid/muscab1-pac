#[doc = "Register `FLASH0_DOUT_2` reader"]
pub type R = crate::R<Flash0Dout2Spec>;
#[doc = "Field `scc_flash0_dout2` reader - eFlash 0 data output\\[95:64\\]"]
pub type SccFlash0Dout2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 data output\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash0_dout2(&self) -> SccFlash0Dout2R {
        SccFlash0Dout2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flash0Dout2Spec;
impl crate::RegisterSpec for Flash0Dout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash0_dout_2::R`](R) reader structure"]
impl crate::Readable for Flash0Dout2Spec {}
#[doc = "`reset()` method sets FLASH0_DOUT_2 to value 0xffff_ffff"]
impl crate::Resettable for Flash0Dout2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
