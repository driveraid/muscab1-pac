#[doc = "Register `BUSWAIT` reader"]
pub type R = crate::R<BuswaitSpec>;
#[doc = "Register `BUSWAIT` writer"]
pub type W = crate::W<BuswaitSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bus Access wait control after reset\n\nYou can [`read`](crate::Reg::read) this register and get [`buswait::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buswait::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuswaitSpec;
impl crate::RegisterSpec for BuswaitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buswait::R`](R) reader structure"]
impl crate::Readable for BuswaitSpec {}
#[doc = "`write(|w| ..)` method takes [`buswait::W`](W) writer structure"]
impl crate::Writable for BuswaitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUSWAIT to value 0"]
impl crate::Resettable for BuswaitSpec {}
