#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adsr: ADSR,
    adcr: ADCR,
    _reserved2: [u8; 0x06],
    sfns: SFNS,
    sccr: SCCR,
    _reserved4: [u8; 0x02],
    scfd: SCFD,
    scis2: SCIS2,
    scis3: SCIS3,
    _reserved7: [u8; 0x02],
    scis0: SCIS0,
    scis1: SCIS1,
    _reserved9: [u8; 0x02],
    pfns: PFNS,
    pccr: PCCR,
    _reserved11: [u8; 0x02],
    pcfd: PCFD,
    pcis: PCIS,
    _reserved13: [u8; 0x03],
    cmpcr: CMPCR,
    _reserved14: [u8; 0x01],
    cmpd: CMPD,
    adss2: ADSS2,
    adss3: ADSS3,
    _reserved17: [u8; 0x02],
    adss0: ADSS0,
    adss1: ADSS1,
    _reserved19: [u8; 0x02],
    adst1: ADST1,
    adst0: ADST0,
    _reserved21: [u8; 0x02],
    adct: ADCT,
    _reserved22: [u8; 0x03],
    prtsl: PRTSL,
    sctsl: SCTSL,
    _reserved24: [u8; 0x02],
    adcen: ADCEN,
}
impl RegisterBlock {
    #[doc = "0x00 - A/D Status Register"]
    #[inline(always)]
    pub const fn adsr(&self) -> &ADSR {
        &self.adsr
    }
    #[doc = "0x01 - A/D Control Register"]
    #[inline(always)]
    pub const fn adcr(&self) -> &ADCR {
        &self.adcr
    }
    #[doc = "0x08 - Scan Conversion FIFO Stage Count Setup Register"]
    #[inline(always)]
    pub const fn sfns(&self) -> &SFNS {
        &self.sfns
    }
    #[doc = "0x09 - Scan Conversion Control Register"]
    #[inline(always)]
    pub const fn sccr(&self) -> &SCCR {
        &self.sccr
    }
    #[doc = "0x0c - Scan Conversion FIFO Data Register"]
    #[inline(always)]
    pub const fn scfd(&self) -> &SCFD {
        &self.scfd
    }
    #[doc = "0x10 - Scan Conversion Input Selection Register 2"]
    #[inline(always)]
    pub const fn scis2(&self) -> &SCIS2 {
        &self.scis2
    }
    #[doc = "0x11 - Scan Conversion Input Selection Register 3"]
    #[inline(always)]
    pub const fn scis3(&self) -> &SCIS3 {
        &self.scis3
    }
    #[doc = "0x14 - Scan Conversion Input Selection Register 0"]
    #[inline(always)]
    pub const fn scis0(&self) -> &SCIS0 {
        &self.scis0
    }
    #[doc = "0x15 - Scan Conversion Input Selection Register 1"]
    #[inline(always)]
    pub const fn scis1(&self) -> &SCIS1 {
        &self.scis1
    }
    #[doc = "0x18 - Priority Conversion FIFO Stage Count Setup Register"]
    #[inline(always)]
    pub const fn pfns(&self) -> &PFNS {
        &self.pfns
    }
    #[doc = "0x19 - Priority Conversion Control Register"]
    #[inline(always)]
    pub const fn pccr(&self) -> &PCCR {
        &self.pccr
    }
    #[doc = "0x1c - Priority Conversion FIFO Data Register"]
    #[inline(always)]
    pub const fn pcfd(&self) -> &PCFD {
        &self.pcfd
    }
    #[doc = "0x20 - Priority Conversion Input Selection Register"]
    #[inline(always)]
    pub const fn pcis(&self) -> &PCIS {
        &self.pcis
    }
    #[doc = "0x24 - A/D Comparison Control Register"]
    #[inline(always)]
    pub const fn cmpcr(&self) -> &CMPCR {
        &self.cmpcr
    }
    #[doc = "0x26 - A/D Comparison Value Setup Register"]
    #[inline(always)]
    pub const fn cmpd(&self) -> &CMPD {
        &self.cmpd
    }
    #[doc = "0x28 - Sampling Time Selection Register 2"]
    #[inline(always)]
    pub const fn adss2(&self) -> &ADSS2 {
        &self.adss2
    }
    #[doc = "0x29 - Sampling Time Selection Register 3"]
    #[inline(always)]
    pub const fn adss3(&self) -> &ADSS3 {
        &self.adss3
    }
    #[doc = "0x2c - Sampling Time Selection Register 0"]
    #[inline(always)]
    pub const fn adss0(&self) -> &ADSS0 {
        &self.adss0
    }
    #[doc = "0x2d - Sampling Time Selection Register 1"]
    #[inline(always)]
    pub const fn adss1(&self) -> &ADSS1 {
        &self.adss1
    }
    #[doc = "0x30 - Sampling Time Setup Register 1"]
    #[inline(always)]
    pub const fn adst1(&self) -> &ADST1 {
        &self.adst1
    }
    #[doc = "0x31 - Sampling Time Setup Register 0"]
    #[inline(always)]
    pub const fn adst0(&self) -> &ADST0 {
        &self.adst0
    }
    #[doc = "0x34 - Comparison Time Setup Register"]
    #[inline(always)]
    pub const fn adct(&self) -> &ADCT {
        &self.adct
    }
    #[doc = "0x38 - Priority Conversion Timer Trigger Selection Register"]
    #[inline(always)]
    pub const fn prtsl(&self) -> &PRTSL {
        &self.prtsl
    }
    #[doc = "0x39 - Scan Conversion Timer Trigger Selection Register"]
    #[inline(always)]
    pub const fn sctsl(&self) -> &SCTSL {
        &self.sctsl
    }
    #[doc = "0x3c - A/D Operation Enable Setup Register"]
    #[inline(always)]
    pub const fn adcen(&self) -> &ADCEN {
        &self.adcen
    }
}
#[doc = "ADCR (rw) register accessor: A/D Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr`]
module"]
pub type ADCR = crate::Reg<adcr::ADCR_SPEC>;
#[doc = "A/D Control Register"]
pub mod adcr;
#[doc = "ADSR (rw) register accessor: A/D Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsr`]
module"]
pub type ADSR = crate::Reg<adsr::ADSR_SPEC>;
#[doc = "A/D Status Register"]
pub mod adsr;
#[doc = "SCCR (rw) register accessor: Scan Conversion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr`]
module"]
pub type SCCR = crate::Reg<sccr::SCCR_SPEC>;
#[doc = "Scan Conversion Control Register"]
pub mod sccr;
#[doc = "SFNS (rw) register accessor: Scan Conversion FIFO Stage Count Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfns::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfns::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfns`]
module"]
pub type SFNS = crate::Reg<sfns::SFNS_SPEC>;
#[doc = "Scan Conversion FIFO Stage Count Setup Register"]
pub mod sfns;
#[doc = "SCFD (r) register accessor: Scan Conversion FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfd`]
module"]
pub type SCFD = crate::Reg<scfd::SCFD_SPEC>;
#[doc = "Scan Conversion FIFO Data Register"]
pub mod scfd;
#[doc = "SCIS3 (rw) register accessor: Scan Conversion Input Selection Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scis3`]
module"]
pub type SCIS3 = crate::Reg<scis3::SCIS3_SPEC>;
#[doc = "Scan Conversion Input Selection Register 3"]
pub mod scis3;
#[doc = "SCIS2 (rw) register accessor: Scan Conversion Input Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scis2`]
module"]
pub type SCIS2 = crate::Reg<scis2::SCIS2_SPEC>;
#[doc = "Scan Conversion Input Selection Register 2"]
pub mod scis2;
#[doc = "SCIS1 (rw) register accessor: Scan Conversion Input Selection Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scis1`]
module"]
pub type SCIS1 = crate::Reg<scis1::SCIS1_SPEC>;
#[doc = "Scan Conversion Input Selection Register 1"]
pub mod scis1;
#[doc = "SCIS0 (rw) register accessor: Scan Conversion Input Selection Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scis0`]
module"]
pub type SCIS0 = crate::Reg<scis0::SCIS0_SPEC>;
#[doc = "Scan Conversion Input Selection Register 0"]
pub mod scis0;
#[doc = "PFNS (rw) register accessor: Priority Conversion FIFO Stage Count Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfns::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfns::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfns`]
module"]
pub type PFNS = crate::Reg<pfns::PFNS_SPEC>;
#[doc = "Priority Conversion FIFO Stage Count Setup Register"]
pub mod pfns;
#[doc = "PCCR (rw) register accessor: Priority Conversion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pccr`]
module"]
pub type PCCR = crate::Reg<pccr::PCCR_SPEC>;
#[doc = "Priority Conversion Control Register"]
pub mod pccr;
#[doc = "PCFD (r) register accessor: Priority Conversion FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcfd`]
module"]
pub type PCFD = crate::Reg<pcfd::PCFD_SPEC>;
#[doc = "Priority Conversion FIFO Data Register"]
pub mod pcfd;
#[doc = "PCIS (rw) register accessor: Priority Conversion Input Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcis`]
module"]
pub type PCIS = crate::Reg<pcis::PCIS_SPEC>;
#[doc = "Priority Conversion Input Selection Register"]
pub mod pcis;
#[doc = "CMPCR (rw) register accessor: A/D Comparison Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpcr`]
module"]
pub type CMPCR = crate::Reg<cmpcr::CMPCR_SPEC>;
#[doc = "A/D Comparison Control Register"]
pub mod cmpcr;
#[doc = "CMPD (rw) register accessor: A/D Comparison Value Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpd`]
module"]
pub type CMPD = crate::Reg<cmpd::CMPD_SPEC>;
#[doc = "A/D Comparison Value Setup Register"]
pub mod cmpd;
#[doc = "ADSS3 (rw) register accessor: Sampling Time Selection Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adss3`]
module"]
pub type ADSS3 = crate::Reg<adss3::ADSS3_SPEC>;
#[doc = "Sampling Time Selection Register 3"]
pub mod adss3;
#[doc = "ADSS2 (rw) register accessor: Sampling Time Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adss2`]
module"]
pub type ADSS2 = crate::Reg<adss2::ADSS2_SPEC>;
#[doc = "Sampling Time Selection Register 2"]
pub mod adss2;
#[doc = "ADSS1 (rw) register accessor: Sampling Time Selection Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adss1`]
module"]
pub type ADSS1 = crate::Reg<adss1::ADSS1_SPEC>;
#[doc = "Sampling Time Selection Register 1"]
pub mod adss1;
#[doc = "ADSS0 (rw) register accessor: Sampling Time Selection Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adss0`]
module"]
pub type ADSS0 = crate::Reg<adss0::ADSS0_SPEC>;
#[doc = "Sampling Time Selection Register 0"]
pub mod adss0;
#[doc = "ADST1 (rw) register accessor: Sampling Time Setup Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adst1`]
module"]
pub type ADST1 = crate::Reg<adst1::ADST1_SPEC>;
#[doc = "Sampling Time Setup Register 1"]
pub mod adst1;
#[doc = "ADST0 (rw) register accessor: Sampling Time Setup Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adst0`]
module"]
pub type ADST0 = crate::Reg<adst0::ADST0_SPEC>;
#[doc = "Sampling Time Setup Register 0"]
pub mod adst0;
#[doc = "ADCT (rw) register accessor: Comparison Time Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adct`]
module"]
pub type ADCT = crate::Reg<adct::ADCT_SPEC>;
#[doc = "Comparison Time Setup Register"]
pub mod adct;
#[doc = "PRTSL (rw) register accessor: Priority Conversion Timer Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prtsl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prtsl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prtsl`]
module"]
pub type PRTSL = crate::Reg<prtsl::PRTSL_SPEC>;
#[doc = "Priority Conversion Timer Trigger Selection Register"]
pub mod prtsl;
#[doc = "SCTSL (rw) register accessor: Scan Conversion Timer Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctsl`]
module"]
pub type SCTSL = crate::Reg<sctsl::SCTSL_SPEC>;
#[doc = "Scan Conversion Timer Trigger Selection Register"]
pub mod sctsl;
#[doc = "ADCEN (rw) register accessor: A/D Operation Enable Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcen`]
module"]
pub type ADCEN = crate::Reg<adcen::ADCEN_SPEC>;
#[doc = "A/D Operation Enable Setup Register"]
pub mod adcen;
