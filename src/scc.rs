#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_ctrl_sel: ClkCtrlSel,
    clk_pll_prediv_ctrl: ClkPllPredivCtrl,
    _reserved2: [u8; 0x04],
    clk_postdiv_ctrl_flash: ClkPostdivCtrlFlash,
    clk_postdiv_ctrl_qspi: ClkPostdivCtrlQspi,
    clk_postdiv_ctrl_rtc: ClkPostdivCtrlRtc,
    clk_postdiv_ctrl_sd: ClkPostdivCtrlSd,
    clk_postdiv_ctrl_test: ClkPostdivCtrlTest,
    ctrl_bypass_div: CtrlBypassDiv,
    pll_ctrl_pll0_clk: PllCtrlPll0Clk,
    pll_postdiv_ctrl_pll0_clk: PllPostdivCtrlPll0Clk,
    pll_ctrl_mult_pll0_clk: PllCtrlMultPll0Clk,
    clk_ctrl_enable: ClkCtrlEnable,
    clk_status: ClkStatus,
    _reserved13: [u8; 0x08],
    reset_ctrl: ResetCtrl,
    _reserved14: [u8; 0x04],
    dbg_ctrl: DbgCtrl,
    sram_ctrl: SramCtrl,
    intr_ctrl: IntrCtrl,
    clk_test_ctrl: ClkTestCtrl,
    cpu0_vtor: Cpu0Vtor,
    _reserved19: [u8; 0x04],
    cpu1_vtor: Cpu1Vtor,
    az_cpu_vtor: AzCpuVtor,
    iomux_main_insel_0: IomuxMainInsel0,
    iomux_main_insel_1: IomuxMainInsel1,
    iomux_main_outsel_0: IomuxMainOutsel0,
    iomux_main_outsel_1: IomuxMainOutsel1,
    iomux_main_oensel_0: IomuxMainOensel0,
    iomux_main_oensel_1: IomuxMainOensel1,
    iomux_main_default_in_0: IomuxMainDefaultIn0,
    iomux_main_default_in_1: IomuxMainDefaultIn1,
    iomux_altf1_insel_0: IomuxAltf1Insel0,
    iomux_altf1_insel_1: IomuxAltf1Insel1,
    iomux_altf1_outsel_0: IomuxAltf1Outsel0,
    iomux_altf1_outsel_1: IomuxAltf1Outsel1,
    iomux_altf1_oensel_0: IomuxAltf1Oensel0,
    iomux_altf1_oensel_1: IomuxAltf1Oensel1,
    iomux_altf1_default_in_0: IomuxAltf1DefaultIn0,
    iomux_altf1_default_in_1: IomuxAltf1DefaultIn1,
    iomux_altf2_insel_0: IomuxAltf2Insel0,
    iomux_altf2_insel_1: IomuxAltf2Insel1,
    iomux_altf2_outsel_0: IomuxAltf2Outsel0,
    iomux_altf2_outsel_1: IomuxAltf2Outsel1,
    iomux_altf2_oensel_0: IomuxAltf2Oensel0,
    iomux_altf2_oensel_1: IomuxAltf2Oensel1,
    iomux_altf2_default_in_0: IomuxAltf2DefaultIn0,
    iomux_altf2_default_in_1: IomuxAltf2DefaultIn1,
    _reserved45: [u8; 0x20],
    iopad_dso_0: IopadDso0,
    iopad_dso_1: IopadDso1,
    iopad_ds1_0: IopadDs1_0,
    iopad_ds1_1: IopadDs1_1,
    iopad_pe_0: IopadPe0,
    iopad_pe_1: IopadPe1,
    iopad_ps_0: IopadPs0,
    iopad_ps_1: IopadPs1,
    iopad_sr_0: IopadSr0,
    iopad_sr_1: IopadSr1,
    iopad_is_0: IopadIs0,
    iopad_is_1: IopadIs1,
    pvt_ctrl: PvtCtrl,
    _reserved58: [u8; 0x14],
    spare0: Spare0,
    _reserved59: [u8; 0x08],
    static_conf_sig1: StaticConfSig1,
    _reserved60: [u8; 0x60],
    flash_din_0: FlashDin0,
    flash_din_1: FlashDin1,
    flash_din_2: FlashDin2,
    flash_din_3: FlashDin3,
    _reserved64: [u8; 0x10],
    flash0_dout_0: Flash0Dout0,
    flash0_dout_1: Flash0Dout1,
    flash0_dout_2: Flash0Dout2,
    flash0_dout_3: Flash0Dout3,
    flash1_dout_0: Flash1Dout0,
    flash1_dout_1: Flash1Dout1,
    flash1_dout_2: Flash1Dout2,
    flash1_dout_3: Flash1Dout3,
    selection_control_reg: SelectionControlReg,
    az_rom_remap_mask: AzRomRemapMask,
    az_rom_remap_offset: AzRomRemapOffset,
    az_code_remap_mask: AzCodeRemapMask,
    az_code_remap_offset: AzCodeRemapOffset,
    az_sys_remap_mask: AzSysRemapMask,
    az_sys_remap_offset: AzSysRemapOffset,
    _reserved79: [u8; 0x04],
    az_ctrl: AzCtrl,
    _reserved80: [u8; 0x04],
    sse_otp_rd_data: SseOtpRdData,
    _reserved81: [u8; 0x04],
    az_otp_rd_data: AzOtpRdData,
    _reserved82: [u8; 0x08],
    spare_ctrl0: SpareCtrl0,
    spare_ctrl1: SpareCtrl1,
    _reserved84: [u8; 0x01dc],
    chip_id: ChipId,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn clk_ctrl_sel(&self) -> &ClkCtrlSel {
        &self.clk_ctrl_sel
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn clk_pll_prediv_ctrl(&self) -> &ClkPllPredivCtrl {
        &self.clk_pll_prediv_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn clk_postdiv_ctrl_flash(&self) -> &ClkPostdivCtrlFlash {
        &self.clk_postdiv_ctrl_flash
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn clk_postdiv_ctrl_qspi(&self) -> &ClkPostdivCtrlQspi {
        &self.clk_postdiv_ctrl_qspi
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn clk_postdiv_ctrl_rtc(&self) -> &ClkPostdivCtrlRtc {
        &self.clk_postdiv_ctrl_rtc
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn clk_postdiv_ctrl_sd(&self) -> &ClkPostdivCtrlSd {
        &self.clk_postdiv_ctrl_sd
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn clk_postdiv_ctrl_test(&self) -> &ClkPostdivCtrlTest {
        &self.clk_postdiv_ctrl_test
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn ctrl_bypass_div(&self) -> &CtrlBypassDiv {
        &self.ctrl_bypass_div
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn pll_ctrl_pll0_clk(&self) -> &PllCtrlPll0Clk {
        &self.pll_ctrl_pll0_clk
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn pll_postdiv_ctrl_pll0_clk(&self) -> &PllPostdivCtrlPll0Clk {
        &self.pll_postdiv_ctrl_pll0_clk
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn pll_ctrl_mult_pll0_clk(&self) -> &PllCtrlMultPll0Clk {
        &self.pll_ctrl_mult_pll0_clk
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn clk_ctrl_enable(&self) -> &ClkCtrlEnable {
        &self.clk_ctrl_enable
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn clk_status(&self) -> &ClkStatus {
        &self.clk_status
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn reset_ctrl(&self) -> &ResetCtrl {
        &self.reset_ctrl
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn dbg_ctrl(&self) -> &DbgCtrl {
        &self.dbg_ctrl
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn sram_ctrl(&self) -> &SramCtrl {
        &self.sram_ctrl
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn intr_ctrl(&self) -> &IntrCtrl {
        &self.intr_ctrl
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn clk_test_ctrl(&self) -> &ClkTestCtrl {
        &self.clk_test_ctrl
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn cpu0_vtor(&self) -> &Cpu0Vtor {
        &self.cpu0_vtor
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn cpu1_vtor(&self) -> &Cpu1Vtor {
        &self.cpu1_vtor
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn az_cpu_vtor(&self) -> &AzCpuVtor {
        &self.az_cpu_vtor
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn iomux_main_insel_0(&self) -> &IomuxMainInsel0 {
        &self.iomux_main_insel_0
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn iomux_main_insel_1(&self) -> &IomuxMainInsel1 {
        &self.iomux_main_insel_1
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn iomux_main_outsel_0(&self) -> &IomuxMainOutsel0 {
        &self.iomux_main_outsel_0
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn iomux_main_outsel_1(&self) -> &IomuxMainOutsel1 {
        &self.iomux_main_outsel_1
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn iomux_main_oensel_0(&self) -> &IomuxMainOensel0 {
        &self.iomux_main_oensel_0
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn iomux_main_oensel_1(&self) -> &IomuxMainOensel1 {
        &self.iomux_main_oensel_1
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn iomux_main_default_in_0(&self) -> &IomuxMainDefaultIn0 {
        &self.iomux_main_default_in_0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn iomux_main_default_in_1(&self) -> &IomuxMainDefaultIn1 {
        &self.iomux_main_default_in_1
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn iomux_altf1_insel_0(&self) -> &IomuxAltf1Insel0 {
        &self.iomux_altf1_insel_0
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn iomux_altf1_insel_1(&self) -> &IomuxAltf1Insel1 {
        &self.iomux_altf1_insel_1
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn iomux_altf1_outsel_0(&self) -> &IomuxAltf1Outsel0 {
        &self.iomux_altf1_outsel_0
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn iomux_altf1_outsel_1(&self) -> &IomuxAltf1Outsel1 {
        &self.iomux_altf1_outsel_1
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn iomux_altf1_oensel_0(&self) -> &IomuxAltf1Oensel0 {
        &self.iomux_altf1_oensel_0
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn iomux_altf1_oensel_1(&self) -> &IomuxAltf1Oensel1 {
        &self.iomux_altf1_oensel_1
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn iomux_altf1_default_in_0(&self) -> &IomuxAltf1DefaultIn0 {
        &self.iomux_altf1_default_in_0
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn iomux_altf1_default_in_1(&self) -> &IomuxAltf1DefaultIn1 {
        &self.iomux_altf1_default_in_1
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn iomux_altf2_insel_0(&self) -> &IomuxAltf2Insel0 {
        &self.iomux_altf2_insel_0
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn iomux_altf2_insel_1(&self) -> &IomuxAltf2Insel1 {
        &self.iomux_altf2_insel_1
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn iomux_altf2_outsel_0(&self) -> &IomuxAltf2Outsel0 {
        &self.iomux_altf2_outsel_0
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn iomux_altf2_outsel_1(&self) -> &IomuxAltf2Outsel1 {
        &self.iomux_altf2_outsel_1
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn iomux_altf2_oensel_0(&self) -> &IomuxAltf2Oensel0 {
        &self.iomux_altf2_oensel_0
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn iomux_altf2_oensel_1(&self) -> &IomuxAltf2Oensel1 {
        &self.iomux_altf2_oensel_1
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn iomux_altf2_default_in_0(&self) -> &IomuxAltf2DefaultIn0 {
        &self.iomux_altf2_default_in_0
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn iomux_altf2_default_in_1(&self) -> &IomuxAltf2DefaultIn1 {
        &self.iomux_altf2_default_in_1
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn iopad_dso_0(&self) -> &IopadDso0 {
        &self.iopad_dso_0
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn iopad_dso_1(&self) -> &IopadDso1 {
        &self.iopad_dso_1
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn iopad_ds1_0(&self) -> &IopadDs1_0 {
        &self.iopad_ds1_0
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn iopad_ds1_1(&self) -> &IopadDs1_1 {
        &self.iopad_ds1_1
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn iopad_pe_0(&self) -> &IopadPe0 {
        &self.iopad_pe_0
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn iopad_pe_1(&self) -> &IopadPe1 {
        &self.iopad_pe_1
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn iopad_ps_0(&self) -> &IopadPs0 {
        &self.iopad_ps_0
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn iopad_ps_1(&self) -> &IopadPs1 {
        &self.iopad_ps_1
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn iopad_sr_0(&self) -> &IopadSr0 {
        &self.iopad_sr_0
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn iopad_sr_1(&self) -> &IopadSr1 {
        &self.iopad_sr_1
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn iopad_is_0(&self) -> &IopadIs0 {
        &self.iopad_is_0
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn iopad_is_1(&self) -> &IopadIs1 {
        &self.iopad_is_1
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn pvt_ctrl(&self) -> &PvtCtrl {
        &self.pvt_ctrl
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn spare0(&self) -> &Spare0 {
        &self.spare0
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn static_conf_sig1(&self) -> &StaticConfSig1 {
        &self.static_conf_sig1
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn flash_din_0(&self) -> &FlashDin0 {
        &self.flash_din_0
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn flash_din_1(&self) -> &FlashDin1 {
        &self.flash_din_1
    }
    #[doc = "0x1a8 - "]
    #[inline(always)]
    pub const fn flash_din_2(&self) -> &FlashDin2 {
        &self.flash_din_2
    }
    #[doc = "0x1ac - "]
    #[inline(always)]
    pub const fn flash_din_3(&self) -> &FlashDin3 {
        &self.flash_din_3
    }
    #[doc = "0x1c0 - "]
    #[inline(always)]
    pub const fn flash0_dout_0(&self) -> &Flash0Dout0 {
        &self.flash0_dout_0
    }
    #[doc = "0x1c4 - "]
    #[inline(always)]
    pub const fn flash0_dout_1(&self) -> &Flash0Dout1 {
        &self.flash0_dout_1
    }
    #[doc = "0x1c8 - "]
    #[inline(always)]
    pub const fn flash0_dout_2(&self) -> &Flash0Dout2 {
        &self.flash0_dout_2
    }
    #[doc = "0x1cc - "]
    #[inline(always)]
    pub const fn flash0_dout_3(&self) -> &Flash0Dout3 {
        &self.flash0_dout_3
    }
    #[doc = "0x1d0 - "]
    #[inline(always)]
    pub const fn flash1_dout_0(&self) -> &Flash1Dout0 {
        &self.flash1_dout_0
    }
    #[doc = "0x1d4 - "]
    #[inline(always)]
    pub const fn flash1_dout_1(&self) -> &Flash1Dout1 {
        &self.flash1_dout_1
    }
    #[doc = "0x1d8 - "]
    #[inline(always)]
    pub const fn flash1_dout_2(&self) -> &Flash1Dout2 {
        &self.flash1_dout_2
    }
    #[doc = "0x1dc - "]
    #[inline(always)]
    pub const fn flash1_dout_3(&self) -> &Flash1Dout3 {
        &self.flash1_dout_3
    }
    #[doc = "0x1e0 - "]
    #[inline(always)]
    pub const fn selection_control_reg(&self) -> &SelectionControlReg {
        &self.selection_control_reg
    }
    #[doc = "0x1e4 - "]
    #[inline(always)]
    pub const fn az_rom_remap_mask(&self) -> &AzRomRemapMask {
        &self.az_rom_remap_mask
    }
    #[doc = "0x1e8 - "]
    #[inline(always)]
    pub const fn az_rom_remap_offset(&self) -> &AzRomRemapOffset {
        &self.az_rom_remap_offset
    }
    #[doc = "0x1ec - "]
    #[inline(always)]
    pub const fn az_code_remap_mask(&self) -> &AzCodeRemapMask {
        &self.az_code_remap_mask
    }
    #[doc = "0x1f0 - "]
    #[inline(always)]
    pub const fn az_code_remap_offset(&self) -> &AzCodeRemapOffset {
        &self.az_code_remap_offset
    }
    #[doc = "0x1f4 - "]
    #[inline(always)]
    pub const fn az_sys_remap_mask(&self) -> &AzSysRemapMask {
        &self.az_sys_remap_mask
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn az_sys_remap_offset(&self) -> &AzSysRemapOffset {
        &self.az_sys_remap_offset
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn az_ctrl(&self) -> &AzCtrl {
        &self.az_ctrl
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn sse_otp_rd_data(&self) -> &SseOtpRdData {
        &self.sse_otp_rd_data
    }
    #[doc = "0x210 - "]
    #[inline(always)]
    pub const fn az_otp_rd_data(&self) -> &AzOtpRdData {
        &self.az_otp_rd_data
    }
    #[doc = "0x21c - "]
    #[inline(always)]
    pub const fn spare_ctrl0(&self) -> &SpareCtrl0 {
        &self.spare_ctrl0
    }
    #[doc = "0x220 - "]
    #[inline(always)]
    pub const fn spare_ctrl1(&self) -> &SpareCtrl1 {
        &self.spare_ctrl1
    }
    #[doc = "0x400 - "]
    #[inline(always)]
    pub const fn chip_id(&self) -> &ChipId {
        &self.chip_id
    }
}
#[doc = "CLK_CTRL_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ctrl_sel`] module"]
#[doc(alias = "CLK_CTRL_SEL")]
pub type ClkCtrlSel = crate::Reg<clk_ctrl_sel::ClkCtrlSelSpec>;
#[doc = ""]
pub mod clk_ctrl_sel;
#[doc = "CLK_PLL_PREDIV_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pll_prediv_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pll_prediv_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll_prediv_ctrl`] module"]
#[doc(alias = "CLK_PLL_PREDIV_CTRL")]
pub type ClkPllPredivCtrl = crate::Reg<clk_pll_prediv_ctrl::ClkPllPredivCtrlSpec>;
#[doc = ""]
pub mod clk_pll_prediv_ctrl;
#[doc = "CLK_POSTDIV_CTRL_FLASH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_flash::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_flash::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_postdiv_ctrl_flash`] module"]
#[doc(alias = "CLK_POSTDIV_CTRL_FLASH")]
pub type ClkPostdivCtrlFlash = crate::Reg<clk_postdiv_ctrl_flash::ClkPostdivCtrlFlashSpec>;
#[doc = ""]
pub mod clk_postdiv_ctrl_flash;
#[doc = "CLK_POSTDIV_CTRL_QSPI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_qspi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_qspi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_postdiv_ctrl_qspi`] module"]
#[doc(alias = "CLK_POSTDIV_CTRL_QSPI")]
pub type ClkPostdivCtrlQspi = crate::Reg<clk_postdiv_ctrl_qspi::ClkPostdivCtrlQspiSpec>;
#[doc = ""]
pub mod clk_postdiv_ctrl_qspi;
#[doc = "CLK_POSTDIV_CTRL_RTC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_rtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_rtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_postdiv_ctrl_rtc`] module"]
#[doc(alias = "CLK_POSTDIV_CTRL_RTC")]
pub type ClkPostdivCtrlRtc = crate::Reg<clk_postdiv_ctrl_rtc::ClkPostdivCtrlRtcSpec>;
#[doc = ""]
pub mod clk_postdiv_ctrl_rtc;
#[doc = "CLK_POSTDIV_CTRL_SD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_sd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_sd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_postdiv_ctrl_sd`] module"]
#[doc(alias = "CLK_POSTDIV_CTRL_SD")]
pub type ClkPostdivCtrlSd = crate::Reg<clk_postdiv_ctrl_sd::ClkPostdivCtrlSdSpec>;
#[doc = ""]
pub mod clk_postdiv_ctrl_sd;
#[doc = "CLK_POSTDIV_CTRL_TEST (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_postdiv_ctrl_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_postdiv_ctrl_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_postdiv_ctrl_test`] module"]
#[doc(alias = "CLK_POSTDIV_CTRL_TEST")]
pub type ClkPostdivCtrlTest = crate::Reg<clk_postdiv_ctrl_test::ClkPostdivCtrlTestSpec>;
#[doc = ""]
pub mod clk_postdiv_ctrl_test;
#[doc = "CTRL_BYPASS_DIV (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_bypass_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_bypass_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_bypass_div`] module"]
#[doc(alias = "CTRL_BYPASS_DIV")]
pub type CtrlBypassDiv = crate::Reg<ctrl_bypass_div::CtrlBypassDivSpec>;
#[doc = ""]
pub mod ctrl_bypass_div;
#[doc = "PLL_CTRL_PLL0_CLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ctrl_pll0_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ctrl_pll0_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ctrl_pll0_clk`] module"]
#[doc(alias = "PLL_CTRL_PLL0_CLK")]
pub type PllCtrlPll0Clk = crate::Reg<pll_ctrl_pll0_clk::PllCtrlPll0ClkSpec>;
#[doc = ""]
pub mod pll_ctrl_pll0_clk;
#[doc = "PLL_POSTDIV_CTRL_PLL0_CLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pll_postdiv_ctrl_pll0_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_postdiv_ctrl_pll0_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_postdiv_ctrl_pll0_clk`] module"]
#[doc(alias = "PLL_POSTDIV_CTRL_PLL0_CLK")]
pub type PllPostdivCtrlPll0Clk = crate::Reg<pll_postdiv_ctrl_pll0_clk::PllPostdivCtrlPll0ClkSpec>;
#[doc = ""]
pub mod pll_postdiv_ctrl_pll0_clk;
#[doc = "PLL_CTRL_MULT_PLL0_CLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ctrl_mult_pll0_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ctrl_mult_pll0_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ctrl_mult_pll0_clk`] module"]
#[doc(alias = "PLL_CTRL_MULT_PLL0_CLK")]
pub type PllCtrlMultPll0Clk = crate::Reg<pll_ctrl_mult_pll0_clk::PllCtrlMultPll0ClkSpec>;
#[doc = ""]
pub mod pll_ctrl_mult_pll0_clk;
#[doc = "CLK_CTRL_ENABLE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ctrl_enable`] module"]
#[doc(alias = "CLK_CTRL_ENABLE")]
pub type ClkCtrlEnable = crate::Reg<clk_ctrl_enable::ClkCtrlEnableSpec>;
#[doc = ""]
pub mod clk_ctrl_enable;
#[doc = "CLK_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_status`] module"]
#[doc(alias = "CLK_STATUS")]
pub type ClkStatus = crate::Reg<clk_status::ClkStatusSpec>;
#[doc = ""]
pub mod clk_status;
#[doc = "RESET_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reset_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_ctrl`] module"]
#[doc(alias = "RESET_CTRL")]
pub type ResetCtrl = crate::Reg<reset_ctrl::ResetCtrlSpec>;
#[doc = ""]
pub mod reset_ctrl;
#[doc = "DBG_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_ctrl`] module"]
#[doc(alias = "DBG_CTRL")]
pub type DbgCtrl = crate::Reg<dbg_ctrl::DbgCtrlSpec>;
#[doc = ""]
pub mod dbg_ctrl;
#[doc = "SRAM_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ctrl`] module"]
#[doc(alias = "SRAM_CTRL")]
pub type SramCtrl = crate::Reg<sram_ctrl::SramCtrlSpec>;
#[doc = ""]
pub mod sram_ctrl;
#[doc = "INTR_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`intr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_ctrl`] module"]
#[doc(alias = "INTR_CTRL")]
pub type IntrCtrl = crate::Reg<intr_ctrl::IntrCtrlSpec>;
#[doc = ""]
pub mod intr_ctrl;
#[doc = "CLK_TEST_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_test_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_test_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_test_ctrl`] module"]
#[doc(alias = "CLK_TEST_CTRL")]
pub type ClkTestCtrl = crate::Reg<clk_test_ctrl::ClkTestCtrlSpec>;
#[doc = ""]
pub mod clk_test_ctrl;
#[doc = "CPU0_VTOR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_vtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_vtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_vtor`] module"]
#[doc(alias = "CPU0_VTOR")]
pub type Cpu0Vtor = crate::Reg<cpu0_vtor::Cpu0VtorSpec>;
#[doc = ""]
pub mod cpu0_vtor;
#[doc = "CPU1_VTOR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_vtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_vtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_vtor`] module"]
#[doc(alias = "CPU1_VTOR")]
pub type Cpu1Vtor = crate::Reg<cpu1_vtor::Cpu1VtorSpec>;
#[doc = ""]
pub mod cpu1_vtor;
#[doc = "AZ_CPU_VTOR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_cpu_vtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_cpu_vtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_cpu_vtor`] module"]
#[doc(alias = "AZ_CPU_VTOR")]
pub type AzCpuVtor = crate::Reg<az_cpu_vtor::AzCpuVtorSpec>;
#[doc = ""]
pub mod az_cpu_vtor;
#[doc = "IOMUX_MAIN_INSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_insel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_insel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_insel_0`] module"]
#[doc(alias = "IOMUX_MAIN_INSEL_0")]
pub type IomuxMainInsel0 = crate::Reg<iomux_main_insel_0::IomuxMainInsel0Spec>;
#[doc = ""]
pub mod iomux_main_insel_0;
#[doc = "IOMUX_MAIN_INSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_insel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_insel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_insel_1`] module"]
#[doc(alias = "IOMUX_MAIN_INSEL_1")]
pub type IomuxMainInsel1 = crate::Reg<iomux_main_insel_1::IomuxMainInsel1Spec>;
#[doc = ""]
pub mod iomux_main_insel_1;
#[doc = "IOMUX_MAIN_OUTSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_outsel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_outsel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_outsel_0`] module"]
#[doc(alias = "IOMUX_MAIN_OUTSEL_0")]
pub type IomuxMainOutsel0 = crate::Reg<iomux_main_outsel_0::IomuxMainOutsel0Spec>;
#[doc = ""]
pub mod iomux_main_outsel_0;
#[doc = "IOMUX_MAIN_OUTSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_outsel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_outsel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_outsel_1`] module"]
#[doc(alias = "IOMUX_MAIN_OUTSEL_1")]
pub type IomuxMainOutsel1 = crate::Reg<iomux_main_outsel_1::IomuxMainOutsel1Spec>;
#[doc = ""]
pub mod iomux_main_outsel_1;
#[doc = "IOMUX_MAIN_OENSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_oensel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_oensel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_oensel_0`] module"]
#[doc(alias = "IOMUX_MAIN_OENSEL_0")]
pub type IomuxMainOensel0 = crate::Reg<iomux_main_oensel_0::IomuxMainOensel0Spec>;
#[doc = ""]
pub mod iomux_main_oensel_0;
#[doc = "IOMUX_MAIN_OENSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_oensel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_oensel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_oensel_1`] module"]
#[doc(alias = "IOMUX_MAIN_OENSEL_1")]
pub type IomuxMainOensel1 = crate::Reg<iomux_main_oensel_1::IomuxMainOensel1Spec>;
#[doc = ""]
pub mod iomux_main_oensel_1;
#[doc = "IOMUX_MAIN_DEFAULT_IN_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_default_in_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_default_in_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_default_in_0`] module"]
#[doc(alias = "IOMUX_MAIN_DEFAULT_IN_0")]
pub type IomuxMainDefaultIn0 = crate::Reg<iomux_main_default_in_0::IomuxMainDefaultIn0Spec>;
#[doc = ""]
pub mod iomux_main_default_in_0;
#[doc = "IOMUX_MAIN_DEFAULT_IN_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_main_default_in_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_main_default_in_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_main_default_in_1`] module"]
#[doc(alias = "IOMUX_MAIN_DEFAULT_IN_1")]
pub type IomuxMainDefaultIn1 = crate::Reg<iomux_main_default_in_1::IomuxMainDefaultIn1Spec>;
#[doc = ""]
pub mod iomux_main_default_in_1;
#[doc = "IOMUX_ALTF1_INSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_insel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_insel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_insel_0`] module"]
#[doc(alias = "IOMUX_ALTF1_INSEL_0")]
pub type IomuxAltf1Insel0 = crate::Reg<iomux_altf1_insel_0::IomuxAltf1Insel0Spec>;
#[doc = ""]
pub mod iomux_altf1_insel_0;
#[doc = "IOMUX_ALTF1_INSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_insel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_insel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_insel_1`] module"]
#[doc(alias = "IOMUX_ALTF1_INSEL_1")]
pub type IomuxAltf1Insel1 = crate::Reg<iomux_altf1_insel_1::IomuxAltf1Insel1Spec>;
#[doc = ""]
pub mod iomux_altf1_insel_1;
#[doc = "IOMUX_ALTF1_OUTSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_outsel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_outsel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_outsel_0`] module"]
#[doc(alias = "IOMUX_ALTF1_OUTSEL_0")]
pub type IomuxAltf1Outsel0 = crate::Reg<iomux_altf1_outsel_0::IomuxAltf1Outsel0Spec>;
#[doc = ""]
pub mod iomux_altf1_outsel_0;
#[doc = "IOMUX_ALTF1_OUTSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_outsel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_outsel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_outsel_1`] module"]
#[doc(alias = "IOMUX_ALTF1_OUTSEL_1")]
pub type IomuxAltf1Outsel1 = crate::Reg<iomux_altf1_outsel_1::IomuxAltf1Outsel1Spec>;
#[doc = ""]
pub mod iomux_altf1_outsel_1;
#[doc = "IOMUX_ALTF1_OENSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_oensel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_oensel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_oensel_0`] module"]
#[doc(alias = "IOMUX_ALTF1_OENSEL_0")]
pub type IomuxAltf1Oensel0 = crate::Reg<iomux_altf1_oensel_0::IomuxAltf1Oensel0Spec>;
#[doc = ""]
pub mod iomux_altf1_oensel_0;
#[doc = "IOMUX_ALTF1_OENSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_oensel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_oensel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_oensel_1`] module"]
#[doc(alias = "IOMUX_ALTF1_OENSEL_1")]
pub type IomuxAltf1Oensel1 = crate::Reg<iomux_altf1_oensel_1::IomuxAltf1Oensel1Spec>;
#[doc = ""]
pub mod iomux_altf1_oensel_1;
#[doc = "IOMUX_ALTF1_DEFAULT_IN_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_default_in_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_default_in_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_default_in_0`] module"]
#[doc(alias = "IOMUX_ALTF1_DEFAULT_IN_0")]
pub type IomuxAltf1DefaultIn0 = crate::Reg<iomux_altf1_default_in_0::IomuxAltf1DefaultIn0Spec>;
#[doc = ""]
pub mod iomux_altf1_default_in_0;
#[doc = "IOMUX_ALTF1_DEFAULT_IN_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf1_default_in_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf1_default_in_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf1_default_in_1`] module"]
#[doc(alias = "IOMUX_ALTF1_DEFAULT_IN_1")]
pub type IomuxAltf1DefaultIn1 = crate::Reg<iomux_altf1_default_in_1::IomuxAltf1DefaultIn1Spec>;
#[doc = ""]
pub mod iomux_altf1_default_in_1;
#[doc = "IOMUX_ALTF2_INSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_insel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_insel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_insel_0`] module"]
#[doc(alias = "IOMUX_ALTF2_INSEL_0")]
pub type IomuxAltf2Insel0 = crate::Reg<iomux_altf2_insel_0::IomuxAltf2Insel0Spec>;
#[doc = ""]
pub mod iomux_altf2_insel_0;
#[doc = "IOMUX_ALTF2_INSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_insel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_insel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_insel_1`] module"]
#[doc(alias = "IOMUX_ALTF2_INSEL_1")]
pub type IomuxAltf2Insel1 = crate::Reg<iomux_altf2_insel_1::IomuxAltf2Insel1Spec>;
#[doc = ""]
pub mod iomux_altf2_insel_1;
#[doc = "IOMUX_ALTF2_OUTSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_outsel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_outsel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_outsel_0`] module"]
#[doc(alias = "IOMUX_ALTF2_OUTSEL_0")]
pub type IomuxAltf2Outsel0 = crate::Reg<iomux_altf2_outsel_0::IomuxAltf2Outsel0Spec>;
#[doc = ""]
pub mod iomux_altf2_outsel_0;
#[doc = "IOMUX_ALTF2_OUTSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_outsel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_outsel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_outsel_1`] module"]
#[doc(alias = "IOMUX_ALTF2_OUTSEL_1")]
pub type IomuxAltf2Outsel1 = crate::Reg<iomux_altf2_outsel_1::IomuxAltf2Outsel1Spec>;
#[doc = ""]
pub mod iomux_altf2_outsel_1;
#[doc = "IOMUX_ALTF2_OENSEL_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_oensel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_oensel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_oensel_0`] module"]
#[doc(alias = "IOMUX_ALTF2_OENSEL_0")]
pub type IomuxAltf2Oensel0 = crate::Reg<iomux_altf2_oensel_0::IomuxAltf2Oensel0Spec>;
#[doc = ""]
pub mod iomux_altf2_oensel_0;
#[doc = "IOMUX_ALTF2_OENSEL_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_oensel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_oensel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_oensel_1`] module"]
#[doc(alias = "IOMUX_ALTF2_OENSEL_1")]
pub type IomuxAltf2Oensel1 = crate::Reg<iomux_altf2_oensel_1::IomuxAltf2Oensel1Spec>;
#[doc = ""]
pub mod iomux_altf2_oensel_1;
#[doc = "IOMUX_ALTF2_DEFAULT_IN_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_default_in_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_default_in_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_default_in_0`] module"]
#[doc(alias = "IOMUX_ALTF2_DEFAULT_IN_0")]
pub type IomuxAltf2DefaultIn0 = crate::Reg<iomux_altf2_default_in_0::IomuxAltf2DefaultIn0Spec>;
#[doc = ""]
pub mod iomux_altf2_default_in_0;
#[doc = "IOMUX_ALTF2_DEFAULT_IN_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_altf2_default_in_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_altf2_default_in_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_altf2_default_in_1`] module"]
#[doc(alias = "IOMUX_ALTF2_DEFAULT_IN_1")]
pub type IomuxAltf2DefaultIn1 = crate::Reg<iomux_altf2_default_in_1::IomuxAltf2DefaultIn1Spec>;
#[doc = ""]
pub mod iomux_altf2_default_in_1;
#[doc = "IOPAD_DSO_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_dso_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_dso_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_dso_0`] module"]
#[doc(alias = "IOPAD_DSO_0")]
pub type IopadDso0 = crate::Reg<iopad_dso_0::IopadDso0Spec>;
#[doc = ""]
pub mod iopad_dso_0;
#[doc = "IOPAD_DSO_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_dso_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_dso_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_dso_1`] module"]
#[doc(alias = "IOPAD_DSO_1")]
pub type IopadDso1 = crate::Reg<iopad_dso_1::IopadDso1Spec>;
#[doc = ""]
pub mod iopad_dso_1;
#[doc = "IOPAD_DS1_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ds1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ds1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_ds1_0`] module"]
#[doc(alias = "IOPAD_DS1_0")]
pub type IopadDs1_0 = crate::Reg<iopad_ds1_0::IopadDs1_0Spec>;
#[doc = ""]
pub mod iopad_ds1_0;
#[doc = "IOPAD_DS1_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ds1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ds1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_ds1_1`] module"]
#[doc(alias = "IOPAD_DS1_1")]
pub type IopadDs1_1 = crate::Reg<iopad_ds1_1::IopadDs1_1Spec>;
#[doc = ""]
pub mod iopad_ds1_1;
#[doc = "IOPAD_PE_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_pe_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_pe_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_pe_0`] module"]
#[doc(alias = "IOPAD_PE_0")]
pub type IopadPe0 = crate::Reg<iopad_pe_0::IopadPe0Spec>;
#[doc = ""]
pub mod iopad_pe_0;
#[doc = "IOPAD_PE_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_pe_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_pe_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_pe_1`] module"]
#[doc(alias = "IOPAD_PE_1")]
pub type IopadPe1 = crate::Reg<iopad_pe_1::IopadPe1Spec>;
#[doc = ""]
pub mod iopad_pe_1;
#[doc = "IOPAD_PS_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ps_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ps_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_ps_0`] module"]
#[doc(alias = "IOPAD_PS_0")]
pub type IopadPs0 = crate::Reg<iopad_ps_0::IopadPs0Spec>;
#[doc = ""]
pub mod iopad_ps_0;
#[doc = "IOPAD_PS_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_ps_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_ps_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_ps_1`] module"]
#[doc(alias = "IOPAD_PS_1")]
pub type IopadPs1 = crate::Reg<iopad_ps_1::IopadPs1Spec>;
#[doc = ""]
pub mod iopad_ps_1;
#[doc = "IOPAD_SR_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_sr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_sr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_sr_0`] module"]
#[doc(alias = "IOPAD_SR_0")]
pub type IopadSr0 = crate::Reg<iopad_sr_0::IopadSr0Spec>;
#[doc = ""]
pub mod iopad_sr_0;
#[doc = "IOPAD_SR_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_sr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_sr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_sr_1`] module"]
#[doc(alias = "IOPAD_SR_1")]
pub type IopadSr1 = crate::Reg<iopad_sr_1::IopadSr1Spec>;
#[doc = ""]
pub mod iopad_sr_1;
#[doc = "IOPAD_IS_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_is_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_is_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_is_0`] module"]
#[doc(alias = "IOPAD_IS_0")]
pub type IopadIs0 = crate::Reg<iopad_is_0::IopadIs0Spec>;
#[doc = ""]
pub mod iopad_is_0;
#[doc = "IOPAD_IS_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`iopad_is_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopad_is_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iopad_is_1`] module"]
#[doc(alias = "IOPAD_IS_1")]
pub type IopadIs1 = crate::Reg<iopad_is_1::IopadIs1Spec>;
#[doc = ""]
pub mod iopad_is_1;
#[doc = "PVT_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_ctrl`] module"]
#[doc(alias = "PVT_CTRL")]
pub type PvtCtrl = crate::Reg<pvt_ctrl::PvtCtrlSpec>;
#[doc = ""]
pub mod pvt_ctrl;
#[doc = "SPARE0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spare0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spare0`] module"]
#[doc(alias = "SPARE0")]
pub type Spare0 = crate::Reg<spare0::Spare0Spec>;
#[doc = ""]
pub mod spare0;
#[doc = "STATIC_CONF_SIG1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`static_conf_sig1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static_conf_sig1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static_conf_sig1`] module"]
#[doc(alias = "STATIC_CONF_SIG1")]
pub type StaticConfSig1 = crate::Reg<static_conf_sig1::StaticConfSig1Spec>;
#[doc = ""]
pub mod static_conf_sig1;
#[doc = "FLASH_DIN_0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_din_0`] module"]
#[doc(alias = "FLASH_DIN_0")]
pub type FlashDin0 = crate::Reg<flash_din_0::FlashDin0Spec>;
#[doc = ""]
pub mod flash_din_0;
#[doc = "FLASH_DIN_1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_din_1`] module"]
#[doc(alias = "FLASH_DIN_1")]
pub type FlashDin1 = crate::Reg<flash_din_1::FlashDin1Spec>;
#[doc = ""]
pub mod flash_din_1;
#[doc = "FLASH_DIN_2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_din_2`] module"]
#[doc(alias = "FLASH_DIN_2")]
pub type FlashDin2 = crate::Reg<flash_din_2::FlashDin2Spec>;
#[doc = ""]
pub mod flash_din_2;
#[doc = "FLASH_DIN_3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_din_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_din_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_din_3`] module"]
#[doc(alias = "FLASH_DIN_3")]
pub type FlashDin3 = crate::Reg<flash_din_3::FlashDin3Spec>;
#[doc = ""]
pub mod flash_din_3;
#[doc = "FLASH0_DOUT_0 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash0_dout_0`] module"]
#[doc(alias = "FLASH0_DOUT_0")]
pub type Flash0Dout0 = crate::Reg<flash0_dout_0::Flash0Dout0Spec>;
#[doc = ""]
pub mod flash0_dout_0;
#[doc = "FLASH0_DOUT_1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash0_dout_1`] module"]
#[doc(alias = "FLASH0_DOUT_1")]
pub type Flash0Dout1 = crate::Reg<flash0_dout_1::Flash0Dout1Spec>;
#[doc = ""]
pub mod flash0_dout_1;
#[doc = "FLASH0_DOUT_2 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash0_dout_2`] module"]
#[doc(alias = "FLASH0_DOUT_2")]
pub type Flash0Dout2 = crate::Reg<flash0_dout_2::Flash0Dout2Spec>;
#[doc = ""]
pub mod flash0_dout_2;
#[doc = "FLASH0_DOUT_3 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash0_dout_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash0_dout_3`] module"]
#[doc(alias = "FLASH0_DOUT_3")]
pub type Flash0Dout3 = crate::Reg<flash0_dout_3::Flash0Dout3Spec>;
#[doc = ""]
pub mod flash0_dout_3;
#[doc = "FLASH1_DOUT_0 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash1_dout_0`] module"]
#[doc(alias = "FLASH1_DOUT_0")]
pub type Flash1Dout0 = crate::Reg<flash1_dout_0::Flash1Dout0Spec>;
#[doc = ""]
pub mod flash1_dout_0;
#[doc = "FLASH1_DOUT_1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash1_dout_1`] module"]
#[doc(alias = "FLASH1_DOUT_1")]
pub type Flash1Dout1 = crate::Reg<flash1_dout_1::Flash1Dout1Spec>;
#[doc = ""]
pub mod flash1_dout_1;
#[doc = "FLASH1_DOUT_2 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash1_dout_2`] module"]
#[doc(alias = "FLASH1_DOUT_2")]
pub type Flash1Dout2 = crate::Reg<flash1_dout_2::Flash1Dout2Spec>;
#[doc = ""]
pub mod flash1_dout_2;
#[doc = "FLASH1_DOUT_3 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash1_dout_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash1_dout_3`] module"]
#[doc(alias = "FLASH1_DOUT_3")]
pub type Flash1Dout3 = crate::Reg<flash1_dout_3::Flash1Dout3Spec>;
#[doc = ""]
pub mod flash1_dout_3;
#[doc = "SELECTION_CONTROL_REG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`selection_control_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selection_control_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selection_control_reg`] module"]
#[doc(alias = "SELECTION_CONTROL_REG")]
pub type SelectionControlReg = crate::Reg<selection_control_reg::SelectionControlRegSpec>;
#[doc = ""]
pub mod selection_control_reg;
#[doc = "AZ_ROM_REMAP_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_rom_remap_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_rom_remap_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_rom_remap_mask`] module"]
#[doc(alias = "AZ_ROM_REMAP_MASK")]
pub type AzRomRemapMask = crate::Reg<az_rom_remap_mask::AzRomRemapMaskSpec>;
#[doc = ""]
pub mod az_rom_remap_mask;
#[doc = "AZ_ROM_REMAP_OFFSET (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_rom_remap_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_rom_remap_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_rom_remap_offset`] module"]
#[doc(alias = "AZ_ROM_REMAP_OFFSET")]
pub type AzRomRemapOffset = crate::Reg<az_rom_remap_offset::AzRomRemapOffsetSpec>;
#[doc = ""]
pub mod az_rom_remap_offset;
#[doc = "AZ_CODE_REMAP_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_code_remap_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_code_remap_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_code_remap_mask`] module"]
#[doc(alias = "AZ_CODE_REMAP_MASK")]
pub type AzCodeRemapMask = crate::Reg<az_code_remap_mask::AzCodeRemapMaskSpec>;
#[doc = ""]
pub mod az_code_remap_mask;
#[doc = "AZ_CODE_REMAP_OFFSET (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_code_remap_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_code_remap_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_code_remap_offset`] module"]
#[doc(alias = "AZ_CODE_REMAP_OFFSET")]
pub type AzCodeRemapOffset = crate::Reg<az_code_remap_offset::AzCodeRemapOffsetSpec>;
#[doc = ""]
pub mod az_code_remap_offset;
#[doc = "AZ_SYS_REMAP_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_sys_remap_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_sys_remap_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_sys_remap_mask`] module"]
#[doc(alias = "AZ_SYS_REMAP_MASK")]
pub type AzSysRemapMask = crate::Reg<az_sys_remap_mask::AzSysRemapMaskSpec>;
#[doc = ""]
pub mod az_sys_remap_mask;
#[doc = "AZ_SYS_REMAP_OFFSET (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_sys_remap_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_sys_remap_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_sys_remap_offset`] module"]
#[doc(alias = "AZ_SYS_REMAP_OFFSET")]
pub type AzSysRemapOffset = crate::Reg<az_sys_remap_offset::AzSysRemapOffsetSpec>;
#[doc = ""]
pub mod az_sys_remap_offset;
#[doc = "AZ_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_ctrl`] module"]
#[doc(alias = "AZ_CTRL")]
pub type AzCtrl = crate::Reg<az_ctrl::AzCtrlSpec>;
#[doc = ""]
pub mod az_ctrl;
#[doc = "SSE_OTP_RD_DATA (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sse_otp_rd_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sse_otp_rd_data`] module"]
#[doc(alias = "SSE_OTP_RD_DATA")]
pub type SseOtpRdData = crate::Reg<sse_otp_rd_data::SseOtpRdDataSpec>;
#[doc = ""]
pub mod sse_otp_rd_data;
#[doc = "AZ_OTP_RD_DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`az_otp_rd_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_otp_rd_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@az_otp_rd_data`] module"]
#[doc(alias = "AZ_OTP_RD_DATA")]
pub type AzOtpRdData = crate::Reg<az_otp_rd_data::AzOtpRdDataSpec>;
#[doc = ""]
pub mod az_otp_rd_data;
#[doc = "SPARE_CTRL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spare_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spare_ctrl0`] module"]
#[doc(alias = "SPARE_CTRL0")]
pub type SpareCtrl0 = crate::Reg<spare_ctrl0::SpareCtrl0Spec>;
#[doc = ""]
pub mod spare_ctrl0;
#[doc = "SPARE_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spare_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spare_ctrl1`] module"]
#[doc(alias = "SPARE_CTRL1")]
pub type SpareCtrl1 = crate::Reg<spare_ctrl1::SpareCtrl1Spec>;
#[doc = ""]
pub mod spare_ctrl1;
#[doc = "CHIP_ID (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`chip_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chip_id`] module"]
#[doc(alias = "CHIP_ID")]
pub type ChipId = crate::Reg<chip_id::ChipIdSpec>;
#[doc = ""]
pub mod chip_id;
