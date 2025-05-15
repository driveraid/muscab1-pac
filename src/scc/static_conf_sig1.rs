#[doc = "Register `STATIC_CONF_SIG1` reader"]
pub type R = crate::R<StaticConfSig1Spec>;
#[doc = "Register `STATIC_CONF_SIG1` writer"]
pub type W = crate::W<StaticConfSig1Spec>;
#[doc = "Field `TISBYPASSIN` reader - Cross Trigger Interface synchronous bypass on CTITRIGIN"]
pub type TisbypassinR = crate::FieldReader;
#[doc = "Field `TISBYPASSIN` writer - Cross Trigger Interface synchronous bypass on CTITRIGIN"]
pub type TisbypassinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TISBYPASSACK` reader - Cross Trigger Interface synchronous bypass on CTITRIGOUTACK"]
pub type TisbypassackR = crate::FieldReader;
#[doc = "Field `TISBYPASSACK` writer - Cross Trigger Interface synchronous bypass on CTITRIGOUTACK"]
pub type TisbypassackW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIHSBYPASS` reader - Cross Trigger Interface handshake bypass on CTITRIGOUT"]
pub type TihsbypassR = crate::FieldReader;
#[doc = "Field `TIHSBYPASS` writer - Cross Trigger Interface handshake bypass on CTITRIGOUT"]
pub type TihsbypassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TINIDENSEL` reader - NIDEN mask on CTITRIGINT"]
pub type TinidenselR = crate::FieldReader;
#[doc = "Field `TINIDENSEL` writer - NIDEN mask on CTITRIGINT"]
pub type TinidenselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TODBGENSEL` reader - DBGEN mask on CTITRIGOUT"]
pub type TodbgenselR = crate::FieldReader;
#[doc = "Field `TODBGENSEL` writer - DBGEN mask on CTITRIGOUT"]
pub type TodbgenselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Cross Trigger Interface synchronous bypass on CTITRIGIN"]
    #[inline(always)]
    pub fn tisbypassin(&self) -> TisbypassinR {
        TisbypassinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Cross Trigger Interface synchronous bypass on CTITRIGOUTACK"]
    #[inline(always)]
    pub fn tisbypassack(&self) -> TisbypassackR {
        TisbypassackR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Cross Trigger Interface handshake bypass on CTITRIGOUT"]
    #[inline(always)]
    pub fn tihsbypass(&self) -> TihsbypassR {
        TihsbypassR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - NIDEN mask on CTITRIGINT"]
    #[inline(always)]
    pub fn tinidensel(&self) -> TinidenselR {
        TinidenselR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - DBGEN mask on CTITRIGOUT"]
    #[inline(always)]
    pub fn todbgensel(&self) -> TodbgenselR {
        TodbgenselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cross Trigger Interface synchronous bypass on CTITRIGIN"]
    #[inline(always)]
    pub fn tisbypassin(&mut self) -> TisbypassinW<StaticConfSig1Spec> {
        TisbypassinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Cross Trigger Interface synchronous bypass on CTITRIGOUTACK"]
    #[inline(always)]
    pub fn tisbypassack(&mut self) -> TisbypassackW<StaticConfSig1Spec> {
        TisbypassackW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Cross Trigger Interface handshake bypass on CTITRIGOUT"]
    #[inline(always)]
    pub fn tihsbypass(&mut self) -> TihsbypassW<StaticConfSig1Spec> {
        TihsbypassW::new(self, 12)
    }
    #[doc = "Bits 16:23 - NIDEN mask on CTITRIGINT"]
    #[inline(always)]
    pub fn tinidensel(&mut self) -> TinidenselW<StaticConfSig1Spec> {
        TinidenselW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DBGEN mask on CTITRIGOUT"]
    #[inline(always)]
    pub fn todbgensel(&mut self) -> TodbgenselW<StaticConfSig1Spec> {
        TodbgenselW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`static_conf_sig1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static_conf_sig1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaticConfSig1Spec;
impl crate::RegisterSpec for StaticConfSig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static_conf_sig1::R`](R) reader structure"]
impl crate::Readable for StaticConfSig1Spec {}
#[doc = "`write(|w| ..)` method takes [`static_conf_sig1::W`](W) writer structure"]
impl crate::Writable for StaticConfSig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC_CONF_SIG1 to value 0"]
impl crate::Resettable for StaticConfSig1Spec {}
