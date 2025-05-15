#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdogload: Wdogload,
    wdogvalue: Wdogvalue,
    wdogcontrol: Wdogcontrol,
    wdogintclr: Wdogintclr,
    wdogris: Wdogris,
    wdogmis: Wdogmis,
    _reserved6: [u8; 0x0be8],
    wdoglock: Wdoglock,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Load Register"]
    #[inline(always)]
    pub const fn wdogload(&self) -> &Wdogload {
        &self.wdogload
    }
    #[doc = "0x04 - Watchdog Value Register"]
    #[inline(always)]
    pub const fn wdogvalue(&self) -> &Wdogvalue {
        &self.wdogvalue
    }
    #[doc = "0x08 - Watchdog Control Register"]
    #[inline(always)]
    pub const fn wdogcontrol(&self) -> &Wdogcontrol {
        &self.wdogcontrol
    }
    #[doc = "0x0c - Watchdog Interrupt Clear Register"]
    #[inline(always)]
    pub const fn wdogintclr(&self) -> &Wdogintclr {
        &self.wdogintclr
    }
    #[doc = "0x10 - Watchdog Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn wdogris(&self) -> &Wdogris {
        &self.wdogris
    }
    #[doc = "0x14 - Watchdog Mask Interrupt Status Register"]
    #[inline(always)]
    pub const fn wdogmis(&self) -> &Wdogmis {
        &self.wdogmis
    }
    #[doc = "0xc00 - Watchdog Lock Register"]
    #[inline(always)]
    pub const fn wdoglock(&self) -> &Wdoglock {
        &self.wdoglock
    }
}
#[doc = "WDOGLOAD (rw) register accessor: Watchdog Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogload`] module"]
#[doc(alias = "WDOGLOAD")]
pub type Wdogload = crate::Reg<wdogload::WdogloadSpec>;
#[doc = "Watchdog Load Register"]
pub mod wdogload;
#[doc = "WDOGVALUE (r) register accessor: Watchdog Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogvalue`] module"]
#[doc(alias = "WDOGVALUE")]
pub type Wdogvalue = crate::Reg<wdogvalue::WdogvalueSpec>;
#[doc = "Watchdog Value Register"]
pub mod wdogvalue;
#[doc = "WDOGCONTROL (rw) register accessor: Watchdog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcontrol`] module"]
#[doc(alias = "WDOGCONTROL")]
pub type Wdogcontrol = crate::Reg<wdogcontrol::WdogcontrolSpec>;
#[doc = "Watchdog Control Register"]
pub mod wdogcontrol;
#[doc = "WDOGINTCLR (w) register accessor: Watchdog Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogintclr`] module"]
#[doc(alias = "WDOGINTCLR")]
pub type Wdogintclr = crate::Reg<wdogintclr::WdogintclrSpec>;
#[doc = "Watchdog Interrupt Clear Register"]
pub mod wdogintclr;
#[doc = "WDOGRIS (r) register accessor: Watchdog Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogris`] module"]
#[doc(alias = "WDOGRIS")]
pub type Wdogris = crate::Reg<wdogris::WdogrisSpec>;
#[doc = "Watchdog Raw Interrupt Status Register"]
pub mod wdogris;
#[doc = "WDOGMIS (r) register accessor: Watchdog Mask Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogmis`] module"]
#[doc(alias = "WDOGMIS")]
pub type Wdogmis = crate::Reg<wdogmis::WdogmisSpec>;
#[doc = "Watchdog Mask Interrupt Status Register"]
pub mod wdogmis;
#[doc = "WDOGLOCK (rw) register accessor: Watchdog Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdoglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdoglock`] module"]
#[doc(alias = "WDOGLOCK")]
pub type Wdoglock = crate::Reg<wdoglock::WdoglockSpec>;
#[doc = "Watchdog Lock Register"]
pub mod wdoglock;
