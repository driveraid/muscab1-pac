#[doc = "Register `IOPAD_DS1_0` reader"]
pub type R = crate::R<IopadDs1_0Spec>;
#[doc = "Register `IOPAD_DS1_0` writer"]
pub type W = crate::W<IopadDs1_0Spec>;
#[doc = "Field `drive_strength1` reader - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
pub type DriveStrength1R = crate::FieldReader<u32>;
#[doc = "Field `drive_strength1` writer - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
pub type DriveStrength1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength1(&self) -> DriveStrength1R {
        DriveStrength1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength1(&mut self) -> DriveStrength1W<IopadDs1_0Spec> {
        DriveStrength1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ds1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ds1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopadDs1_0Spec;
impl crate::RegisterSpec for IopadDs1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopad_ds1_0::R`](R) reader structure"]
impl crate::Readable for IopadDs1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`iopad_ds1_0::W`](W) writer structure"]
impl crate::Writable for IopadDs1_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPAD_DS1_0 to value 0xffff_ffff"]
impl crate::Resettable for IopadDs1_0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
