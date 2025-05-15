#[doc = "Register `PVT_CTRL` reader"]
pub type R = crate::R<PvtCtrlSpec>;
#[doc = "Register `PVT_CTRL` writer"]
pub type W = crate::W<PvtCtrlSpec>;
#[doc = "Field `TSTSENNUM` reader - Select PVT sensor to write to and read from"]
pub type TstsennumR = crate::FieldReader;
#[doc = "Field `TSTSENNUM` writer - Select PVT sensor to write to and read from"]
pub type TstsennumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select PVT sensor to write to and read from"]
    #[inline(always)]
    pub fn tstsennum(&self) -> TstsennumR {
        TstsennumR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select PVT sensor to write to and read from"]
    #[inline(always)]
    pub fn tstsennum(&mut self) -> TstsennumW<PvtCtrlSpec> {
        TstsennumW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PvtCtrlSpec;
impl crate::RegisterSpec for PvtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt_ctrl::R`](R) reader structure"]
impl crate::Readable for PvtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pvt_ctrl::W`](W) writer structure"]
impl crate::Writable for PvtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PVT_CTRL to value 0"]
impl crate::Resettable for PvtCtrlSpec {}
