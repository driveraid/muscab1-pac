#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer1load: Timer1load,
    timer1value: Timer1value,
    timer1control: Timer1control,
    timer1intclr: Timer1intclr,
    timer1ris: Timer1ris,
    timer1mis: Timer1mis,
    timer1bgload: Timer1bgload,
    _reserved7: [u8; 0x04],
    timer2load: Timer2load,
    timer2value: Timer2value,
    timer2control: Timer2control,
    timer2intclr: Timer2intclr,
    timer2ris: Timer2ris,
    timer2mis: Timer2mis,
    timer2bgload: Timer2bgload,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer 1 Load Register"]
    #[inline(always)]
    pub const fn timer1load(&self) -> &Timer1load {
        &self.timer1load
    }
    #[doc = "0x04 - Timer 1 Value Register"]
    #[inline(always)]
    pub const fn timer1value(&self) -> &Timer1value {
        &self.timer1value
    }
    #[doc = "0x08 - Timer 1 Control Register"]
    #[inline(always)]
    pub const fn timer1control(&self) -> &Timer1control {
        &self.timer1control
    }
    #[doc = "0x0c - Timer 1 Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timer1intclr(&self) -> &Timer1intclr {
        &self.timer1intclr
    }
    #[doc = "0x10 - Timer 1 Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer1ris(&self) -> &Timer1ris {
        &self.timer1ris
    }
    #[doc = "0x14 - Timer 1 Mask Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer1mis(&self) -> &Timer1mis {
        &self.timer1mis
    }
    #[doc = "0x18 - Timer 1 Background Load Register"]
    #[inline(always)]
    pub const fn timer1bgload(&self) -> &Timer1bgload {
        &self.timer1bgload
    }
    #[doc = "0x20 - Timer 2 Load Register"]
    #[inline(always)]
    pub const fn timer2load(&self) -> &Timer2load {
        &self.timer2load
    }
    #[doc = "0x24 - Timer 2 Value Register"]
    #[inline(always)]
    pub const fn timer2value(&self) -> &Timer2value {
        &self.timer2value
    }
    #[doc = "0x28 - Timer 2 Control Register"]
    #[inline(always)]
    pub const fn timer2control(&self) -> &Timer2control {
        &self.timer2control
    }
    #[doc = "0x2c - Timer 2 Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timer2intclr(&self) -> &Timer2intclr {
        &self.timer2intclr
    }
    #[doc = "0x30 - Timer 2 Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer2ris(&self) -> &Timer2ris {
        &self.timer2ris
    }
    #[doc = "0x34 - Timer 2 Mask Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer2mis(&self) -> &Timer2mis {
        &self.timer2mis
    }
    #[doc = "0x38 - Timer 2 Background Load Register"]
    #[inline(always)]
    pub const fn timer2bgload(&self) -> &Timer2bgload {
        &self.timer2bgload
    }
}
#[doc = "TIMER1LOAD (rw) register accessor: Timer 1 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1load`] module"]
#[doc(alias = "TIMER1LOAD")]
pub type Timer1load = crate::Reg<timer1load::Timer1loadSpec>;
#[doc = "Timer 1 Load Register"]
pub mod timer1load;
#[doc = "TIMER1VALUE (r) register accessor: Timer 1 Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1value`] module"]
#[doc(alias = "TIMER1VALUE")]
pub type Timer1value = crate::Reg<timer1value::Timer1valueSpec>;
#[doc = "Timer 1 Value Register"]
pub mod timer1value;
#[doc = "TIMER1CONTROL (rw) register accessor: Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1control`] module"]
#[doc(alias = "TIMER1CONTROL")]
pub type Timer1control = crate::Reg<timer1control::Timer1controlSpec>;
#[doc = "Timer 1 Control Register"]
pub mod timer1control;
#[doc = "TIMER1INTCLR (w) register accessor: Timer 1 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1intclr`] module"]
#[doc(alias = "TIMER1INTCLR")]
pub type Timer1intclr = crate::Reg<timer1intclr::Timer1intclrSpec>;
#[doc = "Timer 1 Interrupt Clear Register"]
pub mod timer1intclr;
#[doc = "TIMER1RIS (r) register accessor: Timer 1 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1ris`] module"]
#[doc(alias = "TIMER1RIS")]
pub type Timer1ris = crate::Reg<timer1ris::Timer1risSpec>;
#[doc = "Timer 1 Raw Interrupt Status Register"]
pub mod timer1ris;
#[doc = "TIMER1MIS (r) register accessor: Timer 1 Mask Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1mis`] module"]
#[doc(alias = "TIMER1MIS")]
pub type Timer1mis = crate::Reg<timer1mis::Timer1misSpec>;
#[doc = "Timer 1 Mask Interrupt Status Register"]
pub mod timer1mis;
#[doc = "TIMER1BGLOAD (rw) register accessor: Timer 1 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1bgload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1bgload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1bgload`] module"]
#[doc(alias = "TIMER1BGLOAD")]
pub type Timer1bgload = crate::Reg<timer1bgload::Timer1bgloadSpec>;
#[doc = "Timer 1 Background Load Register"]
pub mod timer1bgload;
#[doc = "TIMER2LOAD (rw) register accessor: Timer 2 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2load::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2load::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2load`] module"]
#[doc(alias = "TIMER2LOAD")]
pub type Timer2load = crate::Reg<timer2load::Timer2loadSpec>;
#[doc = "Timer 2 Load Register"]
pub mod timer2load;
#[doc = "TIMER2VALUE (r) register accessor: Timer 2 Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2value`] module"]
#[doc(alias = "TIMER2VALUE")]
pub type Timer2value = crate::Reg<timer2value::Timer2valueSpec>;
#[doc = "Timer 2 Value Register"]
pub mod timer2value;
#[doc = "TIMER2CONTROL (rw) register accessor: Timer 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2control`] module"]
#[doc(alias = "TIMER2CONTROL")]
pub type Timer2control = crate::Reg<timer2control::Timer2controlSpec>;
#[doc = "Timer 2 Control Register"]
pub mod timer2control;
#[doc = "TIMER2INTCLR (w) register accessor: Timer 2 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2intclr`] module"]
#[doc(alias = "TIMER2INTCLR")]
pub type Timer2intclr = crate::Reg<timer2intclr::Timer2intclrSpec>;
#[doc = "Timer 2 Interrupt Clear Register"]
pub mod timer2intclr;
#[doc = "TIMER2RIS (r) register accessor: Timer 2 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2ris`] module"]
#[doc(alias = "TIMER2RIS")]
pub type Timer2ris = crate::Reg<timer2ris::Timer2risSpec>;
#[doc = "Timer 2 Raw Interrupt Status Register"]
pub mod timer2ris;
#[doc = "TIMER2MIS (r) register accessor: Timer 2 Mask Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2mis`] module"]
#[doc(alias = "TIMER2MIS")]
pub type Timer2mis = crate::Reg<timer2mis::Timer2misSpec>;
#[doc = "Timer 2 Mask Interrupt Status Register"]
pub mod timer2mis;
#[doc = "TIMER2BGLOAD (rw) register accessor: Timer 2 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2bgload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2bgload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2bgload`] module"]
#[doc(alias = "TIMER2BGLOAD")]
pub type Timer2bgload = crate::Reg<timer2bgload::Timer2bgloadSpec>;
#[doc = "Timer 2 Background Load Register"]
pub mod timer2bgload;
