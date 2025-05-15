#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gptreset: Gptreset,
    gptintm: Gptintm,
    gptintc: Gptintc,
    _reserved3: [u8; 0x04],
    gptalarm0: Gptalarm0,
    gptalarm1: Gptalarm1,
    gptintr: Gptintr,
    gptcounter: Gptcounter,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Reset Register"]
    #[inline(always)]
    pub const fn gptreset(&self) -> &Gptreset {
        &self.gptreset
    }
    #[doc = "0x04 - Masked interrupt status register"]
    #[inline(always)]
    pub const fn gptintm(&self) -> &Gptintm {
        &self.gptintm
    }
    #[doc = "0x08 - Interrupt clear register"]
    #[inline(always)]
    pub const fn gptintc(&self) -> &Gptintc {
        &self.gptintc
    }
    #[doc = "0x10 - ALARM0 data value register"]
    #[inline(always)]
    pub const fn gptalarm0(&self) -> &Gptalarm0 {
        &self.gptalarm0
    }
    #[doc = "0x14 - ALARM1 data value register"]
    #[inline(always)]
    pub const fn gptalarm1(&self) -> &Gptalarm1 {
        &self.gptalarm1
    }
    #[doc = "0x18 - Raw interrupt status register"]
    #[inline(always)]
    pub const fn gptintr(&self) -> &Gptintr {
        &self.gptintr
    }
    #[doc = "0x1c - Counter data value register"]
    #[inline(always)]
    pub const fn gptcounter(&self) -> &Gptcounter {
        &self.gptcounter
    }
}
#[doc = "GPTRESET (r) register accessor: Control Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptreset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptreset`]
module"]
#[doc(alias = "GPTRESET")]
pub type Gptreset = crate::Reg<gptreset::GptresetSpec>;
#[doc = "Control Reset Register"]
pub mod gptreset;
#[doc = "GPTINTM (rw) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptintm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptintm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptintm`]
module"]
#[doc(alias = "GPTINTM")]
pub type Gptintm = crate::Reg<gptintm::GptintmSpec>;
#[doc = "Masked interrupt status register"]
pub mod gptintm;
#[doc = "GPTINTC (rw) register accessor: Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptintc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptintc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptintc`]
module"]
#[doc(alias = "GPTINTC")]
pub type Gptintc = crate::Reg<gptintc::GptintcSpec>;
#[doc = "Interrupt clear register"]
pub mod gptintc;
#[doc = "GPTALARM0 (rw) register accessor: ALARM0 data value register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptalarm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptalarm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptalarm0`]
module"]
#[doc(alias = "GPTALARM0")]
pub type Gptalarm0 = crate::Reg<gptalarm0::Gptalarm0Spec>;
#[doc = "ALARM0 data value register"]
pub mod gptalarm0;
#[doc = "GPTALARM1 (rw) register accessor: ALARM1 data value register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptalarm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptalarm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptalarm1`]
module"]
#[doc(alias = "GPTALARM1")]
pub type Gptalarm1 = crate::Reg<gptalarm1::Gptalarm1Spec>;
#[doc = "ALARM1 data value register"]
pub mod gptalarm1;
#[doc = "GPTINTR (r) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptintr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptintr`]
module"]
#[doc(alias = "GPTINTR")]
pub type Gptintr = crate::Reg<gptintr::GptintrSpec>;
#[doc = "Raw interrupt status register"]
pub mod gptintr;
#[doc = "GPTCOUNTER (r) register accessor: Counter data value register\n\nYou can [`read`](crate::Reg::read) this register and get [`gptcounter::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptcounter`]
module"]
#[doc(alias = "GPTCOUNTER")]
pub type Gptcounter = crate::Reg<gptcounter::GptcounterSpec>;
#[doc = "Counter data value register"]
pub mod gptcounter;
