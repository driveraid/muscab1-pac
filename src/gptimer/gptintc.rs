#[doc = "Register `GPTINTC` reader"]
pub type R = crate::R<GptintcSpec>;
#[doc = "Register `GPTINTC` writer"]
pub type W = crate::W<GptintcSpec>;
#[doc = "Field `GPTINTC` reader - Writing 0b1 disables the ALARM\\[n\\] interrupt"]
pub type GptintcR = crate::FieldReader;
#[doc = "Field `GPTINTC` writer - Writing 0b1 disables the ALARM\\[n\\] interrupt"]
pub type GptintcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Writing 0b1 disables the ALARM\\[n\\] interrupt"]
    #[inline(always)]
    pub fn gptintc(&self) -> GptintcR {
        GptintcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Writing 0b1 disables the ALARM\\[n\\] interrupt"]
    #[inline(always)]
    pub fn gptintc(&mut self) -> GptintcW<GptintcSpec> {
        GptintcW::new(self, 0)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptintc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptintc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptintcSpec;
impl crate::RegisterSpec for GptintcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptintc::R`](R) reader structure"]
impl crate::Readable for GptintcSpec {}
#[doc = "`write(|w| ..)` method takes [`gptintc::W`](W) writer structure"]
impl crate::Writable for GptintcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTINTC to value 0"]
impl crate::Resettable for GptintcSpec {}
