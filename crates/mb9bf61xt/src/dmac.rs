#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmacr: DMACR,
    _reserved1: [u8; 0x0c],
    dmaca0: DMACA0,
    dmacb0: DMACB0,
    dmacsa0: DMACSA0,
    dmacda0: DMACDA0,
    dmaca1: DMACA1,
    dmacb1: DMACB1,
    dmacsa1: DMACSA1,
    dmacda1: DMACDA1,
    dmaca2: DMACA2,
    dmacb2: DMACB2,
    dmacsa2: DMACSA2,
    dmacda2: DMACDA2,
    dmaca3: DMACA3,
    dmacb3: DMACB3,
    dmacsa3: DMACSA3,
    dmacda3: DMACDA3,
    dmaca4: DMACA4,
    dmacb4: DMACB4,
    dmacsa4: DMACSA4,
    dmacda4: DMACDA4,
    dmaca5: DMACA5,
    dmacb5: DMACB5,
    dmacsa5: DMACSA5,
    dmacda5: DMACDA5,
    dmaca6: DMACA6,
    dmacb6: DMACB6,
    dmacsa6: DMACSA6,
    dmacda6: DMACDA6,
    dmaca7: DMACA7,
    dmacb7: DMACB7,
    dmacsa7: DMACSA7,
    dmacda7: DMACDA7,
}
impl RegisterBlock {
    #[doc = "0x00 - Entire DMAC Configuration Register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
    #[doc = "0x10 - Configuration A Register"]
    #[inline(always)]
    pub const fn dmaca0(&self) -> &DMACA0 {
        &self.dmaca0
    }
    #[doc = "0x14 - Configuration B Register"]
    #[inline(always)]
    pub const fn dmacb0(&self) -> &DMACB0 {
        &self.dmacb0
    }
    #[doc = "0x18 - Transfer Source Address Register"]
    #[inline(always)]
    pub const fn dmacsa0(&self) -> &DMACSA0 {
        &self.dmacsa0
    }
    #[doc = "0x1c - Transfer Destination Address Register"]
    #[inline(always)]
    pub const fn dmacda0(&self) -> &DMACDA0 {
        &self.dmacda0
    }
    #[doc = "0x20 - Configuration A Register 1"]
    #[inline(always)]
    pub const fn dmaca1(&self) -> &DMACA1 {
        &self.dmaca1
    }
    #[doc = "0x24 - Configuration B Register 1"]
    #[inline(always)]
    pub const fn dmacb1(&self) -> &DMACB1 {
        &self.dmacb1
    }
    #[doc = "0x28 - Transfer Source Address Register 1"]
    #[inline(always)]
    pub const fn dmacsa1(&self) -> &DMACSA1 {
        &self.dmacsa1
    }
    #[doc = "0x2c - Transfer Destination Address Register 1"]
    #[inline(always)]
    pub const fn dmacda1(&self) -> &DMACDA1 {
        &self.dmacda1
    }
    #[doc = "0x30 - Configuration A Register 2"]
    #[inline(always)]
    pub const fn dmaca2(&self) -> &DMACA2 {
        &self.dmaca2
    }
    #[doc = "0x34 - Configuration B Register 2"]
    #[inline(always)]
    pub const fn dmacb2(&self) -> &DMACB2 {
        &self.dmacb2
    }
    #[doc = "0x38 - Transfer Source Address Register 2"]
    #[inline(always)]
    pub const fn dmacsa2(&self) -> &DMACSA2 {
        &self.dmacsa2
    }
    #[doc = "0x3c - Transfer Destination Address Register 2"]
    #[inline(always)]
    pub const fn dmacda2(&self) -> &DMACDA2 {
        &self.dmacda2
    }
    #[doc = "0x40 - Configuration A Register 3"]
    #[inline(always)]
    pub const fn dmaca3(&self) -> &DMACA3 {
        &self.dmaca3
    }
    #[doc = "0x44 - Configuration B Register 3"]
    #[inline(always)]
    pub const fn dmacb3(&self) -> &DMACB3 {
        &self.dmacb3
    }
    #[doc = "0x48 - Transfer Source Address Register 3"]
    #[inline(always)]
    pub const fn dmacsa3(&self) -> &DMACSA3 {
        &self.dmacsa3
    }
    #[doc = "0x4c - Transfer Destination Address Register 3"]
    #[inline(always)]
    pub const fn dmacda3(&self) -> &DMACDA3 {
        &self.dmacda3
    }
    #[doc = "0x50 - Configuration A Register 4"]
    #[inline(always)]
    pub const fn dmaca4(&self) -> &DMACA4 {
        &self.dmaca4
    }
    #[doc = "0x54 - Configuration B Register 4"]
    #[inline(always)]
    pub const fn dmacb4(&self) -> &DMACB4 {
        &self.dmacb4
    }
    #[doc = "0x58 - Transfer Source Address Register 4"]
    #[inline(always)]
    pub const fn dmacsa4(&self) -> &DMACSA4 {
        &self.dmacsa4
    }
    #[doc = "0x5c - Transfer Destination Address Register 4"]
    #[inline(always)]
    pub const fn dmacda4(&self) -> &DMACDA4 {
        &self.dmacda4
    }
    #[doc = "0x60 - Configuration A Register 5"]
    #[inline(always)]
    pub const fn dmaca5(&self) -> &DMACA5 {
        &self.dmaca5
    }
    #[doc = "0x64 - Configuration B Register 5"]
    #[inline(always)]
    pub const fn dmacb5(&self) -> &DMACB5 {
        &self.dmacb5
    }
    #[doc = "0x68 - Transfer Source Address Register 5"]
    #[inline(always)]
    pub const fn dmacsa5(&self) -> &DMACSA5 {
        &self.dmacsa5
    }
    #[doc = "0x6c - Transfer Destination Address Register 5"]
    #[inline(always)]
    pub const fn dmacda5(&self) -> &DMACDA5 {
        &self.dmacda5
    }
    #[doc = "0x70 - Configuration A Register 6"]
    #[inline(always)]
    pub const fn dmaca6(&self) -> &DMACA6 {
        &self.dmaca6
    }
    #[doc = "0x74 - Configuration B Register 6"]
    #[inline(always)]
    pub const fn dmacb6(&self) -> &DMACB6 {
        &self.dmacb6
    }
    #[doc = "0x78 - Transfer Source Address Register 6"]
    #[inline(always)]
    pub const fn dmacsa6(&self) -> &DMACSA6 {
        &self.dmacsa6
    }
    #[doc = "0x7c - Transfer Destination Address Register 6"]
    #[inline(always)]
    pub const fn dmacda6(&self) -> &DMACDA6 {
        &self.dmacda6
    }
    #[doc = "0x80 - Configuration A Register 7"]
    #[inline(always)]
    pub const fn dmaca7(&self) -> &DMACA7 {
        &self.dmaca7
    }
    #[doc = "0x84 - Configuration B Register 7"]
    #[inline(always)]
    pub const fn dmacb7(&self) -> &DMACB7 {
        &self.dmacb7
    }
    #[doc = "0x88 - Transfer Source Address Register 7"]
    #[inline(always)]
    pub const fn dmacsa7(&self) -> &DMACSA7 {
        &self.dmacsa7
    }
    #[doc = "0x8c - Transfer Destination Address Register 7"]
    #[inline(always)]
    pub const fn dmacda7(&self) -> &DMACDA7 {
        &self.dmacda7
    }
}
#[doc = "DMACR (rw) register accessor: Entire DMAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "Entire DMAC Configuration Register"]
pub mod dmacr;
#[doc = "DMACA0 (rw) register accessor: Configuration A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaca0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaca0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaca0`]
module"]
pub type DMACA0 = crate::Reg<dmaca0::DMACA0_SPEC>;
#[doc = "Configuration A Register"]
pub mod dmaca0;
#[doc = "DMACB0 (rw) register accessor: Configuration B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacb0`]
module"]
pub type DMACB0 = crate::Reg<dmacb0::DMACB0_SPEC>;
#[doc = "Configuration B Register"]
pub mod dmacb0;
#[doc = "DMACSA0 (rw) register accessor: Transfer Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacsa0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacsa0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsa0`]
module"]
pub type DMACSA0 = crate::Reg<dmacsa0::DMACSA0_SPEC>;
#[doc = "Transfer Source Address Register"]
pub mod dmacsa0;
#[doc = "DMACDA0 (rw) register accessor: Transfer Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacda0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacda0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacda0`]
module"]
pub type DMACDA0 = crate::Reg<dmacda0::DMACDA0_SPEC>;
#[doc = "Transfer Destination Address Register"]
pub mod dmacda0;
pub use dmaca0 as dmaca1;
pub use dmaca0 as dmaca2;
pub use dmaca0 as dmaca3;
pub use dmaca0 as dmaca4;
pub use dmaca0 as dmaca5;
pub use dmaca0 as dmaca6;
pub use dmaca0 as dmaca7;
pub use dmacb0 as dmacb1;
pub use dmacb0 as dmacb2;
pub use dmacb0 as dmacb3;
pub use dmacb0 as dmacb4;
pub use dmacb0 as dmacb5;
pub use dmacb0 as dmacb6;
pub use dmacb0 as dmacb7;
pub use dmacda0 as dmacda1;
pub use dmacda0 as dmacda2;
pub use dmacda0 as dmacda3;
pub use dmacda0 as dmacda4;
pub use dmacda0 as dmacda5;
pub use dmacda0 as dmacda6;
pub use dmacda0 as dmacda7;
pub use dmacsa0 as dmacsa1;
pub use dmacsa0 as dmacsa2;
pub use dmacsa0 as dmacsa3;
pub use dmacsa0 as dmacsa4;
pub use dmacsa0 as dmacsa5;
pub use dmacsa0 as dmacsa6;
pub use dmacsa0 as dmacsa7;
pub use DMACA0 as DMACA1;
pub use DMACA0 as DMACA2;
pub use DMACA0 as DMACA3;
pub use DMACA0 as DMACA4;
pub use DMACA0 as DMACA5;
pub use DMACA0 as DMACA6;
pub use DMACA0 as DMACA7;
pub use DMACB0 as DMACB1;
pub use DMACB0 as DMACB2;
pub use DMACB0 as DMACB3;
pub use DMACB0 as DMACB4;
pub use DMACB0 as DMACB5;
pub use DMACB0 as DMACB6;
pub use DMACB0 as DMACB7;
pub use DMACDA0 as DMACDA1;
pub use DMACDA0 as DMACDA2;
pub use DMACDA0 as DMACDA3;
pub use DMACDA0 as DMACDA4;
pub use DMACDA0 as DMACDA5;
pub use DMACDA0 as DMACDA6;
pub use DMACDA0 as DMACDA7;
pub use DMACSA0 as DMACSA1;
pub use DMACSA0 as DMACSA2;
pub use DMACSA0 as DMACSA3;
pub use DMACSA0 as DMACSA4;
pub use DMACSA0 as DMACSA5;
pub use DMACSA0 as DMACSA6;
pub use DMACSA0 as DMACSA7;
