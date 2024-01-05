#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    qpcr: QPCR,
    _reserved1: [u8; 0x02],
    qrcr: QRCR,
    _reserved2: [u8; 0x02],
    qpccr: QPCCR,
    _reserved3: [u8; 0x02],
    qprcr: QPRCR,
    _reserved4: [u8; 0x02],
    qmpr: QMPR,
    _reserved5: [u8; 0x02],
    qicrl: QICRL,
    qicrh: QICRH,
    _reserved7: [u8; 0x02],
    qcr: QCR,
    _reserved8: [u8; 0x02],
    qecr: QECR,
    _reserved9: [u8; 0x1e],
    qrcrr: QRCRR,
    qpcrr: QPCRR,
}
impl RegisterBlock {
    #[doc = "0x00 - QPRC Position Count Register"]
    #[inline(always)]
    pub const fn qpcr(&self) -> &QPCR {
        &self.qpcr
    }
    #[doc = "0x04 - QPRC Revolution Count Register"]
    #[inline(always)]
    pub const fn qrcr(&self) -> &QRCR {
        &self.qrcr
    }
    #[doc = "0x08 - QPRC Position Counter Compare Register"]
    #[inline(always)]
    pub const fn qpccr(&self) -> &QPCCR {
        &self.qpccr
    }
    #[doc = "0x0c - QPRC Position and Revolution Counter Compare Register"]
    #[inline(always)]
    pub const fn qprcr(&self) -> &QPRCR {
        &self.qprcr
    }
    #[doc = "0x10 - QPRC Maximum Position Register"]
    #[inline(always)]
    pub const fn qmpr(&self) -> &QMPR {
        &self.qmpr
    }
    #[doc = "0x14 - Low-Order Bytes of QPRC Interrupt Control Register"]
    #[inline(always)]
    pub const fn qicrl(&self) -> &QICRL {
        &self.qicrl
    }
    #[doc = "0x15 - High-Order Bytes of QPRC Interrupt Control Register"]
    #[inline(always)]
    pub const fn qicrh(&self) -> &QICRH {
        &self.qicrh
    }
    #[doc = "0x18 - QPRC Control Register"]
    #[inline(always)]
    pub const fn qcr(&self) -> &QCR {
        &self.qcr
    }
    #[doc = "0x1c - QPRC Extension Control Register"]
    #[inline(always)]
    pub const fn qecr(&self) -> &QECR {
        &self.qecr
    }
    #[doc = "0x3c - Quad counter rotation count Register"]
    #[inline(always)]
    pub const fn qrcrr(&self) -> &QRCRR {
        &self.qrcrr
    }
    #[doc = "0x3e - Quad counter position count Register"]
    #[inline(always)]
    pub const fn qpcrr(&self) -> &QPCRR {
        &self.qpcrr
    }
}
#[doc = "QPCR (rw) register accessor: QPRC Position Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qpcr`]
module"]
pub type QPCR = crate::Reg<qpcr::QPCR_SPEC>;
#[doc = "QPRC Position Count Register"]
pub mod qpcr;
#[doc = "QRCR (rw) register accessor: QPRC Revolution Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qrcr`]
module"]
pub type QRCR = crate::Reg<qrcr::QRCR_SPEC>;
#[doc = "QPRC Revolution Count Register"]
pub mod qrcr;
#[doc = "QPCCR (rw) register accessor: QPRC Position Counter Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qpccr`]
module"]
pub type QPCCR = crate::Reg<qpccr::QPCCR_SPEC>;
#[doc = "QPRC Position Counter Compare Register"]
pub mod qpccr;
#[doc = "QPRCR (rw) register accessor: QPRC Position and Revolution Counter Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qprcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qprcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qprcr`]
module"]
pub type QPRCR = crate::Reg<qprcr::QPRCR_SPEC>;
#[doc = "QPRC Position and Revolution Counter Compare Register"]
pub mod qprcr;
#[doc = "QCR (rw) register accessor: QPRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcr`]
module"]
pub type QCR = crate::Reg<qcr::QCR_SPEC>;
#[doc = "QPRC Control Register"]
pub mod qcr;
#[doc = "QECR (rw) register accessor: QPRC Extension Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qecr`]
module"]
pub type QECR = crate::Reg<qecr::QECR_SPEC>;
#[doc = "QPRC Extension Control Register"]
pub mod qecr;
#[doc = "QICRL (rw) register accessor: Low-Order Bytes of QPRC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qicrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qicrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qicrl`]
module"]
pub type QICRL = crate::Reg<qicrl::QICRL_SPEC>;
#[doc = "Low-Order Bytes of QPRC Interrupt Control Register"]
pub mod qicrl;
#[doc = "QICRH (rw) register accessor: High-Order Bytes of QPRC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qicrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qicrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qicrh`]
module"]
pub type QICRH = crate::Reg<qicrh::QICRH_SPEC>;
#[doc = "High-Order Bytes of QPRC Interrupt Control Register"]
pub mod qicrh;
#[doc = "QMPR (rw) register accessor: QPRC Maximum Position Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qmpr`]
module"]
pub type QMPR = crate::Reg<qmpr::QMPR_SPEC>;
#[doc = "QPRC Maximum Position Register"]
pub mod qmpr;
#[doc = "QRCRR (rw) register accessor: Quad counter rotation count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qrcrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qrcrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qrcrr`]
module"]
pub type QRCRR = crate::Reg<qrcrr::QRCRR_SPEC>;
#[doc = "Quad counter rotation count Register"]
pub mod qrcrr;
#[doc = "QPCRR (rw) register accessor: Quad counter position count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qpcrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qpcrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qpcrr`]
module"]
pub type QPCRR = crate::Reg<qpcrr::QPCRR_SPEC>;
#[doc = "Quad counter position count Register"]
pub mod qpcrr;
