#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    value: Value,
    reload: Reload,
    _reserved_3_intclear: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Current Timer Counter Value"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
    #[doc = "0x08 - Counter Reload Value"]
    #[inline(always)]
    pub const fn reload(&self) -> &Reload {
        &self.reload
    }
    #[doc = "0x0c - Timer Interrupt clear register"]
    #[inline(always)]
    pub const fn intclear(&self) -> &Intclear {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Timer Interrupt status register"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "VALUE (rw) register accessor: Current Timer Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Current Timer Counter Value"]
pub mod value;
#[doc = "RELOAD (rw) register accessor: Counter Reload Value\n\nYou can [`read`](crate::Reg::read) this register and get [`reload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reload`] module"]
#[doc(alias = "RELOAD")]
pub type Reload = crate::Reg<reload::ReloadSpec>;
#[doc = "Counter Reload Value"]
pub mod reload;
#[doc = "INTSTATUS (r) register accessor: Timer Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`] module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Timer Interrupt status register"]
pub mod intstatus;
#[doc = "INTCLEAR (w) register accessor: Timer Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear`] module"]
#[doc(alias = "INTCLEAR")]
pub type Intclear = crate::Reg<intclear::IntclearSpec>;
#[doc = "Timer Interrupt clear register"]
pub mod intclear;
