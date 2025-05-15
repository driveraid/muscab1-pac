#[doc = "Register `IOPAD_DSO_1` reader"]
pub type R = crate::R<IopadDso1Spec>;
#[doc = "Register `IOPAD_DSO_1` writer"]
pub type W = crate::W<IopadDso1Spec>;
#[doc = "Field `drive_strength_0` reader - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
pub type DriveStrength0R = crate::FieldReader;
#[doc = "Field `drive_strength_0` writer - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
pub type DriveStrength0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_0(&self) -> DriveStrength0R {
        DriveStrength0R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_0(&mut self) -> DriveStrength0W<IopadDso1Spec> {
        DriveStrength0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_dso_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_dso_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadDso1Spec;
impl crate::RegisterSpec for IopadDso1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_dso_1::R`](R) reader structure"]
impl crate::Readable for IopadDso1Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_dso_1::W`](W) writer structure"]
impl crate::Writable for IopadDso1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_DSO_1 to value 0xffff_ffff"]
impl crate::Resettable for IopadDso1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
