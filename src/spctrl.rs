#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spcsectrl: Spcsectrl,
    buswait: Buswait,
    _reserved2: [u8; 0x08],
    secrespcfg: Secrespcfg,
    nsccfg: Nsccfg,
    _reserved4: [u8; 0x04],
    secmpcintstatus: Secmpcintstatus,
    secppcintstat: Secppcintstat,
    secppcintclr: Secppcintclr,
    secppcinten: Secppcinten,
    _reserved8: [u8; 0x04],
    secmscintstat: Secmscintstat,
    secmscintclr: Secmscintclr,
    secmscinten: Secmscinten,
    _reserved11: [u8; 0x04],
    brgintstat: Brgintstat,
    brgintclr: Brgintclr,
    brginten: Brginten,
    _reserved14: [u8; 0x04],
    ahbnsppc0: Ahbnsppc0,
    _reserved15: [u8; 0x0c],
    ahbnsppcexp0: Ahbnsppcexp0,
    ahbnsppcexp1: Ahbnsppcexp1,
    ahbnsppcexp2: Ahbnsppcexp2,
    ahbnsppcexp3: Ahbnsppcexp3,
    apbnsppc0: Apbnsppc0,
    apbnsppc1: Apbnsppc1,
    _reserved21: [u8; 0x08],
    apbnsppcexp0: Apbnsppcexp0,
    apbnsppcexp1: Apbnsppcexp1,
    apbnsppcexp2: Apbnsppcexp2,
    apbnsppcexp3: Apbnsppcexp3,
    ahbspppc0: Ahbspppc0,
    _reserved26: [u8; 0x0c],
    ahbspppcexp0: Ahbspppcexp0,
    ahbspppcexp1: Ahbspppcexp1,
    ahbspppcexp2: Ahbspppcexp2,
    ahbspppcexp3: Ahbspppcexp3,
    apbspppc0: Apbspppc0,
    apbspppc1: Apbspppc1,
    _reserved32: [u8; 0x08],
    apbspppcexp0: Apbspppcexp0,
    apbspppcexp1: Apbspppcexp1,
    apbspppcexp2: Apbspppcexp2,
    apbspppcexp3: Apbspppcexp3,
    nsmscexp: Nsmscexp,
    _reserved37: [u8; 0x0efc],
    pid4: Pid4,
    _reserved38: [u8; 0x0c],
    pid0: Pid0,
    pid1: Pid1,
    pid2: Pid2,
    pid3: Pid3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Secure Privilege Controller Secure Configuration Control register"]
    #[inline(always)]
    pub const fn spcsectrl(&self) -> &Spcsectrl {
        &self.spcsectrl
    }
    #[doc = "0x04 - Bus Access wait control after reset"]
    #[inline(always)]
    pub const fn buswait(&self) -> &Buswait {
        &self.buswait
    }
    #[doc = "0x10 - Security Violation Response Configuration register"]
    #[inline(always)]
    pub const fn secrespcfg(&self) -> &Secrespcfg {
        &self.secrespcfg
    }
    #[doc = "0x14 - Non Secure Callable Configuration for IDAU"]
    #[inline(always)]
    pub const fn nsccfg(&self) -> &Nsccfg {
        &self.nsccfg
    }
    #[doc = "0x1c - Secure MPC Interrupt Status"]
    #[inline(always)]
    pub const fn secmpcintstatus(&self) -> &Secmpcintstatus {
        &self.secmpcintstatus
    }
    #[doc = "0x20 - Secure PPC Interrupt Status"]
    #[inline(always)]
    pub const fn secppcintstat(&self) -> &Secppcintstat {
        &self.secppcintstat
    }
    #[doc = "0x24 - Secure PPC Interrupt Clear"]
    #[inline(always)]
    pub const fn secppcintclr(&self) -> &Secppcintclr {
        &self.secppcintclr
    }
    #[doc = "0x28 - Secure PPC Interrupt Enable"]
    #[inline(always)]
    pub const fn secppcinten(&self) -> &Secppcinten {
        &self.secppcinten
    }
    #[doc = "0x30 - Secure MSC Interrupt Status"]
    #[inline(always)]
    pub const fn secmscintstat(&self) -> &Secmscintstat {
        &self.secmscintstat
    }
    #[doc = "0x34 - Secure MSC Interrupt Clear"]
    #[inline(always)]
    pub const fn secmscintclr(&self) -> &Secmscintclr {
        &self.secmscintclr
    }
    #[doc = "0x38 - Secure MSC Interrupt Enable"]
    #[inline(always)]
    pub const fn secmscinten(&self) -> &Secmscinten {
        &self.secmscinten
    }
    #[doc = "0x40 - Bridge Buffer Error Interrupt Status"]
    #[inline(always)]
    pub const fn brgintstat(&self) -> &Brgintstat {
        &self.brgintstat
    }
    #[doc = "0x44 - Bridge Buffer Error Interrupt Clear"]
    #[inline(always)]
    pub const fn brgintclr(&self) -> &Brgintclr {
        &self.brgintclr
    }
    #[doc = "0x48 - Bridge Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub const fn brginten(&self) -> &Brginten {
        &self.brginten
    }
    #[doc = "0x50 - Non-Secure Access AHB slave Peripheral Protection Control 0"]
    #[inline(always)]
    pub const fn ahbnsppc0(&self) -> &Ahbnsppc0 {
        &self.ahbnsppc0
    }
    #[doc = "0x60 - Expansion 0 Non_Secure Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnsppcexp0(&self) -> &Ahbnsppcexp0 {
        &self.ahbnsppcexp0
    }
    #[doc = "0x64 - Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnsppcexp1(&self) -> &Ahbnsppcexp1 {
        &self.ahbnsppcexp1
    }
    #[doc = "0x68 - Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnsppcexp2(&self) -> &Ahbnsppcexp2 {
        &self.ahbnsppcexp2
    }
    #[doc = "0x6c - Expansion 3 Non_Secure Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnsppcexp3(&self) -> &Ahbnsppcexp3 {
        &self.ahbnsppcexp3
    }
    #[doc = "0x70 - Non-Secure Access APB slave Peripheral Protection Control 0"]
    #[inline(always)]
    pub const fn apbnsppc0(&self) -> &Apbnsppc0 {
        &self.apbnsppc0
    }
    #[doc = "0x74 - Non-Secure Access APB slave Peripheral Protection Control 1"]
    #[inline(always)]
    pub const fn apbnsppc1(&self) -> &Apbnsppc1 {
        &self.apbnsppc1
    }
    #[doc = "0x80 - Expansion 0 Non_Secure Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnsppcexp0(&self) -> &Apbnsppcexp0 {
        &self.apbnsppcexp0
    }
    #[doc = "0x84 - Expansion 1 Non_Secure Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnsppcexp1(&self) -> &Apbnsppcexp1 {
        &self.apbnsppcexp1
    }
    #[doc = "0x88 - Expansion 2 Non_Secure Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnsppcexp2(&self) -> &Apbnsppcexp2 {
        &self.apbnsppcexp2
    }
    #[doc = "0x8c - Expansion 3 Non_Secure Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnsppcexp3(&self) -> &Apbnsppcexp3 {
        &self.apbnsppcexp3
    }
    #[doc = "0x90 - Secure Unprivileged Access AHB slave Peripheral Protection Control 0"]
    #[inline(always)]
    pub const fn ahbspppc0(&self) -> &Ahbspppc0 {
        &self.ahbspppc0
    }
    #[doc = "0xa0 - Expansion 0 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbspppcexp0(&self) -> &Ahbspppcexp0 {
        &self.ahbspppcexp0
    }
    #[doc = "0xa4 - Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbspppcexp1(&self) -> &Ahbspppcexp1 {
        &self.ahbspppcexp1
    }
    #[doc = "0xa8 - Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbspppcexp2(&self) -> &Ahbspppcexp2 {
        &self.ahbspppcexp2
    }
    #[doc = "0xac - Expansion 3 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbspppcexp3(&self) -> &Ahbspppcexp3 {
        &self.ahbspppcexp3
    }
    #[doc = "0xb0 - Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
    #[inline(always)]
    pub const fn apbspppc0(&self) -> &Apbspppc0 {
        &self.apbspppc0
    }
    #[doc = "0xb4 - Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
    #[inline(always)]
    pub const fn apbspppc1(&self) -> &Apbspppc1 {
        &self.apbspppc1
    }
    #[doc = "0xc0 - Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbspppcexp0(&self) -> &Apbspppcexp0 {
        &self.apbspppcexp0
    }
    #[doc = "0xc4 - Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbspppcexp1(&self) -> &Apbspppcexp1 {
        &self.apbspppcexp1
    }
    #[doc = "0xc8 - Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbspppcexp2(&self) -> &Apbspppcexp2 {
        &self.apbspppcexp2
    }
    #[doc = "0xcc - Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbspppcexp3(&self) -> &Apbspppcexp3 {
        &self.apbspppcexp3
    }
    #[doc = "0xd0 - Expansion MSC Non-Secure Configuration"]
    #[inline(always)]
    pub const fn nsmscexp(&self) -> &Nsmscexp {
        &self.nsmscexp
    }
    #[doc = "0xfd0 - Peripheral ID 4"]
    #[inline(always)]
    pub const fn pid4(&self) -> &Pid4 {
        &self.pid4
    }
    #[doc = "0xfe0 - Peripheral ID 0"]
    #[inline(always)]
    pub const fn pid0(&self) -> &Pid0 {
        &self.pid0
    }
    #[doc = "0xfe4 - Peripheral ID 1"]
    #[inline(always)]
    pub const fn pid1(&self) -> &Pid1 {
        &self.pid1
    }
    #[doc = "0xfe8 - Peripheral ID 2"]
    #[inline(always)]
    pub const fn pid2(&self) -> &Pid2 {
        &self.pid2
    }
    #[doc = "0xfec - Peripheral ID 3"]
    #[inline(always)]
    pub const fn pid3(&self) -> &Pid3 {
        &self.pid3
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
#[doc = "SPCSECTRL (rw) register accessor: Secure Privilege Controller Secure Configuration Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spcsectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcsectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spcsectrl`] module"]
#[doc(alias = "SPCSECTRL")]
pub type Spcsectrl = crate::Reg<spcsectrl::SpcsectrlSpec>;
#[doc = "Secure Privilege Controller Secure Configuration Control register"]
pub mod spcsectrl;
#[doc = "BUSWAIT (rw) register accessor: Bus Access wait control after reset\n\nYou can [`read`](crate::Reg::read) this register and get [`buswait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buswait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswait`] module"]
#[doc(alias = "BUSWAIT")]
pub type Buswait = crate::Reg<buswait::BuswaitSpec>;
#[doc = "Bus Access wait control after reset"]
pub mod buswait;
#[doc = "SECRESPCFG (rw) register accessor: Security Violation Response Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`secrespcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secrespcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secrespcfg`] module"]
#[doc(alias = "SECRESPCFG")]
pub type Secrespcfg = crate::Reg<secrespcfg::SecrespcfgSpec>;
#[doc = "Security Violation Response Configuration register"]
pub mod secrespcfg;
#[doc = "NSCCFG (rw) register accessor: Non Secure Callable Configuration for IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`nsccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsccfg`] module"]
#[doc(alias = "NSCCFG")]
pub type Nsccfg = crate::Reg<nsccfg::NsccfgSpec>;
#[doc = "Non Secure Callable Configuration for IDAU"]
pub mod nsccfg;
#[doc = "SECMPCINTSTATUS (r) register accessor: Secure MPC Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secmpcintstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secmpcintstatus`] module"]
#[doc(alias = "SECMPCINTSTATUS")]
pub type Secmpcintstatus = crate::Reg<secmpcintstatus::SecmpcintstatusSpec>;
#[doc = "Secure MPC Interrupt Status"]
pub mod secmpcintstatus;
#[doc = "SECPPCINTSTAT (r) register accessor: Secure PPC Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secppcintstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secppcintstat`] module"]
#[doc(alias = "SECPPCINTSTAT")]
pub type Secppcintstat = crate::Reg<secppcintstat::SecppcintstatSpec>;
#[doc = "Secure PPC Interrupt Status"]
pub mod secppcintstat;
#[doc = "SECPPCINTCLR (w) register accessor: Secure PPC Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secppcintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secppcintclr`] module"]
#[doc(alias = "SECPPCINTCLR")]
pub type Secppcintclr = crate::Reg<secppcintclr::SecppcintclrSpec>;
#[doc = "Secure PPC Interrupt Clear"]
pub mod secppcintclr;
#[doc = "SECPPCINTEN (rw) register accessor: Secure PPC Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`secppcinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secppcinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secppcinten`] module"]
#[doc(alias = "SECPPCINTEN")]
pub type Secppcinten = crate::Reg<secppcinten::SecppcintenSpec>;
#[doc = "Secure PPC Interrupt Enable"]
pub mod secppcinten;
#[doc = "SECMSCINTSTAT (r) register accessor: Secure MSC Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`secmscintstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secmscintstat`] module"]
#[doc(alias = "SECMSCINTSTAT")]
pub type Secmscintstat = crate::Reg<secmscintstat::SecmscintstatSpec>;
#[doc = "Secure MSC Interrupt Status"]
pub mod secmscintstat;
#[doc = "SECMSCINTCLR (rw) register accessor: Secure MSC Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`secmscintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secmscintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secmscintclr`] module"]
#[doc(alias = "SECMSCINTCLR")]
pub type Secmscintclr = crate::Reg<secmscintclr::SecmscintclrSpec>;
#[doc = "Secure MSC Interrupt Clear"]
pub mod secmscintclr;
#[doc = "SECMSCINTEN (rw) register accessor: Secure MSC Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`secmscinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secmscinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secmscinten`] module"]
#[doc(alias = "SECMSCINTEN")]
pub type Secmscinten = crate::Reg<secmscinten::SecmscintenSpec>;
#[doc = "Secure MSC Interrupt Enable"]
pub mod secmscinten;
#[doc = "BRGINTSTAT (r) register accessor: Bridge Buffer Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`brgintstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brgintstat`] module"]
#[doc(alias = "BRGINTSTAT")]
pub type Brgintstat = crate::Reg<brgintstat::BrgintstatSpec>;
#[doc = "Bridge Buffer Error Interrupt Status"]
pub mod brgintstat;
#[doc = "BRGINTCLR (w) register accessor: Bridge Buffer Error Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brgintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brgintclr`] module"]
#[doc(alias = "BRGINTCLR")]
pub type Brgintclr = crate::Reg<brgintclr::BrgintclrSpec>;
#[doc = "Bridge Buffer Error Interrupt Clear"]
pub mod brgintclr;
#[doc = "BRGINTEN (rw) register accessor: Bridge Buffer Error Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`brginten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brginten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brginten`] module"]
#[doc(alias = "BRGINTEN")]
pub type Brginten = crate::Reg<brginten::BrgintenSpec>;
#[doc = "Bridge Buffer Error Interrupt Enable"]
pub mod brginten;
#[doc = "AHBNSPPC0 (rw) register accessor: Non-Secure Access AHB slave Peripheral Protection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnsppc0`] module"]
#[doc(alias = "AHBNSPPC0")]
pub type Ahbnsppc0 = crate::Reg<ahbnsppc0::Ahbnsppc0Spec>;
#[doc = "Non-Secure Access AHB slave Peripheral Protection Control 0"]
pub mod ahbnsppc0;
#[doc = "AHBNSPPCEXP0 (rw) register accessor: Expansion 0 Non_Secure Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppcexp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppcexp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnsppcexp0`] module"]
#[doc(alias = "AHBNSPPCEXP0")]
pub type Ahbnsppcexp0 = crate::Reg<ahbnsppcexp0::Ahbnsppcexp0Spec>;
#[doc = "Expansion 0 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp0;
#[doc = "AHBNSPPCEXP1 (rw) register accessor: Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppcexp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppcexp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnsppcexp1`] module"]
#[doc(alias = "AHBNSPPCEXP1")]
pub type Ahbnsppcexp1 = crate::Reg<ahbnsppcexp1::Ahbnsppcexp1Spec>;
#[doc = "Expansion 1 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp1;
#[doc = "AHBNSPPCEXP2 (rw) register accessor: Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppcexp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppcexp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnsppcexp2`] module"]
#[doc(alias = "AHBNSPPCEXP2")]
pub type Ahbnsppcexp2 = crate::Reg<ahbnsppcexp2::Ahbnsppcexp2Spec>;
#[doc = "Expansion 2 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp2;
#[doc = "AHBNSPPCEXP3 (rw) register accessor: Expansion 3 Non_Secure Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnsppcexp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnsppcexp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnsppcexp3`] module"]
#[doc(alias = "AHBNSPPCEXP3")]
pub type Ahbnsppcexp3 = crate::Reg<ahbnsppcexp3::Ahbnsppcexp3Spec>;
#[doc = "Expansion 3 Non_Secure Access AHB slave Peripheral Protection Control"]
pub mod ahbnsppcexp3;
#[doc = "APBNSPPC0 (rw) register accessor: Non-Secure Access APB slave Peripheral Protection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnsppc0`] module"]
#[doc(alias = "APBNSPPC0")]
pub type Apbnsppc0 = crate::Reg<apbnsppc0::Apbnsppc0Spec>;
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 0"]
pub mod apbnsppc0;
#[doc = "APBNSPPC1 (rw) register accessor: Non-Secure Access APB slave Peripheral Protection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnsppc1`] module"]
#[doc(alias = "APBNSPPC1")]
pub type Apbnsppc1 = crate::Reg<apbnsppc1::Apbnsppc1Spec>;
#[doc = "Non-Secure Access APB slave Peripheral Protection Control 1"]
pub mod apbnsppc1;
#[doc = "APBNSPPCEXP0 (rw) register accessor: Expansion 0 Non_Secure Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppcexp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppcexp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnsppcexp0`] module"]
#[doc(alias = "APBNSPPCEXP0")]
pub type Apbnsppcexp0 = crate::Reg<apbnsppcexp0::Apbnsppcexp0Spec>;
#[doc = "Expansion 0 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp0;
#[doc = "APBNSPPCEXP1 (rw) register accessor: Expansion 1 Non_Secure Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppcexp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppcexp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnsppcexp1`] module"]
#[doc(alias = "APBNSPPCEXP1")]
pub type Apbnsppcexp1 = crate::Reg<apbnsppcexp1::Apbnsppcexp1Spec>;
#[doc = "Expansion 1 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp1;
#[doc = "APBNSPPCEXP2 (rw) register accessor: Expansion 2 Non_Secure Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppcexp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppcexp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnsppcexp2`] module"]
#[doc(alias = "APBNSPPCEXP2")]
pub type Apbnsppcexp2 = crate::Reg<apbnsppcexp2::Apbnsppcexp2Spec>;
#[doc = "Expansion 2 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp2;
#[doc = "APBNSPPCEXP3 (rw) register accessor: Expansion 3 Non_Secure Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnsppcexp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnsppcexp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnsppcexp3`] module"]
#[doc(alias = "APBNSPPCEXP3")]
pub type Apbnsppcexp3 = crate::Reg<apbnsppcexp3::Apbnsppcexp3Spec>;
#[doc = "Expansion 3 Non_Secure Access APB slave Peripheral Protection Control"]
pub mod apbnsppcexp3;
#[doc = "AHBSPPPC0 (r) register accessor: Secure Unprivileged Access AHB slave Peripheral Protection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppc0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbspppc0`] module"]
#[doc(alias = "AHBSPPPC0")]
pub type Ahbspppc0 = crate::Reg<ahbspppc0::Ahbspppc0Spec>;
#[doc = "Secure Unprivileged Access AHB slave Peripheral Protection Control 0"]
pub mod ahbspppc0;
#[doc = "AHBSPPPCEXP0 (rw) register accessor: Expansion 0 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppcexp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbspppcexp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbspppcexp0`] module"]
#[doc(alias = "AHBSPPPCEXP0")]
pub type Ahbspppcexp0 = crate::Reg<ahbspppcexp0::Ahbspppcexp0Spec>;
#[doc = "Expansion 0 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp0;
#[doc = "AHBSPPPCEXP1 (rw) register accessor: Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppcexp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbspppcexp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbspppcexp1`] module"]
#[doc(alias = "AHBSPPPCEXP1")]
pub type Ahbspppcexp1 = crate::Reg<ahbspppcexp1::Ahbspppcexp1Spec>;
#[doc = "Expansion 1 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp1;
#[doc = "AHBSPPPCEXP2 (rw) register accessor: Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppcexp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbspppcexp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbspppcexp2`] module"]
#[doc(alias = "AHBSPPPCEXP2")]
pub type Ahbspppcexp2 = crate::Reg<ahbspppcexp2::Ahbspppcexp2Spec>;
#[doc = "Expansion 2 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp2;
#[doc = "AHBSPPPCEXP3 (rw) register accessor: Expansion 3 Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspppcexp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbspppcexp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbspppcexp3`] module"]
#[doc(alias = "AHBSPPPCEXP3")]
pub type Ahbspppcexp3 = crate::Reg<ahbspppcexp3::Ahbspppcexp3Spec>;
#[doc = "Expansion 3 Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbspppcexp3;
#[doc = "APBSPPPC0 (rw) register accessor: Secure Unprivileged Access APB slave Peripheral Protection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspppc0`] module"]
#[doc(alias = "APBSPPPC0")]
pub type Apbspppc0 = crate::Reg<apbspppc0::Apbspppc0Spec>;
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
pub mod apbspppc0;
#[doc = "APBSPPPC1 (rw) register accessor: Secure Unprivileged Access APB slave Peripheral Protection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspppc1`] module"]
#[doc(alias = "APBSPPPC1")]
pub type Apbspppc1 = crate::Reg<apbspppc1::Apbspppc1Spec>;
#[doc = "Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
pub mod apbspppc1;
#[doc = "APBSPPPCEXP0 (rw) register accessor: Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspppcexp0`] module"]
#[doc(alias = "APBSPPPCEXP0")]
pub type Apbspppcexp0 = crate::Reg<apbspppcexp0::Apbspppcexp0Spec>;
#[doc = "Expansion 0 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp0;
#[doc = "APBSPPPCEXP1 (rw) register accessor: Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspppcexp1`] module"]
#[doc(alias = "APBSPPPCEXP1")]
pub type Apbspppcexp1 = crate::Reg<apbspppcexp1::Apbspppcexp1Spec>;
#[doc = "Expansion 1 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp1;
#[doc = "APBSPPPCEXP2 (rw) register accessor: Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspppcexp2`] module"]
#[doc(alias = "APBSPPPCEXP2")]
pub type Apbspppcexp2 = crate::Reg<apbspppcexp2::Apbspppcexp2Spec>;
#[doc = "Expansion 2 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp2;
#[doc = "APBSPPPCEXP3 (rw) register accessor: Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbspppcexp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbspppcexp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbspppcexp3`] module"]
#[doc(alias = "APBSPPPCEXP3")]
pub type Apbspppcexp3 = crate::Reg<apbspppcexp3::Apbspppcexp3Spec>;
#[doc = "Expansion 3 Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbspppcexp3;
#[doc = "NSMSCEXP (r) register accessor: Expansion MSC Non-Secure Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`nsmscexp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsmscexp`] module"]
#[doc(alias = "NSMSCEXP")]
pub type Nsmscexp = crate::Reg<nsmscexp::NsmscexpSpec>;
#[doc = "Expansion MSC Non-Secure Configuration"]
pub mod nsmscexp;
#[doc = "PID4 (r) register accessor: Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid4`] module"]
#[doc(alias = "PID4")]
pub type Pid4 = crate::Reg<pid4::Pid4Spec>;
#[doc = "Peripheral ID 4"]
pub mod pid4;
#[doc = "PID0 (r) register accessor: Peripheral ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid0`] module"]
#[doc(alias = "PID0")]
pub type Pid0 = crate::Reg<pid0::Pid0Spec>;
#[doc = "Peripheral ID 0"]
pub mod pid0;
#[doc = "PID1 (r) register accessor: Peripheral ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid1`] module"]
#[doc(alias = "PID1")]
pub type Pid1 = crate::Reg<pid1::Pid1Spec>;
#[doc = "Peripheral ID 1"]
pub mod pid1;
#[doc = "PID2 (r) register accessor: Peripheral ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid2`] module"]
#[doc(alias = "PID2")]
pub type Pid2 = crate::Reg<pid2::Pid2Spec>;
#[doc = "Peripheral ID 2"]
pub mod pid2;
#[doc = "PID3 (r) register accessor: Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid3`] module"]
#[doc(alias = "PID3")]
pub type Pid3 = crate::Reg<pid3::Pid3Spec>;
#[doc = "Peripheral ID 3"]
pub mod pid3;
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
