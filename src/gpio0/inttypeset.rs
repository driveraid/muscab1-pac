#[doc = "Register `INTTYPESET` reader"]
pub type R = crate::R<InttypesetSpec>;
#[doc = "Register `INTTYPESET` writer"]
pub type W = crate::W<InttypesetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt type set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttypeset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttypeset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttypesetSpec;
impl crate::RegisterSpec for InttypesetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttypeset::R`](R) reader structure"]
impl crate::Readable for InttypesetSpec {}
#[doc = "`write(|w| ..)` method takes [`inttypeset::W`](W) writer structure"]
impl crate::Writable for InttypesetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTTYPESET to value 0"]
impl crate::Resettable for InttypesetSpec {}
