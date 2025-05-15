#[doc = "Register `AZ_OTP_RD_DATA` reader"]
pub type R = crate::R<AzOtpRdDataSpec>;
#[doc = "Register `AZ_OTP_RD_DATA` writer"]
pub type W = crate::W<AzOtpRdDataSpec>;
#[doc = "Field `az_otp_rd_data` reader - Alcatraz OTP read data"]
pub type AzOtpRdDataR = crate::FieldReader<u32>;
#[doc = "Field `az_otp_rd_data` writer - Alcatraz OTP read data"]
pub type AzOtpRdDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alcatraz OTP read data"]
    #[inline(always)]
    pub fn az_otp_rd_data(&self) -> AzOtpRdDataR {
        AzOtpRdDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alcatraz OTP read data"]
    #[inline(always)]
    pub fn az_otp_rd_data(&mut self) -> AzOtpRdDataW<AzOtpRdDataSpec> {
        AzOtpRdDataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_otp_rd_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_otp_rd_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzOtpRdDataSpec;
impl crate::RegisterSpec for AzOtpRdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_otp_rd_data::R`](R) reader structure"]
impl crate::Readable for AzOtpRdDataSpec {}
#[doc = "`write(|w| ..)` method takes [`az_otp_rd_data::W`](W) writer structure"]
impl crate::Writable for AzOtpRdDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AZ_OTP_RD_DATA to value 0"]
impl crate::Resettable for AzOtpRdDataSpec {}
