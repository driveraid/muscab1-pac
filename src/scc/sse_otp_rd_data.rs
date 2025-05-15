#[doc = "Register `SSE_OTP_RD_DATA` reader"]
pub type R = crate::R<SseOtpRdDataSpec>;
#[doc = "Field `sse_otp_rd_data` reader - SSE-200 OTP read data"]
pub type SseOtpRdDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SSE-200 OTP read data"]
    #[inline(always)]
    pub fn sse_otp_rd_data(&self) -> SseOtpRdDataR {
        SseOtpRdDataR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sse_otp_rd_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SseOtpRdDataSpec;
impl crate::RegisterSpec for SseOtpRdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sse_otp_rd_data::R`](R) reader structure"]
impl crate::Readable for SseOtpRdDataSpec {}
#[doc = "`reset()` method sets SSE_OTP_RD_DATA to value 0"]
impl crate::Resettable for SseOtpRdDataSpec {}
