#[doc = "Register `FLASHCMDADDR` reader"]
pub type R = crate::R<FlashcmdaddrSpec>;
#[doc = "Register `FLASHCMDADDR` writer"]
pub type W = crate::W<FlashcmdaddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Flash Command Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcmdaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcmdaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcmdaddrSpec;
impl crate::RegisterSpec for FlashcmdaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcmdaddr::R`](R) reader structure"]
impl crate::Readable for FlashcmdaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcmdaddr::W`](W) writer structure"]
impl crate::Writable for FlashcmdaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCMDADDR to value 0"]
impl crate::Resettable for FlashcmdaddrSpec {}
