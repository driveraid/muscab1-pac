#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    type_: Type,
    rnr: Rnr,
    rbar: Rbar,
    rlar: Rlar,
    sfsr: Sfsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Type Register"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x08 - Region Number Register"]
    #[inline(always)]
    pub const fn rnr(&self) -> &Rnr {
        &self.rnr
    }
    #[doc = "0x0c - Region Base Address Register"]
    #[inline(always)]
    pub const fn rbar(&self) -> &Rbar {
        &self.rbar
    }
    #[doc = "0x10 - Region Limit Address Register"]
    #[inline(always)]
    pub const fn rlar(&self) -> &Rlar {
        &self.rlar
    }
    #[doc = "0x14 - Secure Fault Status Register"]
    #[inline(always)]
    pub const fn sfsr(&self) -> &Sfsr {
        &self.sfsr
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "TYPE (r) register accessor: Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`] module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "Type Register"]
pub mod type_;
#[doc = "RNR (rw) register accessor: Region Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnr`] module"]
#[doc(alias = "RNR")]
pub type Rnr = crate::Reg<rnr::RnrSpec>;
#[doc = "Region Number Register"]
pub mod rnr;
#[doc = "RBAR (rw) register accessor: Region Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbar`] module"]
#[doc(alias = "RBAR")]
pub type Rbar = crate::Reg<rbar::RbarSpec>;
#[doc = "Region Base Address Register"]
pub mod rbar;
#[doc = "RLAR (rw) register accessor: Region Limit Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlar`] module"]
#[doc(alias = "RLAR")]
pub type Rlar = crate::Reg<rlar::RlarSpec>;
#[doc = "Region Limit Address Register"]
pub mod rlar;
#[doc = "SFSR (rw) register accessor: Secure Fault Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfsr`] module"]
#[doc(alias = "SFSR")]
pub type Sfsr = crate::Reg<sfsr::SfsrSpec>;
#[doc = "Secure Fault Status Register"]
pub mod sfsr;
