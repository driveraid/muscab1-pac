#[doc = "Register `INTR_CTRL` reader"]
pub type R = crate::R<IntrCtrlSpec>;
#[doc = "Register `INTR_CTRL` writer"]
pub type W = crate::W<IntrCtrlSpec>;
#[doc = "Field `QSPI_MPC_CFG_INIT_VALUE` reader - 0: Secure mode 1: Non-secure mode"]
pub type QspiMpcCfgInitValueR = crate::BitReader;
#[doc = "Field `QSPI_MPC_CFG_INIT_VALUE` writer - 0: Secure mode 1: Non-secure mode"]
pub type QspiMpcCfgInitValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_MPC_CFG_INIT_VALUE` reader - 0: Secure mode 1: Non-secure mode"]
pub type SramMpcCfgInitValueR = crate::BitReader;
#[doc = "Field `SRAM_MPC_CFG_INIT_VALUE` writer - 0: Secure mode 1: Non-secure mode"]
pub type SramMpcCfgInitValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ_MPC_CFG_INIT_VALUE` reader - 0: Secure mode 1: Non-secure mode"]
pub type AzMpcCfgInitValueR = crate::BitReader;
#[doc = "Field `AZ_MPC_CFG_INIT_VALUE` writer - 0: Secure mode 1: Non-secure mode"]
pub type AzMpcCfgInitValueW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn qspi_mpc_cfg_init_value(&self) -> QspiMpcCfgInitValueR {
        QspiMpcCfgInitValueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn sram_mpc_cfg_init_value(&self) -> SramMpcCfgInitValueR {
        SramMpcCfgInitValueR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn az_mpc_cfg_init_value(&self) -> AzMpcCfgInitValueR {
        AzMpcCfgInitValueR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn qspi_mpc_cfg_init_value(&mut self) -> QspiMpcCfgInitValueW<IntrCtrlSpec> {
        QspiMpcCfgInitValueW::new(self, 3)
    }
    #[doc = "Bit 5 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn sram_mpc_cfg_init_value(&mut self) -> SramMpcCfgInitValueW<IntrCtrlSpec> {
        SramMpcCfgInitValueW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn az_mpc_cfg_init_value(&mut self) -> AzMpcCfgInitValueW<IntrCtrlSpec> {
        AzMpcCfgInitValueW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCtrlSpec;
impl crate::RegisterSpec for IntrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_ctrl::R`](R) reader structure"]
impl crate::Readable for IntrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_ctrl::W`](W) writer structure"]
impl crate::Writable for IntrCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_CTRL to value 0"]
impl crate::Resettable for IntrCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
