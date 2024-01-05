#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    crccr: CRCCR,
    _reserved1: [u8; 0x03],
    crcinit: CRCINIT,
    crcin: CRCIN,
    crcr: CRCR,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Control Register"]
    #[inline(always)]
    pub const fn crccr(&self) -> &CRCCR {
        &self.crccr
    }
    #[doc = "0x04 - Initial Value Register"]
    #[inline(always)]
    pub const fn crcinit(&self) -> &CRCINIT {
        &self.crcinit
    }
    #[doc = "0x08 - Input Data Register"]
    #[inline(always)]
    pub const fn crcin(&self) -> &CRCIN {
        &self.crcin
    }
    #[doc = "0x0c - CRC Register"]
    #[inline(always)]
    pub const fn crcr(&self) -> &CRCR {
        &self.crcr
    }
}
#[doc = "CRCCR (rw) register accessor: CRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr`]
module"]
pub type CRCCR = crate::Reg<crccr::CRCCR_SPEC>;
#[doc = "CRC Control Register"]
pub mod crccr;
#[doc = "CRCINIT (rw) register accessor: Initial Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcinit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcinit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinit`]
module"]
pub type CRCINIT = crate::Reg<crcinit::CRCINIT_SPEC>;
#[doc = "Initial Value Register"]
pub mod crcinit;
#[doc = "CRCIN (rw) register accessor: Input Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin`]
module"]
pub type CRCIN = crate::Reg<crcin::CRCIN_SPEC>;
#[doc = "Input Data Register"]
pub mod crcin;
#[doc = "CRCR (r) register accessor: CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcr`]
module"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRC Register"]
pub mod crcr;
