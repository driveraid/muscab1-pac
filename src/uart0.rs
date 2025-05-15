#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uartdr: Uartdr,
    uartrsr_uartecr: UartrsrUartecr,
    _reserved2: [u8; 0x10],
    uartrfr: Uartrfr,
    _reserved3: [u8; 0x04],
    uartilpr: Uartilpr,
    uartibrd: Uartibrd,
    uartfbrd: Uartfbrd,
    uartlcr_h: UartlcrH,
    uartcr: Uartcr,
    uartifls: Uartifls,
    uartimsc: Uartimsc,
    uartris: Uartris,
    uartmis: Uartmis,
    uarticr: Uarticr,
    uartdmacr: Uartdmacr,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn uartdr(&self) -> &Uartdr {
        &self.uartdr
    }
    #[doc = "0x04 - Receive status register/error clear register"]
    #[inline(always)]
    pub const fn uartrsr_uartecr(&self) -> &UartrsrUartecr {
        &self.uartrsr_uartecr
    }
    #[doc = "0x18 - Flag register"]
    #[inline(always)]
    pub const fn uartrfr(&self) -> &Uartrfr {
        &self.uartrfr
    }
    #[doc = "0x20 - IrDA low-power counter register"]
    #[inline(always)]
    pub const fn uartilpr(&self) -> &Uartilpr {
        &self.uartilpr
    }
    #[doc = "0x24 - Integer baud rate register"]
    #[inline(always)]
    pub const fn uartibrd(&self) -> &Uartibrd {
        &self.uartibrd
    }
    #[doc = "0x28 - Fractional baud rate register"]
    #[inline(always)]
    pub const fn uartfbrd(&self) -> &Uartfbrd {
        &self.uartfbrd
    }
    #[doc = "0x2c - Line control register"]
    #[inline(always)]
    pub const fn uartlcr_h(&self) -> &UartlcrH {
        &self.uartlcr_h
    }
    #[doc = "0x30 - Control register"]
    #[inline(always)]
    pub const fn uartcr(&self) -> &Uartcr {
        &self.uartcr
    }
    #[doc = "0x34 - Interrupt FIFO level select register"]
    #[inline(always)]
    pub const fn uartifls(&self) -> &Uartifls {
        &self.uartifls
    }
    #[doc = "0x38 - Interrupt mask set/clear register"]
    #[inline(always)]
    pub const fn uartimsc(&self) -> &Uartimsc {
        &self.uartimsc
    }
    #[doc = "0x3c - Raw interrupt status register"]
    #[inline(always)]
    pub const fn uartris(&self) -> &Uartris {
        &self.uartris
    }
    #[doc = "0x40 - Masked interrupt status register"]
    #[inline(always)]
    pub const fn uartmis(&self) -> &Uartmis {
        &self.uartmis
    }
    #[doc = "0x44 - Interrupt clear register"]
    #[inline(always)]
    pub const fn uarticr(&self) -> &Uarticr {
        &self.uarticr
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn uartdmacr(&self) -> &Uartdmacr {
        &self.uartdmacr
    }
}
#[doc = "UARTDR (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdr`] module"]
#[doc(alias = "UARTDR")]
pub type Uartdr = crate::Reg<uartdr::UartdrSpec>;
#[doc = "Data register"]
pub mod uartdr;
#[doc = "UARTRSR_UARTECR (rw) register accessor: Receive status register/error clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrsr_uartecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrsr_uartecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrsr_uartecr`] module"]
#[doc(alias = "UARTRSR_UARTECR")]
pub type UartrsrUartecr = crate::Reg<uartrsr_uartecr::UartrsrUartecrSpec>;
#[doc = "Receive status register/error clear register"]
pub mod uartrsr_uartecr;
#[doc = "UARTRFR (r) register accessor: Flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrfr`] module"]
#[doc(alias = "UARTRFR")]
pub type Uartrfr = crate::Reg<uartrfr::UartrfrSpec>;
#[doc = "Flag register"]
pub mod uartrfr;
#[doc = "UARTILPR (rw) register accessor: IrDA low-power counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartilpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartilpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartilpr`] module"]
#[doc(alias = "UARTILPR")]
pub type Uartilpr = crate::Reg<uartilpr::UartilprSpec>;
#[doc = "IrDA low-power counter register"]
pub mod uartilpr;
#[doc = "UARTIBRD (rw) register accessor: Integer baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartibrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartibrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartibrd`] module"]
#[doc(alias = "UARTIBRD")]
pub type Uartibrd = crate::Reg<uartibrd::UartibrdSpec>;
#[doc = "Integer baud rate register"]
pub mod uartibrd;
#[doc = "UARTFBRD (rw) register accessor: Fractional baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartfbrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartfbrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartfbrd`] module"]
#[doc(alias = "UARTFBRD")]
pub type Uartfbrd = crate::Reg<uartfbrd::UartfbrdSpec>;
#[doc = "Fractional baud rate register"]
pub mod uartfbrd;
#[doc = "UARTLCR_H (rw) register accessor: Line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlcr_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartlcr_h`] module"]
#[doc(alias = "UARTLCR_H")]
pub type UartlcrH = crate::Reg<uartlcr_h::UartlcrHSpec>;
#[doc = "Line control register"]
pub mod uartlcr_h;
#[doc = "UARTCR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartcr`] module"]
#[doc(alias = "UARTCR")]
pub type Uartcr = crate::Reg<uartcr::UartcrSpec>;
#[doc = "Control register"]
pub mod uartcr;
#[doc = "UARTIFLS (rw) register accessor: Interrupt FIFO level select register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartifls::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartifls::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartifls`] module"]
#[doc(alias = "UARTIFLS")]
pub type Uartifls = crate::Reg<uartifls::UartiflsSpec>;
#[doc = "Interrupt FIFO level select register"]
pub mod uartifls;
#[doc = "UARTIMSC (rw) register accessor: Interrupt mask set/clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartimsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartimsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartimsc`] module"]
#[doc(alias = "UARTIMSC")]
pub type Uartimsc = crate::Reg<uartimsc::UartimscSpec>;
#[doc = "Interrupt mask set/clear register"]
pub mod uartimsc;
#[doc = "UARTRIS (r) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartris`] module"]
#[doc(alias = "UARTRIS")]
pub type Uartris = crate::Reg<uartris::UartrisSpec>;
#[doc = "Raw interrupt status register"]
pub mod uartris;
#[doc = "UARTMIS (r) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartmis`] module"]
#[doc(alias = "UARTMIS")]
pub type Uartmis = crate::Reg<uartmis::UartmisSpec>;
#[doc = "Masked interrupt status register"]
pub mod uartmis;
#[doc = "UARTICR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uarticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uarticr`] module"]
#[doc(alias = "UARTICR")]
pub type Uarticr = crate::Reg<uarticr::UarticrSpec>;
#[doc = "Interrupt clear register"]
pub mod uarticr;
#[doc = "UARTDMACR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdmacr`] module"]
#[doc(alias = "UARTDMACR")]
pub type Uartdmacr = crate::Reg<uartdmacr::UartdmacrSpec>;
#[doc = "DMA control register"]
pub mod uartdmacr;
