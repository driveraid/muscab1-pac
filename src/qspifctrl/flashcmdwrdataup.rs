#[doc = "Register `FLASHCMDWRDATAUP` reader"]
pub type R = crate::R<FlashcmdwrdataupSpec>;
#[doc = "Register `FLASHCMDWRDATAUP` writer"]
pub type W = crate::W<FlashcmdwrdataupSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Command Write Data Register (Upper)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdwrdataup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdwrdataup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdwrdataupSpec;
impl crate::RegisterSpec for FlashcmdwrdataupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdwrdataup::R`](R) reader structure"]
impl crate::Readable for FlashcmdwrdataupSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcmdwrdataup::W`](W) writer structure"]
impl crate::Writable for FlashcmdwrdataupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCMDWRDATAUP to value 0"]
impl crate::Resettable for FlashcmdwrdataupSpec {}
