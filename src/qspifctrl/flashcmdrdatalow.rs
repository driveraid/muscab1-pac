#[doc = "Register `FLASHCMDRDATALOW` reader"]
pub type R = crate::R<FlashcmdrdatalowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Flash Command Read Data Register (Lower)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdrdatalow::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdrdatalowSpec;
impl crate::RegisterSpec for FlashcmdrdatalowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdrdatalow::R`](R) reader structure"]
impl crate::Readable for FlashcmdrdatalowSpec {}
#[doc = "`reset()` method sets FLASHCMDRDATALOW to value 0"]
impl crate::Resettable for FlashcmdrdatalowSpec {}
