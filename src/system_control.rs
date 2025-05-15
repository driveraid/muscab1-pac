#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    secdbgstat: Secdbgstat,
    secdbgset: Secdbgset,
    secdbgclr: Secdbgclr,
    scsecctrl: Scsecctrl,
    fclk_div: FclkDiv,
    sysclk_div: SysclkDiv,
    clock_force: ClockForce,
    _reserved7: [u8; 0xe4],
    reset_syndrome: ResetSyndrome,
    reset_mask: ResetMask,
    swreset: Swreset,
    gretreg: Gretreg,
    initsvrtor0: Initsvrtor0,
    initsvrtor1: Initsvrtor1,
    cpuwait: Cpuwait,
    nmi_enable: NmiEnable,
    wicctrl: Wicctrl,
    ewctrl: Ewctrl,
    _reserved17: [u8; 0xd8],
    pdcm_pd_sys_sense: PdcmPdSysSense,
    _reserved18: [u8; 0x08],
    pdcm_pd_sram0_sense: PdcmPdSram0Sense,
    pdcm_pd_sram1_sense: PdcmPdSram1Sense,
    pdcm_pd_sram2_sense: PdcmPdSram2Sense,
    pdcm_pd_sram3_sense: PdcmPdSram3Sense,
    _reserved22: [u8; 0x0db4],
    pidr4: Pidr4,
    _reserved23: [u8; 0x0c],
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Secure Debug Configuration Status"]
    #[inline(always)]
    pub const fn secdbgstat(&self) -> &Secdbgstat {
        &self.secdbgstat
    }
    #[doc = "0x04 - Secure Debug Configuration Set"]
    #[inline(always)]
    pub const fn secdbgset(&self) -> &Secdbgset {
        &self.secdbgset
    }
    #[doc = "0x08 - Secure Debug Configuration Clear"]
    #[inline(always)]
    pub const fn secdbgclr(&self) -> &Secdbgclr {
        &self.secdbgclr
    }
    #[doc = "0x0c - System Security Control"]
    #[inline(always)]
    pub const fn scsecctrl(&self) -> &Scsecctrl {
        &self.scsecctrl
    }
    #[doc = "0x10 - Fast Clock Divider Configuration"]
    #[inline(always)]
    pub const fn fclk_div(&self) -> &FclkDiv {
        &self.fclk_div
    }
    #[doc = "0x14 - System Clock Divider Configuration"]
    #[inline(always)]
    pub const fn sysclk_div(&self) -> &SysclkDiv {
        &self.sysclk_div
    }
    #[doc = "0x18 - Clock Force"]
    #[inline(always)]
    pub const fn clock_force(&self) -> &ClockForce {
        &self.clock_force
    }
    #[doc = "0x100 - Reset Syndrome"]
    #[inline(always)]
    pub const fn reset_syndrome(&self) -> &ResetSyndrome {
        &self.reset_syndrome
    }
    #[doc = "0x104 - Reset Mask"]
    #[inline(always)]
    pub const fn reset_mask(&self) -> &ResetMask {
        &self.reset_mask
    }
    #[doc = "0x108 - Software Reset"]
    #[inline(always)]
    pub const fn swreset(&self) -> &Swreset {
        &self.swreset
    }
    #[doc = "0x10c - General Purpose Retention"]
    #[inline(always)]
    pub const fn gretreg(&self) -> &Gretreg {
        &self.gretreg
    }
    #[doc = "0x110 - Initial Secure Reset Vector Register For CPU 0"]
    #[inline(always)]
    pub const fn initsvrtor0(&self) -> &Initsvrtor0 {
        &self.initsvrtor0
    }
    #[doc = "0x114 - Initial Secure Reset Vector Register For CPU 1"]
    #[inline(always)]
    pub const fn initsvrtor1(&self) -> &Initsvrtor1 {
        &self.initsvrtor1
    }
    #[doc = "0x118 - CPU Boot wait control after reset"]
    #[inline(always)]
    pub const fn cpuwait(&self) -> &Cpuwait {
        &self.cpuwait
    }
    #[doc = "0x11c - NMI Enable Register"]
    #[inline(always)]
    pub const fn nmi_enable(&self) -> &NmiEnable {
        &self.nmi_enable
    }
    #[doc = "0x120 - WIC request and acknowledge handshake"]
    #[inline(always)]
    pub const fn wicctrl(&self) -> &Wicctrl {
        &self.wicctrl
    }
    #[doc = "0x124 - External Wakeup Control"]
    #[inline(always)]
    pub const fn ewctrl(&self) -> &Ewctrl {
        &self.ewctrl
    }
    #[doc = "0x200 - External Wakeup Control"]
    #[inline(always)]
    pub const fn pdcm_pd_sys_sense(&self) -> &PdcmPdSysSense {
        &self.pdcm_pd_sys_sense
    }
    #[doc = "0x20c - Power Control Depedendency Matrix PD_SRAM0 Power Domain Sensitivity"]
    #[inline(always)]
    pub const fn pdcm_pd_sram0_sense(&self) -> &PdcmPdSram0Sense {
        &self.pdcm_pd_sram0_sense
    }
    #[doc = "0x210 - Power Control Depedendency Matrix PD_SRAM1 Power Domain Sensitivity"]
    #[inline(always)]
    pub const fn pdcm_pd_sram1_sense(&self) -> &PdcmPdSram1Sense {
        &self.pdcm_pd_sram1_sense
    }
    #[doc = "0x214 - Power Control Depedendency Matrix PD_SRAM2 Power Domain Sensitivity"]
    #[inline(always)]
    pub const fn pdcm_pd_sram2_sense(&self) -> &PdcmPdSram2Sense {
        &self.pdcm_pd_sram2_sense
    }
    #[doc = "0x218 - Power Control Depedendency Matrix PD_SRAM3 Power Domain Sensitivity"]
    #[inline(always)]
    pub const fn pdcm_pd_sram3_sense(&self) -> &PdcmPdSram3Sense {
        &self.pdcm_pd_sram3_sense
    }
    #[doc = "0xfd0 - Peripheral ID 4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfe0 - Peripheral ID 0"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Peripheral ID 1"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Peripheral ID 2"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Peripheral ID 3"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Component ID 0"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Component ID 1"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Component ID 2"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - Component ID 3"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "SECDBGSTAT (r) register accessor: Secure Debug Configuration Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secdbgstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secdbgstat`] module"]
