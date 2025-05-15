#[doc = "Register `SELECTION_CONTROL_REG` reader"]
pub type R = crate::R<SelectionControlRegSpec>;
#[doc = "Register `SELECTION_CONTROL_REG` writer"]
pub type W = crate::W<SelectionControlRegSpec>;
#[doc = "Field `clock_phase_shifter_select` reader - QSPI input clock phase shift control"]
pub type ClockPhaseShifterSelectR = crate::FieldReader;
#[doc = "Field `clock_phase_shifter_select` writer - QSPI input clock phase shift control"]
pub type ClockPhaseShifterSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clock_phase_shifter_bypass` reader - QSPI input clock phase shift control"]
pub type ClockPhaseShifterBypassR = crate::BitReader;
#[doc = "Field `clock_phase_shifter_bypass` writer - QSPI input clock phase shift control"]
pub type ClockPhaseShifterBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_mask_delay` reader - SDIO mask delay"]
pub type SdioMaskDelayR = crate::FieldReader;
#[doc = "Field `sdio_mask_delay` writer - SDIO mask delay"]
pub type SdioMaskDelayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_select(&self) -> ClockPhaseShifterSelectR {
        ClockPhaseShifterSelectR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_bypass(&self) -> ClockPhaseShifterBypassR {
        ClockPhaseShifterBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SDIO mask delay"]
    #[inline(always)]
    pub fn sdio_mask_delay(&self) -> SdioMaskDelayR {
        SdioMaskDelayR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_select(
        &mut self,
    ) -> ClockPhaseShifterSelectW<SelectionControlRegSpec> {
        ClockPhaseShifterSelectW::new(self, 0)
    }
    #[doc = "Bit 2 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_bypass(
        &mut self,
    ) -> ClockPhaseShifterBypassW<SelectionControlRegSpec> {
        ClockPhaseShifterBypassW::new(self, 2)
    }
    #[doc = "Bits 8:9 - SDIO mask delay"]
    #[inline(always)]
    pub fn sdio_mask_delay(&mut self) -> SdioMaskDelayW<SelectionControlRegSpec> {
        SdioMaskDelayW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`selection_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selection_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelectionControlRegSpec;
impl crate::RegisterSpec for SelectionControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`selection_control_reg::R`](R) reader structure"]
impl crate::Readable for SelectionControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`selection_control_reg::W`](W) writer structure"]
impl crate::Writable for SelectionControlRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SELECTION_CONTROL_REG to value 0x0100_0200"]
impl crate::Resettable for SelectionControlRegSpec {
    const RESET_VALUE: u32 = 0x0100_0200;
}
