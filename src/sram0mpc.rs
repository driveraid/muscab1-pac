#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x0c],
    blk_max: BlkMax,
    blk_cfg: BlkCfg,
    blk_idx: BlkIdx,
    blk_lut: BlkLut,
    int_stat: IntStat,
    int_clear: IntClear,
    int_en: IntEn,
    int_info1: IntInfo1,
    int_info2: IntInfo2,
    int_set: IntSet,
    _reserved11: [u8; 0x0f98],
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
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
    #[doc = "0x00 - MPC Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - Maximum value of block based index register"]
    #[inline(always)]
    pub const fn blk_max(&self) -> &BlkMax {
        &self.blk_max
    }
    #[doc = "0x14 - Block Configuration"]
    #[inline(always)]
    pub const fn blk_cfg(&self) -> &BlkCfg {
        &self.blk_cfg
    }
    #[doc = "0x18 - Index value for accessing block based look up table"]
    #[inline(always)]
    pub const fn blk_idx(&self) -> &BlkIdx {
        &self.blk_idx
    }
    #[doc = "0x1c - Block based gating Look Up Table"]
    #[inline(always)]
    pub const fn blk_lut(&self) -> &BlkLut {
        &self.blk_lut
    }
    #[doc = "0x20 - Interrupt state"]
    #[inline(always)]
    pub const fn int_stat(&self) -> &IntStat {
        &self.int_stat
    }
    #[doc = "0x24 - Interrupt clear"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x28 - Interrupt enable"]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x2c - Interrupt information 1"]
    #[inline(always)]
    pub const fn int_info1(&self) -> &IntInfo1 {
        &self.int_info1
    }
    #[doc = "0x30 - Interrupt information 2"]
    #[inline(always)]
    pub const fn int_info2(&self) -> &IntInfo2 {
        &self.int_info2
    }
    #[doc = "0x34 - Interrupt set. Debug purpose only"]
    #[inline(always)]
    pub const fn int_set(&self) -> &IntSet {
        &self.int_set
    }
    #[doc = "0xfd0 - Peripheral ID 4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Peripheral ID 5"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Peripheral ID 6"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Peripheral ID 7"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
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
#[doc = "CTRL (rw) register accessor: MPC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "MPC Control register"]
pub mod ctrl;
#[doc = "BLK_MAX (r) register accessor: Maximum value of block based index register\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_max::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_max`] module"]
#[doc(alias = "BLK_MAX")]
pub type BlkMax = crate::Reg<blk_max::BlkMaxSpec>;
#[doc = "Maximum value of block based index register"]
pub mod blk_max;
#[doc = "BLK_CFG (r) register accessor: Block Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_cfg`] module"]
#[doc(alias = "BLK_CFG")]
pub type BlkCfg = crate::Reg<blk_cfg::BlkCfgSpec>;
#[doc = "Block Configuration"]
pub mod blk_cfg;
#[doc = "BLK_IDX (rw) register accessor: Index value for accessing block based look up table\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_idx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_idx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_idx`] module"]
#[doc(alias = "BLK_IDX")]
pub type BlkIdx = crate::Reg<blk_idx::BlkIdxSpec>;
#[doc = "Index value for accessing block based look up table"]
pub mod blk_idx;
#[doc = "BLK_LUT (rw) register accessor: Block based gating Look Up Table\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_lut`] module"]
#[doc(alias = "BLK_LUT")]
pub type BlkLut = crate::Reg<blk_lut::BlkLutSpec>;
#[doc = "Block based gating Look Up Table"]
pub mod blk_lut;
#[doc = "INT_STAT (r) register accessor: Interrupt state\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_stat`] module"]
#[doc(alias = "INT_STAT")]
pub type IntStat = crate::Reg<int_stat::IntStatSpec>;
#[doc = "Interrupt state"]
pub mod int_stat;
#[doc = "INT_CLEAR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`] module"]
#[doc(alias = "INT_CLEAR")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = "Interrupt clear"]
pub mod int_clear;
#[doc = "INT_EN (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "Interrupt enable"]
pub mod int_en;
#[doc = "INT_INFO1 (r) register accessor: Interrupt information 1\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_info1`] module"]
#[doc(alias = "INT_INFO1")]
pub type IntInfo1 = crate::Reg<int_info1::IntInfo1Spec>;
#[doc = "Interrupt information 1"]
pub mod int_info1;
#[doc = "INT_INFO2 (r) register accessor: Interrupt information 2\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_info2`] module"]
#[doc(alias = "INT_INFO2")]
pub type IntInfo2 = crate::Reg<int_info2::IntInfo2Spec>;
#[doc = "Interrupt information 2"]
pub mod int_info2;
#[doc = "INT_SET (w) register accessor: Interrupt set. Debug purpose only\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_set`] module"]
#[doc(alias = "INT_SET")]
pub type IntSet = crate::Reg<int_set::IntSetSpec>;
#[doc = "Interrupt set. Debug purpose only"]
pub mod int_set;
#[doc = "PIDR4 (r) register accessor: Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Peripheral ID 4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: Peripheral ID 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`] module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Peripheral ID 5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: Peripheral ID 6\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`] module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Peripheral ID 6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: Peripheral ID 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`] module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Peripheral ID 7"]
pub mod pidr7;
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