#[doc(alias = "SECDBGSTAT")]
pub type Secdbgstat = crate::Reg<secdbgstat::SecdbgstatSpec>;
#[doc = "Secure Debug Configuration Status"]
pub mod secdbgstat;
#[doc = "SECDBGSET (w) register accessor: Secure Debug Configuration Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secdbgset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secdbgset`] module"]
#[doc(alias = "SECDBGSET")]
pub type Secdbgset = crate::Reg<secdbgset::SecdbgsetSpec>;
#[doc = "Secure Debug Configuration Set"]
pub mod secdbgset;
#[doc = "SECDBGCLR (w) register accessor: Secure Debug Configuration Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secdbgclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secdbgclr`] module"]
#[doc(alias = "SECDBGCLR")]
pub type Secdbgclr = crate::Reg<secdbgclr::SecdbgclrSpec>;
#[doc = "Secure Debug Configuration Clear"]
pub mod secdbgclr;
#[doc = "SCSECCTRL (rw) register accessor: System Security Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scsecctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsecctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsecctrl`] module"]
#[doc(alias = "SCSECCTRL")]
pub type Scsecctrl = crate::Reg<scsecctrl::ScsecctrlSpec>;
#[doc = "System Security Control"]
pub mod scsecctrl;
#[doc = "FCLK_DIV (rw) register accessor: Fast Clock Divider Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fclk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fclk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fclk_div`] module"]
#[doc(alias = "FCLK_DIV")]
pub type FclkDiv = crate::Reg<fclk_div::FclkDivSpec>;
#[doc = "Fast Clock Divider Configuration"]
pub mod fclk_div;
#[doc = "SYSCLK_DIV (rw) register accessor: System Clock Divider Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_div`] module"]
#[doc(alias = "SYSCLK_DIV")]
pub type SysclkDiv = crate::Reg<sysclk_div::SysclkDivSpec>;
#[doc = "System Clock Divider Configuration"]
pub mod sysclk_div;
#[doc = "CLOCK_FORCE (rw) register accessor: Clock Force\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_force`] module"]
#[doc(alias = "CLOCK_FORCE")]
pub type ClockForce = crate::Reg<clock_force::ClockForceSpec>;
#[doc = "Clock Force"]
pub mod clock_force;
#[doc = "RESET_SYNDROME (rw) register accessor: Reset Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_syndrome::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_syndrome::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_syndrome`] module"]
#[doc(alias = "RESET_SYNDROME")]
pub type ResetSyndrome = crate::Reg<reset_syndrome::ResetSyndromeSpec>;
#[doc = "Reset Syndrome"]
pub mod reset_syndrome;
#[doc = "RESET_MASK (rw) register accessor: Reset Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_mask`] module"]
#[doc(alias = "RESET_MASK")]
pub type ResetMask = crate::Reg<reset_mask::ResetMaskSpec>;
#[doc = "Reset Mask"]
pub mod reset_mask;
#[doc = "SWRESET (w) register accessor: Software Reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreset`] module"]
#[doc(alias = "SWRESET")]
pub type Swreset = crate::Reg<swreset::SwresetSpec>;
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "GRETREG (rw) register accessor: General Purpose Retention\n\nYou can [`read`](crate::Reg::read) this register and get [`gretreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gretreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gretreg`] module"]
#[doc(alias = "GRETREG")]
pub type Gretreg = crate::Reg<gretreg::GretregSpec>;
#[doc = "General Purpose Retention"]
pub mod gretreg;
#[doc = "INITSVRTOR0 (rw) register accessor: Initial Secure Reset Vector Register For CPU 0\n\nYou can [`read`](crate::Reg::read) this register and get [`initsvrtor0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initsvrtor0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initsvrtor0`] module"]
#[doc(alias = "INITSVRTOR0")]
pub type Initsvrtor0 = crate::Reg<initsvrtor0::Initsvrtor0Spec>;
#[doc = "Initial Secure Reset Vector Register For CPU 0"]
pub mod initsvrtor0;
#[doc = "INITSVRTOR1 (rw) register accessor: Initial Secure Reset Vector Register For CPU 1\n\nYou can [`read`](crate::Reg::read) this register and get [`initsvrtor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initsvrtor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initsvrtor1`] module"]
#[doc(alias = "INITSVRTOR1")]
pub type Initsvrtor1 = crate::Reg<initsvrtor1::Initsvrtor1Spec>;
#[doc = "Initial Secure Reset Vector Register For CPU 1"]
pub mod initsvrtor1;
#[doc = "CPUWAIT (rw) register accessor: CPU Boot wait control after reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuwait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuwait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuwait`] module"]
#[doc(alias = "CPUWAIT")]
pub type Cpuwait = crate::Reg<cpuwait::CpuwaitSpec>;
#[doc = "CPU Boot wait control after reset"]
pub mod cpuwait;
#[doc = "NMI_ENABLE (rw) register accessor: NMI Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_enable`] module"]
#[doc(alias = "NMI_ENABLE")]
pub type NmiEnable = crate::Reg<nmi_enable::NmiEnableSpec>;
#[doc = "NMI Enable Register"]
pub mod nmi_enable;
#[doc = "WICCTRL (rw) register accessor: WIC request and acknowledge handshake\n\nYou can [`read`](crate::Reg::read) this register and get [`wicctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wicctrl`] module"]
#[doc(alias = "WICCTRL")]
pub type Wicctrl = crate::Reg<wicctrl::WicctrlSpec>;
#[doc = "WIC request and acknowledge handshake"]
pub mod wicctrl;
#[doc = "EWCTRL (rw) register accessor: External Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ewctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewctrl`] module"]
#[doc(alias = "EWCTRL")]
pub type Ewctrl = crate::Reg<ewctrl::EwctrlSpec>;
#[doc = "External Wakeup Control"]
pub mod ewctrl;
#[doc = "PDCM_PD_SYS_SENSE (rw) register accessor: External Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcm_pd_sys_sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcm_pd_sys_sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcm_pd_sys_sense`] module"]
#[doc(alias = "PDCM_PD_SYS_SENSE")]
pub type PdcmPdSysSense = crate::Reg<pdcm_pd_sys_sense::PdcmPdSysSenseSpec>;
#[doc = "External Wakeup Control"]
pub mod pdcm_pd_sys_sense;
#[doc = "PDCM_PD_SRAM0_SENSE (rw) register accessor: Power Control Depedendency Matrix PD_SRAM0 Power Domain Sensitivity\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcm_pd_sram0_sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcm_pd_sram0_sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcm_pd_sram0_sense`] module"]
#[doc(alias = "PDCM_PD_SRAM0_SENSE")]
pub type PdcmPdSram0Sense = crate::Reg<pdcm_pd_sram0_sense::PdcmPdSram0SenseSpec>;
#[doc = "Power Control Depedendency Matrix PD_SRAM0 Power Domain Sensitivity"]
pub mod pdcm_pd_sram0_sense;
#[doc = "PDCM_PD_SRAM1_SENSE (rw) register accessor: Power Control Depedendency Matrix PD_SRAM1 Power Domain Sensitivity\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcm_pd_sram1_sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcm_pd_sram1_sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcm_pd_sram1_sense`] module"]
#[doc(alias = "PDCM_PD_SRAM1_SENSE")]
pub type PdcmPdSram1Sense = crate::Reg<pdcm_pd_sram1_sense::PdcmPdSram1SenseSpec>;
#[doc = "Power Control Depedendency Matrix PD_SRAM1 Power Domain Sensitivity"]
pub mod pdcm_pd_sram1_sense;
#[doc = "PDCM_PD_SRAM2_SENSE (rw) register accessor: Power Control Depedendency Matrix PD_SRAM2 Power Domain Sensitivity\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcm_pd_sram2_sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcm_pd_sram2_sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcm_pd_sram2_sense`] module"]
#[doc(alias = "PDCM_PD_SRAM2_SENSE")]
pub type PdcmPdSram2Sense = crate::Reg<pdcm_pd_sram2_sense::PdcmPdSram2SenseSpec>;
#[doc = "Power Control Depedendency Matrix PD_SRAM2 Power Domain Sensitivity"]
pub mod pdcm_pd_sram2_sense;
#[doc = "PDCM_PD_SRAM3_SENSE (rw) register accessor: Power Control Depedendency Matrix PD_SRAM3 Power Domain Sensitivity\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcm_pd_sram3_sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcm_pd_sram3_sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcm_pd_sram3_sense`] module"]
#[doc(alias = "PDCM_PD_SRAM3_SENSE")]
pub type PdcmPdSram3Sense = crate::Reg<pdcm_pd_sram3_sense::PdcmPdSram3SenseSpec>;
#[doc = "Power Control Depedendency Matrix PD_SRAM3 Power Domain Sensitivity"]
pub mod pdcm_pd_sram3_sense;
#[doc = "PIDR4 (r) register accessor: Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Peripheral ID 4"]
pub mod pidr4;
#[doc = "PIDR0 (r) register accessor: Peripheral ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`] module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Peripheral ID 0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: Peripheral ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`] module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Peripheral ID 1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: Peripheral ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`] module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Peripheral ID 2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`] module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Peripheral ID 3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: Component ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`] module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "Component ID 0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: Component ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`] module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "Component ID 1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: Component ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`] module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "Component ID 2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: Component ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`] module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "Component ID 3"]
pub mod cidr3;
