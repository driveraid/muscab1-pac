#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: Data,
    dataout: Dataout,
    _reserved2: [u8; 0x08],
    outenset: Outenset,
    outenclr: Outenclr,
    altfuncset: Altfuncset,
    altfuncclr: Altfuncclr,
    intenset: Intenset,
    intenclr: Intenclr,
    inttypeset: Inttypeset,
    inttypeclr: Inttypeclr,
    intpolset: Intpolset,
    intpolclr: Intpolclr,
    _reserved_12_intclear: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Data Register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - Data Output Register"]
    #[inline(always)]
    pub const fn dataout(&self) -> &Dataout {
        &self.dataout
    }
    #[doc = "0x10 - Ouptut enable set Register"]
    #[inline(always)]
    pub const fn outenset(&self) -> &Outenset {
        &self.outenset
    }
    #[doc = "0x14 - Ouptut enable clear Register"]
    #[inline(always)]
    pub const fn outenclr(&self) -> &Outenclr {
        &self.outenclr
    }
    #[doc = "0x18 - Alternate function set Register"]
    #[inline(always)]
    pub const fn altfuncset(&self) -> &Altfuncset {
        &self.altfuncset
    }
    #[doc = "0x1c - Alternate function clear Register"]
    #[inline(always)]
    pub const fn altfuncclr(&self) -> &Altfuncclr {
        &self.altfuncclr
    }
    #[doc = "0x20 - Interrupt enable set Register"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x24 - Interrupt enable clear Register"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x28 - Interrupt type set Register"]
    #[inline(always)]
    pub const fn inttypeset(&self) -> &Inttypeset {
        &self.inttypeset
    }
    #[doc = "0x2c - Interrupt type clear Register"]
    #[inline(always)]
    pub const fn inttypeclr(&self) -> &Inttypeclr {
        &self.inttypeclr
    }
    #[doc = "0x30 - Polarity-level, edge interrupt configuration set Register"]
    #[inline(always)]
    pub const fn intpolset(&self) -> &Intpolset {
        &self.intpolset
    }
    #[doc = "0x34 - Polarity-level, edge interrupt configuration clear Register"]
    #[inline(always)]
    pub const fn intpolclr(&self) -> &Intpolclr {
        &self.intpolclr
    }
    #[doc = "0x38 - Interrupt CLEAR Register"]
    #[inline(always)]
    pub const fn intclear(&self) -> &Intclear {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
}
#[doc = "DATA (rw) register accessor: Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data Register"]
pub mod data;
#[doc = "DATAOUT (rw) register accessor: Data Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout`] module"]
#[doc(alias = "DATAOUT")]
pub type Dataout = crate::Reg<dataout::DataoutSpec>;
#[doc = "Data Output Register"]
pub mod dataout;
#[doc = "OUTENSET (rw) register accessor: Ouptut enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outenset`] module"]
#[doc(alias = "OUTENSET")]
pub type Outenset = crate::Reg<outenset::OutensetSpec>;
#[doc = "Ouptut enable set Register"]
pub mod outenset;
#[doc = "OUTENCLR (rw) register accessor: Ouptut enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outenclr`] module"]
#[doc(alias = "OUTENCLR")]
pub type Outenclr = crate::Reg<outenclr::OutenclrSpec>;
#[doc = "Ouptut enable clear Register"]
pub mod outenclr;
#[doc = "ALTFUNCSET (rw) register accessor: Alternate function set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altfuncset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altfuncset`] module"]
#[doc(alias = "ALTFUNCSET")]
pub type Altfuncset = crate::Reg<altfuncset::AltfuncsetSpec>;
#[doc = "Alternate function set Register"]
pub mod altfuncset;
#[doc = "ALTFUNCCLR (rw) register accessor: Alternate function clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altfuncclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altfuncclr`] module"]
#[doc(alias = "ALTFUNCCLR")]
pub type Altfuncclr = crate::Reg<altfuncclr::AltfuncclrSpec>;
#[doc = "Alternate function clear Register"]
pub mod altfuncclr;
#[doc = "INTENSET (rw) register accessor: Interrupt enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt enable set Register"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt enable clear Register"]
pub mod intenclr;
#[doc = "INTTYPESET (rw) register accessor: Interrupt type set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttypeset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttypeset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttypeset`] module"]
#[doc(alias = "INTTYPESET")]
pub type Inttypeset = crate::Reg<inttypeset::InttypesetSpec>;
#[doc = "Interrupt type set Register"]
pub mod inttypeset;
#[doc = "INTTYPECLR (rw) register accessor: Interrupt type clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttypeclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttypeclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttypeclr`] module"]
#[doc(alias = "INTTYPECLR")]
pub type Inttypeclr = crate::Reg<inttypeclr::InttypeclrSpec>;
#[doc = "Interrupt type clear Register"]
pub mod inttypeclr;
#[doc = "INTPOLSET (rw) register accessor: Polarity-level, edge interrupt configuration set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpolset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpolset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpolset`] module"]
#[doc(alias = "INTPOLSET")]
pub type Intpolset = crate::Reg<intpolset::IntpolsetSpec>;
#[doc = "Polarity-level, edge interrupt configuration set Register"]
pub mod intpolset;
#[doc = "INTPOLCLR (rw) register accessor: Polarity-level, edge interrupt configuration clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpolclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpolclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpolclr`] module"]
#[doc(alias = "INTPOLCLR")]
pub type Intpolclr = crate::Reg<intpolclr::IntpolclrSpec>;
#[doc = "Polarity-level, edge interrupt configuration clear Register"]
pub mod intpolclr;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`] module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "INTCLEAR (w) register accessor: Interrupt CLEAR Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear`] module"]
#[doc(alias = "INTCLEAR")]
pub type Intclear = crate::Reg<intclear::IntclearSpec>;
#[doc = "Interrupt CLEAR Register"]
pub mod intclear;
