#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ichwparams: Ichwparams,
    icctrl: Icctrl,
    _reserved2: [u8; 0xf8],
    icirqstat: Icirqstat,
    icirqsclr: Icirqsclr,
    icirqen: Icirqen,
    icdbgfillerr: Icdbgfillerr,
    _reserved6: [u8; 0x01f0],
    icsh: Icsh,
    icsm: Icsm,
    icsuc: Icsuc,
    _reserved9: [u8; 0x0cc4],
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
    #[doc = "0x00 - Hardware Parameter Register"]
    #[inline(always)]
    pub const fn ichwparams(&self) -> &Ichwparams {
        &self.ichwparams
    }
    #[doc = "0x04 - Instruction Cache Control Register"]
    #[inline(always)]
    pub const fn icctrl(&self) -> &Icctrl {
        &self.icctrl
    }
    #[doc = "0x100 - Interrupt Request Status Register"]
    #[inline(always)]
    pub const fn icirqstat(&self) -> &Icirqstat {
        &self.icirqstat
    }
    #[doc = "0x104 - Interrupt Status Clear register"]
    #[inline(always)]
    pub const fn icirqsclr(&self) -> &Icirqsclr {
        &self.icirqsclr
    }
    #[doc = "0x108 - Interrupt Enable register"]
    #[inline(always)]
    pub const fn icirqen(&self) -> &Icirqen {
        &self.icirqen
    }
    #[doc = "0x10c - Address where the latest fill error was seen"]
    #[inline(always)]
    pub const fn icdbgfillerr(&self) -> &Icdbgfillerr {
        &self.icdbgfillerr
    }
    #[doc = "0x300 - Instruction Cache Statistic Hit Count register"]
    #[inline(always)]
    pub const fn icsh(&self) -> &Icsh {
        &self.icsh
    }
    #[doc = "0x304 - Instruction Cache Statistic Miss Count register"]
    #[inline(always)]
    pub const fn icsm(&self) -> &Icsm {
        &self.icsm
    }
    #[doc = "0x308 - Instruction Cache Statistic Uncached Count register"]
    #[inline(always)]
    pub const fn icsuc(&self) -> &Icsuc {
        &self.icsuc
    }
    #[doc = "0xfd0 - Product ID Register 4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Product ID Register 5"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Product ID Register 6"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Product ID Register 7"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - Product ID Register 0"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Product ID Register 1"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Product ID Register 2"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Product ID Register 3"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Component ID Register 0"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Component ID Register 1"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Component ID Register 2"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - Component ID Register 3"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "ICHWPARAMS (r) register accessor: Hardware Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ichwparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ichwparams`] module"]
#[doc(alias = "ICHWPARAMS")]
pub type Ichwparams = crate::Reg<ichwparams::IchwparamsSpec>;
#[doc = "Hardware Parameter Register"]
pub mod ichwparams;
#[doc = "ICCTRL (rw) register accessor: Instruction Cache Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icctrl`] module"]
#[doc(alias = "ICCTRL")]
pub type Icctrl = crate::Reg<icctrl::IcctrlSpec>;
#[doc = "Instruction Cache Control Register"]
pub mod icctrl;
#[doc = "ICIRQSTAT (r) register accessor: Interrupt Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icirqstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icirqstat`] module"]
#[doc(alias = "ICIRQSTAT")]
pub type Icirqstat = crate::Reg<icirqstat::IcirqstatSpec>;
#[doc = "Interrupt Request Status Register"]
pub mod icirqstat;
#[doc = "ICIRQSCLR (w) register accessor: Interrupt Status Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icirqsclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icirqsclr`] module"]
#[doc(alias = "ICIRQSCLR")]
pub type Icirqsclr = crate::Reg<icirqsclr::IcirqsclrSpec>;
#[doc = "Interrupt Status Clear register"]
pub mod icirqsclr;
#[doc = "ICIRQEN (rw) register accessor: Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`icirqen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icirqen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icirqen`] module"]
#[doc(alias = "ICIRQEN")]
pub type Icirqen = crate::Reg<icirqen::IcirqenSpec>;
#[doc = "Interrupt Enable register"]
pub mod icirqen;
#[doc = "ICDBGFILLERR (r) register accessor: Address where the latest fill error was seen\n\nYou can [`read`](crate::Reg::read) this register and get [`icdbgfillerr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icdbgfillerr`] module"]
#[doc(alias = "ICDBGFILLERR")]
pub type Icdbgfillerr = crate::Reg<icdbgfillerr::IcdbgfillerrSpec>;
#[doc = "Address where the latest fill error was seen"]
pub mod icdbgfillerr;
#[doc = "ICSH (r) register accessor: Instruction Cache Statistic Hit Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsh`] module"]
#[doc(alias = "ICSH")]
pub type Icsh = crate::Reg<icsh::IcshSpec>;
#[doc = "Instruction Cache Statistic Hit Count register"]
pub mod icsh;
#[doc = "ICSM (r) register accessor: Instruction Cache Statistic Miss Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsm`] module"]
#[doc(alias = "ICSM")]
pub type Icsm = crate::Reg<icsm::IcsmSpec>;
#[doc = "Instruction Cache Statistic Miss Count register"]
pub mod icsm;
#[doc = "ICSUC (r) register accessor: Instruction Cache Statistic Uncached Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsuc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsuc`] module"]
#[doc(alias = "ICSUC")]
pub type Icsuc = crate::Reg<icsuc::IcsucSpec>;
#[doc = "Instruction Cache Statistic Uncached Count register"]
pub mod icsuc;
#[doc = "PIDR4 (r) register accessor: Product ID Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Product ID Register 4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: Product ID Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`] module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Product ID Register 5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: Product ID Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`] module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Product ID Register 6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: Product ID Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`] module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Product ID Register 7"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: Product ID Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`] module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Product ID Register 0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: Product ID Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`] module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Product ID Register 1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: Product ID Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`] module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Product ID Register 2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: Product ID Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`] module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Product ID Register 3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: Component ID Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`] module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "Component ID Register 0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: Component ID Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`] module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "Component ID Register 1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: Component ID Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`] module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "Component ID Register 2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: Component ID Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`] module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "Component ID Register 3"]
pub mod cidr3;
