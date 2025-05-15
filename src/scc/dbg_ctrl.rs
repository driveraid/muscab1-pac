#[doc = "Register `DBG_CTRL` reader"]
pub type R = crate::R<DbgCtrlSpec>;
#[doc = "Register `DBG_CTRL` writer"]
pub type W = crate::W<DbgCtrlSpec>;
#[doc = "Field `SSE_200_DBGENIN` reader - 0: Not enable 1: Enable"]
pub type Sse200DbgeninR = crate::BitReader;
#[doc = "Field `SSE_200_DBGENIN` writer - 0: Not enable 1: Enable"]
pub type Sse200DbgeninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE_200_NIDENIN` reader - 0: Not enable 1: Enable"]
pub type Sse200NideninR = crate::BitReader;
#[doc = "Field `SSE_200_NIDENIN` writer - 0: Not enable 1: Enable"]
pub type Sse200NideninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE_200_SPIDENIN` reader - 0: Not enable 1: Enable"]
pub type Sse200SpideninR = crate::BitReader;
#[doc = "Field `SSE_200_SPIDENIN` writer - 0: Not enable 1: Enable"]
pub type Sse200SpideninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE_200_SPNIDENIN` reader - 0: Not enable 1: Enable"]
pub type Sse200SpnideninR = crate::BitReader;
#[doc = "Field `SSE_200_SPNIDENIN` writer - 0: Not enable 1: Enable"]
pub type Sse200SpnideninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TODBGENSEL0` reader - 0: Enable 1: Mask or bypass"]
pub type Todbgensel0R = crate::BitReader;
#[doc = "Field `TODBGENSEL0` writer - 0: Enable 1: Mask or bypass"]
pub type Todbgensel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TODBGENSEL1` reader - 0: Enable 1: Mask or bypass"]
pub type Todbgensel1R = crate::BitReader;
#[doc = "Field `TODBGENSEL1` writer - 0: Enable 1: Mask or bypass"]
pub type Todbgensel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_DCU_FORCE` reader - SSE-200 debug ports control"]
pub type DbgDcuForceR = crate::FieldReader;
#[doc = "Field `DBG_DCU_FORCE` writer - SSE-200 debug ports control"]
pub type DbgDcuForceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_dbgenin(&self) -> Sse200DbgeninR {
        Sse200DbgeninR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_nidenin(&self) -> Sse200NideninR {
        Sse200NideninR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spidenin(&self) -> Sse200SpideninR {
        Sse200SpideninR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spnidenin(&self) -> Sse200SpnideninR {
        Sse200SpnideninR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel0(&self) -> Todbgensel0R {
        Todbgensel0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel1(&self) -> Todbgensel1R {
        Todbgensel1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 30:31 - SSE-200 debug ports control"]
    #[inline(always)]
    pub fn dbg_dcu_force(&self) -> DbgDcuForceR {
        DbgDcuForceR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_dbgenin(&mut self) -> Sse200DbgeninW<DbgCtrlSpec> {
        Sse200DbgeninW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_nidenin(&mut self) -> Sse200NideninW<DbgCtrlSpec> {
        Sse200NideninW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spidenin(&mut self) -> Sse200SpideninW<DbgCtrlSpec> {
        Sse200SpideninW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spnidenin(&mut self) -> Sse200SpnideninW<DbgCtrlSpec> {
        Sse200SpnideninW::new(self, 3)
    }
    #[doc = "Bit 7 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel0(&mut self) -> Todbgensel0W<DbgCtrlSpec> {
        Todbgensel0W::new(self, 7)
    }
    #[doc = "Bit 8 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel1(&mut self) -> Todbgensel1W<DbgCtrlSpec> {
        Todbgensel1W::new(self, 8)
    }
    #[doc = "Bits 30:31 - SSE-200 debug ports control"]
    #[inline(always)]
    pub fn dbg_dcu_force(&mut self) -> DbgDcuForceW<DbgCtrlSpec> {
        DbgDcuForceW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgCtrlSpec;
impl crate::RegisterSpec for DbgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_ctrl::R`](R) reader structure"]
impl crate::Readable for DbgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_ctrl::W`](W) writer structure"]
impl crate::Writable for DbgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CTRL to value 0x1f"]
impl crate::Resettable for DbgCtrlSpec {
    const RESET_VALUE: u32 = 0x1f;
}
