#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x90],
    ahbnspppc0: Ahbnspppc0,
    _reserved1: [u8; 0x0c],
    ahbnspppcexp0: Ahbnspppcexp0,
    ahbnspppcexp1: Ahbnspppcexp1,
    ahbnspppcexp2: Ahbnspppcexp2,
    ahbnspppcexp3: Ahbnspppcexp3,
    apbnspppc0: Apbnspppc0,
    apbnspppc1: Apbnspppc1,
    _reserved7: [u8; 0x08],
    apbnspppcexp0: Apbnspppcexp0,
    apbnspppcexp1: Apbnspppcexp1,
    apbnspppcexp2: Apbnspppcexp2,
    apbnspppcexp3: Apbnspppcexp3,
    _reserved11: [u8; 0x0f00],
    pidr4: Pidr4,
    _reserved12: [u8; 0x0c],
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
    #[doc = "0x90 - Non-Secure Unprivileged Access AHB slave Peripheral Protection Control #0"]
    #[inline(always)]
    pub const fn ahbnspppc0(&self) -> &Ahbnspppc0 {
        &self.ahbnspppc0
    }
    #[doc = "0xa0 - Expansion 0 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnspppcexp0(&self) -> &Ahbnspppcexp0 {
        &self.ahbnspppcexp0
    }
    #[doc = "0xa4 - Expansion 1 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnspppcexp1(&self) -> &Ahbnspppcexp1 {
        &self.ahbnspppcexp1
    }
    #[doc = "0xa8 - Expansion 2 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnspppcexp2(&self) -> &Ahbnspppcexp2 {
        &self.ahbnspppcexp2
    }
    #[doc = "0xac - Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn ahbnspppcexp3(&self) -> &Ahbnspppcexp3 {
        &self.ahbnspppcexp3
    }
    #[doc = "0xb0 - Non-Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
    #[inline(always)]
    pub const fn apbnspppc0(&self) -> &Apbnspppc0 {
        &self.apbnspppc0
    }
    #[doc = "0xb4 - Non-Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
    #[inline(always)]
    pub const fn apbnspppc1(&self) -> &Apbnspppc1 {
        &self.apbnspppc1
    }
    #[doc = "0xc0 - Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnspppcexp0(&self) -> &Apbnspppcexp0 {
        &self.apbnspppcexp0
    }
    #[doc = "0xc4 - Expansion 1 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnspppcexp1(&self) -> &Apbnspppcexp1 {
        &self.apbnspppcexp1
    }
    #[doc = "0xc8 - Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnspppcexp2(&self) -> &Apbnspppcexp2 {
        &self.apbnspppcexp2
    }
    #[doc = "0xcc - Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
    #[inline(always)]
    pub const fn apbnspppcexp3(&self) -> &Apbnspppcexp3 {
        &self.apbnspppcexp3
    }
    #[doc = "0xfd0 - Peripheral ID 4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
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
#[doc = "AHBNSPPPC0 (rw) register accessor: Non-Secure Unprivileged Access AHB slave Peripheral Protection Control #0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnspppc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnspppc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnspppc0`] module"]
#[doc(alias = "AHBNSPPPC0")]
pub type Ahbnspppc0 = crate::Reg<ahbnspppc0::Ahbnspppc0Spec>;
#[doc = "Non-Secure Unprivileged Access AHB slave Peripheral Protection Control #0"]
pub mod ahbnspppc0;
#[doc = "AHBNSPPPCEXP0 (rw) register accessor: Expansion 0 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnspppcexp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnspppcexp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnspppcexp0`] module"]
#[doc(alias = "AHBNSPPPCEXP0")]
pub type Ahbnspppcexp0 = crate::Reg<ahbnspppcexp0::Ahbnspppcexp0Spec>;
#[doc = "Expansion 0 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp0;
#[doc = "AHBNSPPPCEXP1 (rw) register accessor: Expansion 1 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnspppcexp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnspppcexp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnspppcexp1`] module"]
#[doc(alias = "AHBNSPPPCEXP1")]
pub type Ahbnspppcexp1 = crate::Reg<ahbnspppcexp1::Ahbnspppcexp1Spec>;
#[doc = "Expansion 1 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp1;
#[doc = "AHBNSPPPCEXP2 (rw) register accessor: Expansion 2 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnspppcexp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnspppcexp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnspppcexp2`] module"]
#[doc(alias = "AHBNSPPPCEXP2")]
pub type Ahbnspppcexp2 = crate::Reg<ahbnspppcexp2::Ahbnspppcexp2Spec>;
#[doc = "Expansion 2 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp2;
#[doc = "AHBNSPPPCEXP3 (rw) register accessor: Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbnspppcexp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbnspppcexp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbnspppcexp3`] module"]
#[doc(alias = "AHBNSPPPCEXP3")]
pub type Ahbnspppcexp3 = crate::Reg<ahbnspppcexp3::Ahbnspppcexp3Spec>;
#[doc = "Expansion 3 Non_Secure Unprivileged Access AHB slave Peripheral Protection Control"]
pub mod ahbnspppcexp3;
#[doc = "APBNSPPPC0 (rw) register accessor: Non-Secure Unprivileged Access APB slave Peripheral Protection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnspppc0`] module"]
#[doc(alias = "APBNSPPPC0")]
pub type Apbnspppc0 = crate::Reg<apbnspppc0::Apbnspppc0Spec>;
#[doc = "Non-Secure Unprivileged Access APB slave Peripheral Protection Control 0"]
pub mod apbnspppc0;
#[doc = "APBNSPPPC1 (rw) register accessor: Non-Secure Unprivileged Access APB slave Peripheral Protection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnspppc1`] module"]
#[doc(alias = "APBNSPPPC1")]
pub type Apbnspppc1 = crate::Reg<apbnspppc1::Apbnspppc1Spec>;
#[doc = "Non-Secure Unprivileged Access APB slave Peripheral Protection Control 1"]
pub mod apbnspppc1;
#[doc = "APBNSPPPCEXP0 (rw) register accessor: Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnspppcexp0`] module"]
#[doc(alias = "APBNSPPPCEXP0")]
pub type Apbnspppcexp0 = crate::Reg<apbnspppcexp0::Apbnspppcexp0Spec>;
#[doc = "Expansion 0 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp0;
#[doc = "APBNSPPPCEXP1 (rw) register accessor: Expansion 1 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnspppcexp1`] module"]
#[doc(alias = "APBNSPPPCEXP1")]
pub type Apbnspppcexp1 = crate::Reg<apbnspppcexp1::Apbnspppcexp1Spec>;
#[doc = "Expansion 1 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp1;
#[doc = "APBNSPPPCEXP2 (rw) register accessor: Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnspppcexp2`] module"]
#[doc(alias = "APBNSPPPCEXP2")]
pub type Apbnspppcexp2 = crate::Reg<apbnspppcexp2::Apbnspppcexp2Spec>;
#[doc = "Expansion 2 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp2;
#[doc = "APBNSPPPCEXP3 (rw) register accessor: Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`apbnspppcexp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbnspppcexp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbnspppcexp3`] module"]
#[doc(alias = "APBNSPPPCEXP3")]
pub type Apbnspppcexp3 = crate::Reg<apbnspppcexp3::Apbnspppcexp3Spec>;
#[doc = "Expansion 3 Non_Secure Unprivileged Access APB slave Peripheral Protection Control"]
pub mod apbnspppcexp3;
#[doc = "PIDR4 (r) register accessor: Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Peripheral ID 4"]
pub mod pidr4;
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
