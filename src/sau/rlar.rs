#[doc = "Register `RLAR` reader"]
pub type R = crate::R<RlarSpec>;
#[doc = "Register `RLAR` writer"]
pub type W = crate::W<RlarSpec>;
#[doc = "Field `ENABLE` reader - SAU Region enabled"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - SAU Region enabled"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSC` reader - Non-Secure Callable"]
pub type NscR = crate::BitReader;
#[doc = "Field `NSC` writer - Non-Secure Callable"]
pub type NscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LADDR` reader - Limit Address"]
pub type LaddrR = crate::FieldReader<u32>;
#[doc = "Field `LADDR` writer - Limit Address"]
pub type LaddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - SAU Region enabled"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Secure Callable"]
    #[inline(always)]
    pub fn nsc(&self) -> NscR {
        NscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:31 - Limit Address"]
    #[inline(always)]
    pub fn laddr(&self) -> LaddrR {
        LaddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - SAU Region enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<RlarSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Non-Secure Callable"]
    #[inline(always)]
    pub fn nsc(&mut self) -> NscW<RlarSpec> {
        NscW::new(self, 1)
    }
    #[doc = "Bits 5:31 - Limit Address"]
    #[inline(always)]
    pub fn laddr(&mut self) -> LaddrW<RlarSpec> {
        LaddrW::new(self, 5)
    }
}
#[doc = "Region Limit Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RlarSpec;
impl crate::RegisterSpec for RlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlar::R`](R) reader structure"]
impl crate::Readable for RlarSpec {}
#[doc = "`write(|w| ..)` method takes [`rlar::W`](W) writer structure"]
impl crate::Writable for RlarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLAR to value 0"]
impl crate::Resettable for RlarSpec {
    const RESET_VALUE: u32 = 0;
}
