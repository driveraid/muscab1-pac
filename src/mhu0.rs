#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu0intr_stat: Cpu0intrStat,
    cpu0intr_set: Cpu0intrSet,
    cpu0intr_clr: Cpu0intrClr,
    _reserved3: [u8; 0x04],
    cpu1intr_stat: Cpu1intrStat,
    cpu1intr_set: Cpu1intrSet,
    cpu1intr_clr: Cpu1intrClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Core 0 interrupt status register"]
    #[inline(always)]
    pub const fn cpu0intr_stat(&self) -> &Cpu0intrStat {
        &self.cpu0intr_stat
    }
    #[doc = "0x04 - Core 0 interrupt set register"]
    #[inline(always)]
    pub const fn cpu0intr_set(&self) -> &Cpu0intrSet {
        &self.cpu0intr_set
    }
    #[doc = "0x08 - Core 0 interrupt clear register"]
    #[inline(always)]
    pub const fn cpu0intr_clr(&self) -> &Cpu0intrClr {
        &self.cpu0intr_clr
    }
    #[doc = "0x10 - Core 1 interrupt status register"]
    #[inline(always)]
    pub const fn cpu1intr_stat(&self) -> &Cpu1intrStat {
        &self.cpu1intr_stat
    }
    #[doc = "0x14 - Core 1 interrupt set register"]
    #[inline(always)]
    pub const fn cpu1intr_set(&self) -> &Cpu1intrSet {
        &self.cpu1intr_set
    }
    #[doc = "0x18 - Core 1 interrupt clear register"]
    #[inline(always)]
    pub const fn cpu1intr_clr(&self) -> &Cpu1intrClr {
        &self.cpu1intr_clr
    }
}
#[doc = "CPU0INTR_STAT (r) register accessor: Core 0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0intr_stat`]
module"]
#[doc(alias = "CPU0INTR_STAT")]
pub type Cpu0intrStat = crate::Reg<cpu0intr_stat::Cpu0intrStatSpec>;
#[doc = "Core 0 interrupt status register"]
pub mod cpu0intr_stat;
#[doc = "CPU0INTR_SET (w) register accessor: Core 0 interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0intr_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0intr_set`]
module"]
#[doc(alias = "CPU0INTR_SET")]
pub type Cpu0intrSet = crate::Reg<cpu0intr_set::Cpu0intrSetSpec>;
#[doc = "Core 0 interrupt set register"]
pub mod cpu0intr_set;
#[doc = "CPU0INTR_CLR (w) register accessor: Core 0 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0intr_clr`]
module"]
#[doc(alias = "CPU0INTR_CLR")]
pub type Cpu0intrClr = crate::Reg<cpu0intr_clr::Cpu0intrClrSpec>;
#[doc = "Core 0 interrupt clear register"]
pub mod cpu0intr_clr;
#[doc = "CPU1INTR_STAT (r) register accessor: Core 1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1intr_stat`]
module"]
#[doc(alias = "CPU1INTR_STAT")]
pub type Cpu1intrStat = crate::Reg<cpu1intr_stat::Cpu1intrStatSpec>;
#[doc = "Core 1 interrupt status register"]
pub mod cpu1intr_stat;
#[doc = "CPU1INTR_SET (w) register accessor: Core 1 interrupt set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1intr_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1intr_set`]
module"]
#[doc(alias = "CPU1INTR_SET")]
pub type Cpu1intrSet = crate::Reg<cpu1intr_set::Cpu1intrSetSpec>;
#[doc = "Core 1 interrupt set register"]
pub mod cpu1intr_set;
#[doc = "CPU1INTR_CLR (w) register accessor: Core 1 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1intr_clr`]
module"]
#[doc(alias = "CPU1INTR_CLR")]
pub type Cpu1intrClr = crate::Reg<cpu1intr_clr::Cpu1intrClrSpec>;
#[doc = "Core 1 interrupt clear register"]
pub mod cpu1intr_clr;
