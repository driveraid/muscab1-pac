#[doc = "Register `IOPAD_DS1_1` reader"]
pub type R = crate::R<IopadDs1_1Spec>;
#[doc = "Register `IOPAD_DS1_1` writer"]
pub type W = crate::W<IopadDs1_1Spec>;
#[doc = "Field `drive_strength_1` reader - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
pub type DriveStrength1R = crate::FieldReader;
#[doc = "Field `drive_strength_1` writer - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
pub type DriveStrength1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_1(&self) -> DriveStrength1R {
        DriveStrength1R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_1(&mut self) -> DriveStrength1W<IopadDs1_1Spec> {
        DriveStrength1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ds1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ds1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadDs1_1Spec;
impl crate::RegisterSpec for IopadDs1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_ds1_1::R`](R) reader structure"]
impl crate::Readable for IopadDs1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_ds1_1::W`](W) writer structure"]
impl crate::Writable for IopadDs1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_DS1_1 to value 0xffff_ffff"]
impl crate::Resettable for IopadDs1_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
