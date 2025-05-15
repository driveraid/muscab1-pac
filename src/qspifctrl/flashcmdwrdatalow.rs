#[doc = "Register `FLASHCMDWRDATALOW` reader"]
pub type R = crate::R<FlashcmdwrdatalowSpec>;
#[doc = "Register `FLASHCMDWRDATALOW` writer"]
pub type W = crate::W<FlashcmdwrdatalowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Command Write Data Register (Lower)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdwrdatalow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdwrdatalow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdwrdatalowSpec;
impl crate::RegisterSpec for FlashcmdwrdatalowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdwrdatalow::R`](R) reader structure"]
impl crate::Readable for FlashcmdwrdatalowSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcmdwrdatalow::W`](W) writer structure"]
impl crate::Writable for FlashcmdwrdatalowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCMDWRDATALOW to value 0"]
impl crate::Resettable for FlashcmdwrdatalowSpec {}
