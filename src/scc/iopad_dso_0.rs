#[doc = "Register `IOPAD_DSO_0` reader"]
pub type R = crate::R<IopadDso0Spec>;
#[doc = "Register `IOPAD_DSO_0` writer"]
pub type W = crate::W<IopadDso0Spec>;
#[doc = "Field `drive_strength0` reader - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
pub type DriveStrength0R = crate::FieldReader<u32>;
#[doc = "Field `drive_strength0` writer - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
pub type DriveStrength0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength0(&self) -> DriveStrength0R {
        DriveStrength0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength0(&mut self) -> DriveStrength0W<IopadDso0Spec> {
        DriveStrength0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_dso_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_dso_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadDso0Spec;
impl crate::RegisterSpec for IopadDso0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_dso_0::R`](R) reader structure"]
impl crate::Readable for IopadDso0Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_dso_0::W`](W) writer structure"]
impl crate::Writable for IopadDso0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_DSO_0 to value 0xfff0_0000"]
impl crate::Resettable for IopadDso0Spec {
    const RESET_VALUE: u32 = 0xfff0_0000;
}
