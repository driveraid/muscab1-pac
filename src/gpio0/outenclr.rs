#[doc = "Register `OUTENCLR` reader"]
pub type R = crate::R<OutenclrSpec>;
#[doc = "Register `OUTENCLR` writer"]
pub type W = crate::W<OutenclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ouptut enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutenclrSpec;
impl crate::RegisterSpec for OutenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outenclr::R`](R) reader structure"]
impl crate::Readable for OutenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`outenclr::W`](W) writer structure"]
impl crate::Writable for OutenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTENCLR to value 0"]
impl crate::Resettable for OutenclrSpec {}
