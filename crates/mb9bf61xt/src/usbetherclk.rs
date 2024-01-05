#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uccr: UCCR,
    _reserved1: [u8; 0x03],
    upcr1: UPCR1,
    _reserved2: [u8; 0x03],
    upcr2: UPCR2,
    _reserved3: [u8; 0x03],
    upcr3: UPCR3,
    _reserved4: [u8; 0x03],
    upcr4: UPCR4,
    _reserved5: [u8; 0x03],
    up_str: UP_STR,
    _reserved6: [u8; 0x03],
    upint_enr: UPINT_ENR,
    _reserved7: [u8; 0x03],
    upint_clr: UPINT_CLR,
    _reserved8: [u8; 0x03],
    upint_str: UPINT_STR,
    _reserved9: [u8; 0x03],
    upcr5: UPCR5,
    _reserved10: [u8; 0x03],
    upcr6: UPCR6,
    _reserved11: [u8; 0x03],
    upcr7: UPCR7,
    _reserved12: [u8; 0x03],
    usben0: USBEN0,
    _reserved13: [u8; 0x03],
    usben1: USBEN1,
}
impl RegisterBlock {
    #[doc = "0x00 - USB/Ethernet-PLL Clock Control Register"]
    #[inline(always)]
    pub const fn uccr(&self) -> &UCCR {
        &self.uccr
    }
    #[doc = "0x04 - USB/Ethernet-PLL Control Register 1"]
    #[inline(always)]
    pub const fn upcr1(&self) -> &UPCR1 {
        &self.upcr1
    }
    #[doc = "0x08 - USB/Ethernet-PLL Control Register 2"]
    #[inline(always)]
    pub const fn upcr2(&self) -> &UPCR2 {
        &self.upcr2
    }
    #[doc = "0x0c - USB/Ethernet-PLL Control Register 3"]
    #[inline(always)]
    pub const fn upcr3(&self) -> &UPCR3 {
        &self.upcr3
    }
    #[doc = "0x10 - USB/Ethernet-PLL Control Register 4"]
    #[inline(always)]
    pub const fn upcr4(&self) -> &UPCR4 {
        &self.upcr4
    }
    #[doc = "0x14 - USB/Ethernet-PLL Status Register"]
    #[inline(always)]
    pub const fn up_str(&self) -> &UP_STR {
        &self.up_str
    }
    #[doc = "0x18 - USB/Ethernet-PLL Interrupt Source Enable Register"]
    #[inline(always)]
    pub const fn upint_enr(&self) -> &UPINT_ENR {
        &self.upint_enr
    }
    #[doc = "0x1c - USB/Ethernet-PLL Interrupt Source Clear Register"]
    #[inline(always)]
    pub const fn upint_clr(&self) -> &UPINT_CLR {
        &self.upint_clr
    }
    #[doc = "0x20 - USB/Ethernet-PLL Interrupt Source Status Register"]
    #[inline(always)]
    pub const fn upint_str(&self) -> &UPINT_STR {
        &self.upint_str
    }
    #[doc = "0x24 - USB/Ethernet-PLL Control Register 5"]
    #[inline(always)]
    pub const fn upcr5(&self) -> &UPCR5 {
        &self.upcr5
    }
    #[doc = "0x28 - USB/Ethernet-PLL Setting Register 6"]
    #[inline(always)]
    pub const fn upcr6(&self) -> &UPCR6 {
        &self.upcr6
    }
    #[doc = "0x2c - USB/Ethernet-PLL Setting Register 7"]
    #[inline(always)]
    pub const fn upcr7(&self) -> &UPCR7 {
        &self.upcr7
    }
    #[doc = "0x30 - USB0 Enable Register"]
    #[inline(always)]
    pub const fn usben0(&self) -> &USBEN0 {
        &self.usben0
    }
    #[doc = "0x34 - USB1 Enable Register"]
    #[inline(always)]
    pub const fn usben1(&self) -> &USBEN1 {
        &self.usben1
    }
}
#[doc = "UCCR (rw) register accessor: USB/Ethernet-PLL Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uccr`]
module"]
pub type UCCR = crate::Reg<uccr::UCCR_SPEC>;
#[doc = "USB/Ethernet-PLL Clock Control Register"]
pub mod uccr;
#[doc = "UPCR1 (rw) register accessor: USB/Ethernet-PLL Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr1`]
module"]
pub type UPCR1 = crate::Reg<upcr1::UPCR1_SPEC>;
#[doc = "USB/Ethernet-PLL Control Register 1"]
pub mod upcr1;
#[doc = "UPCR2 (rw) register accessor: USB/Ethernet-PLL Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr2`]
module"]
pub type UPCR2 = crate::Reg<upcr2::UPCR2_SPEC>;
#[doc = "USB/Ethernet-PLL Control Register 2"]
pub mod upcr2;
#[doc = "UPCR3 (rw) register accessor: USB/Ethernet-PLL Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr3`]
module"]
pub type UPCR3 = crate::Reg<upcr3::UPCR3_SPEC>;
#[doc = "USB/Ethernet-PLL Control Register 3"]
pub mod upcr3;
#[doc = "UPCR4 (rw) register accessor: USB/Ethernet-PLL Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr4`]
module"]
pub type UPCR4 = crate::Reg<upcr4::UPCR4_SPEC>;
#[doc = "USB/Ethernet-PLL Control Register 4"]
pub mod upcr4;
#[doc = "UPCR5 (rw) register accessor: USB/Ethernet-PLL Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr5`]
module"]
pub type UPCR5 = crate::Reg<upcr5::UPCR5_SPEC>;
#[doc = "USB/Ethernet-PLL Control Register 5"]
pub mod upcr5;
#[doc = "UPCR6 (rw) register accessor: USB/Ethernet-PLL Setting Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr6`]
module"]
pub type UPCR6 = crate::Reg<upcr6::UPCR6_SPEC>;
#[doc = "USB/Ethernet-PLL Setting Register 6"]
pub mod upcr6;
#[doc = "UPCR7 (rw) register accessor: USB/Ethernet-PLL Setting Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcr7`]
module"]
pub type UPCR7 = crate::Reg<upcr7::UPCR7_SPEC>;
#[doc = "USB/Ethernet-PLL Setting Register 7"]
pub mod upcr7;
#[doc = "UP_STR (r) register accessor: USB/Ethernet-PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`up_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@up_str`]
module"]
pub type UP_STR = crate::Reg<up_str::UP_STR_SPEC>;
#[doc = "USB/Ethernet-PLL Status Register"]
pub mod up_str;
#[doc = "UPINT_ENR (rw) register accessor: USB/Ethernet-PLL Interrupt Source Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upint_enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upint_enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upint_enr`]
module"]
pub type UPINT_ENR = crate::Reg<upint_enr::UPINT_ENR_SPEC>;
#[doc = "USB/Ethernet-PLL Interrupt Source Enable Register"]
pub mod upint_enr;
#[doc = "UPINT_STR (r) register accessor: USB/Ethernet-PLL Interrupt Source Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upint_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upint_str`]
module"]
pub type UPINT_STR = crate::Reg<upint_str::UPINT_STR_SPEC>;
#[doc = "USB/Ethernet-PLL Interrupt Source Status Register"]
pub mod upint_str;
#[doc = "UPINT_CLR (w) register accessor: USB/Ethernet-PLL Interrupt Source Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upint_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upint_clr`]
module"]
pub type UPINT_CLR = crate::Reg<upint_clr::UPINT_CLR_SPEC>;
#[doc = "USB/Ethernet-PLL Interrupt Source Clear Register"]
pub mod upint_clr;
#[doc = "USBEN0 (rw) register accessor: USB0 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usben0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usben0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usben0`]
module"]
pub type USBEN0 = crate::Reg<usben0::USBEN0_SPEC>;
#[doc = "USB0 Enable Register"]
pub mod usben0;
#[doc = "USBEN1 (rw) register accessor: USB1 Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usben1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usben1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usben1`]
module"]
pub type USBEN1 = crate::Reg<usben1::USBEN1_SPEC>;
#[doc = "USB1 Enable Register"]
pub mod usben1;
