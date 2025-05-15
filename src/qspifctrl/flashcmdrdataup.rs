#[doc = "Register `FLASHCMDRDATAUP` reader"]
pub type R = crate::R<FlashcmdrdataupSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Flash Command Read Data Register (Upper)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdrdataup::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdrdataupSpec;
impl crate::RegisterSpec for FlashcmdrdataupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdrdataup::R`](R) reader structure"]
impl crate::Readable for FlashcmdrdataupSpec {}
#[doc = "`reset()` method sets FLASHCMDRDATAUP to value 0"]
impl crate::Resettable for FlashcmdrdataupSpec {}
