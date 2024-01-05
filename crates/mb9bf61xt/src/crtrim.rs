#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcr_psr: MCR_PSR,
    _reserved1: [u8; 0x03],
    mcr_ftrm: MCR_FTRM,
    _reserved2: [u8; 0x06],
    mcr_rlr: MCR_RLR,
}
impl RegisterBlock {
    #[doc = "0x00 - High-speed CR oscillation Frequency Division Setup Register"]
    #[inline(always)]
    pub const fn mcr_psr(&self) -> &MCR_PSR {
        &self.mcr_psr
    }
    #[doc = "0x04 - High-speed CR oscillation Frequency Trimming Register"]
    #[inline(always)]
    pub const fn mcr_ftrm(&self) -> &MCR_FTRM {
        &self.mcr_ftrm
    }
    #[doc = "0x0c - High-Speed CR Oscillation Register Write-Protect Register"]
    #[inline(always)]
    pub const fn mcr_rlr(&self) -> &MCR_RLR {
        &self.mcr_rlr
    }
}
#[doc = "MCR_PSR (rw) register accessor: High-speed CR oscillation Frequency Division Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr_psr`]
module"]
pub type MCR_PSR = crate::Reg<mcr_psr::MCR_PSR_SPEC>;
#[doc = "High-speed CR oscillation Frequency Division Setup Register"]
pub mod mcr_psr;
#[doc = "MCR_FTRM (rw) register accessor: High-speed CR oscillation Frequency Trimming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr_ftrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr_ftrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr_ftrm`]
module"]
pub type MCR_FTRM = crate::Reg<mcr_ftrm::MCR_FTRM_SPEC>;
#[doc = "High-speed CR oscillation Frequency Trimming Register"]
pub mod mcr_ftrm;
#[doc = "MCR_RLR (rw) register accessor: High-Speed CR Oscillation Register Write-Protect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr_rlr`]
module"]
pub type MCR_RLR = crate::Reg<mcr_rlr::MCR_RLR_SPEC>;
#[doc = "High-Speed CR Oscillation Register Write-Protect Register"]
pub mod mcr_rlr;
