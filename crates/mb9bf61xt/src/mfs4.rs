#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_i2c_i2c_smr: [u8; 0x01],
    _reserved_1_lin_lin_scr: [u8; 0x01],
    _reserved2: [u8; 0x02],
    _reserved_2_i2c_i2c_ibsr: [u8; 0x01],
    _reserved_3_i2c_i2c_ssr: [u8; 0x01],
    _reserved4: [u8; 0x02],
    _reserved_4_i2c_i2c_rdr: [u8; 0x02],
    _reserved5: [u8; 0x02],
    _reserved_5_i2c_i2c_bgr: [u8; 0x02],
    _reserved6: [u8; 0x02],
    i2c_i2c_isba: I2C_I2C_ISBA,
    i2c_i2c_ismk: I2C_I2C_ISMK,
    _reserved8: [u8; 0x02],
    _reserved_8_i2c_i2c_fcr0: [u8; 0x01],
    _reserved_9_i2c_i2c_fcr1: [u8; 0x01],
    _reserved10: [u8; 0x02],
    _reserved_10_i2c_i2c_fbyte1: [u8; 0x01],
    _reserved_11_i2c_i2c_fbyte2: [u8; 0x01],
}
impl RegisterBlock {
    #[doc = "0x00 - Serial Mode Register"]
    #[inline(always)]
    pub const fn i2c_i2c_smr(&self) -> &I2C_I2C_SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Serial Mode Register"]
    #[inline(always)]
    pub const fn lin_lin_smr(&self) -> &LIN_LIN_SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Serial Mode Register"]
    #[inline(always)]
    pub const fn csio_csio_smr(&self) -> &CSIO_CSIO_SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Serial Mode Register"]
    #[inline(always)]
    pub const fn uart_uart_smr(&self) -> &UART_UART_SMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x01 - I2C Bus Control Register"]
    #[inline(always)]
    pub const fn i2c_i2c_ibcr(&self) -> &I2C_I2C_IBCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x01 - Serial Control Register"]
    #[inline(always)]
    pub const fn lin_lin_scr(&self) -> &LIN_LIN_SCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x01 - Serial Control Register"]
    #[inline(always)]
    pub const fn csio_csio_scr(&self) -> &CSIO_CSIO_SCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x01 - Serial Control Register"]
    #[inline(always)]
    pub const fn uart_uart_scr(&self) -> &UART_UART_SCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x04 - I2C Bus Status Register"]
    #[inline(always)]
    pub const fn i2c_i2c_ibsr(&self) -> &I2C_I2C_IBSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Extended Communication Control Register"]
    #[inline(always)]
    pub const fn lin_lin_escr(&self) -> &LIN_LIN_ESCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Extended Communication Control Register"]
    #[inline(always)]
    pub const fn csio_csio_escr(&self) -> &CSIO_CSIO_ESCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Extended Communication Control Register"]
    #[inline(always)]
    pub const fn uart_uart_escr(&self) -> &UART_UART_ESCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x05 - Serial Status Register"]
    #[inline(always)]
    pub const fn i2c_i2c_ssr(&self) -> &I2C_I2C_SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x05 - Serial Status Register"]
    #[inline(always)]
    pub const fn lin_lin_ssr(&self) -> &LIN_LIN_SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x05 - Serial Status Register"]
    #[inline(always)]
    pub const fn csio_csio_ssr(&self) -> &CSIO_CSIO_SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x05 - Serial Status Register"]
    #[inline(always)]
    pub const fn uart_uart_ssr(&self) -> &UART_UART_SSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x08 - Transmit Data Register"]
    #[inline(always)]
    pub const fn i2c_i2c_tdr(&self) -> &I2C_I2C_TDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Received Data Register"]
    #[inline(always)]
    pub const fn i2c_i2c_rdr(&self) -> &I2C_I2C_RDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Transmit Data Register"]
    #[inline(always)]
    pub const fn lin_lin_tdr(&self) -> &LIN_LIN_TDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Received Data Register"]
    #[inline(always)]
    pub const fn lin_lin_rdr(&self) -> &LIN_LIN_RDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Transmit Data Register"]
    #[inline(always)]
    pub const fn csio_csio_tdr(&self) -> &CSIO_CSIO_TDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Received Data Register"]
    #[inline(always)]
    pub const fn csio_csio_rdr(&self) -> &CSIO_CSIO_RDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Transmit Data Register"]
    #[inline(always)]
    pub const fn uart_uart_tdr(&self) -> &UART_UART_TDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Received Data Register"]
    #[inline(always)]
    pub const fn uart_uart_rdr(&self) -> &UART_UART_RDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Baud Rate Generator Registers"]
    #[inline(always)]
    pub const fn i2c_i2c_bgr(&self) -> &I2C_I2C_BGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Baud Rate Generator Registers"]
    #[inline(always)]
    pub const fn lin_lin_bgr(&self) -> &LIN_LIN_BGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Baud Rate Generator Registers"]
    #[inline(always)]
    pub const fn csio_csio_bgr(&self) -> &CSIO_CSIO_BGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Baud Rate Generator Registers"]
    #[inline(always)]
    pub const fn uart_uart_bgr(&self) -> &UART_UART_BGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - 7-bit Slave Address Register"]
    #[inline(always)]
    pub const fn i2c_i2c_isba(&self) -> &I2C_I2C_ISBA {
        &self.i2c_i2c_isba
    }
    #[doc = "0x11 - 7-bit Slave Address Mask Register"]
    #[inline(always)]
    pub const fn i2c_i2c_ismk(&self) -> &I2C_I2C_ISMK {
        &self.i2c_i2c_ismk
    }
    #[doc = "0x14 - FIFO Control Register 0"]
    #[inline(always)]
    pub const fn i2c_i2c_fcr0(&self) -> &I2C_I2C_FCR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - FIFO Control Register 0"]
    #[inline(always)]
    pub const fn lin_lin_fcr0(&self) -> &LIN_LIN_FCR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - FIFO Control Register 0"]
    #[inline(always)]
    pub const fn csio_csio_fcr0(&self) -> &CSIO_CSIO_FCR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - FIFO Control Register 0"]
    #[inline(always)]
    pub const fn uart_uart_fcr0(&self) -> &UART_UART_FCR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x15 - FIFO Control Register 1"]
    #[inline(always)]
    pub const fn i2c_i2c_fcr1(&self) -> &I2C_I2C_FCR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x15 - FIFO Control Register 1"]
    #[inline(always)]
    pub const fn lin_lin_fcr1(&self) -> &LIN_LIN_FCR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x15 - FIFO Control Register 1"]
    #[inline(always)]
    pub const fn csio_csio_fcr1(&self) -> &CSIO_CSIO_FCR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x15 - FIFO Control Register 1"]
    #[inline(always)]
    pub const fn uart_uart_fcr1(&self) -> &UART_UART_FCR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(21).cast() }
    }
    #[doc = "0x18 - FIFO Byte Register 1"]
    #[inline(always)]
    pub const fn i2c_i2c_fbyte1(&self) -> &I2C_I2C_FBYTE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - FIFO Byte Register 1"]
    #[inline(always)]
    pub const fn lin_lin_fbyte1(&self) -> &LIN_LIN_FBYTE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - FIFO Byte Register 1"]
    #[inline(always)]
    pub const fn csio_csio_fbyte1(&self) -> &CSIO_CSIO_FBYTE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - FIFO Byte Register 1"]
    #[inline(always)]
    pub const fn uart_uart_fbyte1(&self) -> &UART_UART_FBYTE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x19 - FIFO Byte Register 2"]
    #[inline(always)]
    pub const fn i2c_i2c_fbyte2(&self) -> &I2C_I2C_FBYTE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(25).cast() }
    }
    #[doc = "0x19 - FIFO Byte Register 2"]
    #[inline(always)]
    pub const fn lin_lin_fbyte2(&self) -> &LIN_LIN_FBYTE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(25).cast() }
    }
    #[doc = "0x19 - FIFO Byte Register 2"]
    #[inline(always)]
    pub const fn csio_csio_fbyte2(&self) -> &CSIO_CSIO_FBYTE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(25).cast() }
    }
    #[doc = "0x19 - FIFO Byte Register 2"]
    #[inline(always)]
    pub const fn uart_uart_fbyte2(&self) -> &UART_UART_FBYTE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(25).cast() }
    }
}
#[doc = "UART_UART_SCR (rw) register accessor: Serial Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_scr`]
module"]
pub type UART_UART_SCR = crate::Reg<uart_uart_scr::UART_UART_SCR_SPEC>;
#[doc = "Serial Control Register"]
pub mod uart_uart_scr;
#[doc = "UART_UART_SMR (rw) register accessor: Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_smr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_smr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_smr`]
module"]
pub type UART_UART_SMR = crate::Reg<uart_uart_smr::UART_UART_SMR_SPEC>;
#[doc = "Serial Mode Register"]
pub mod uart_uart_smr;
#[doc = "UART_UART_SSR (rw) register accessor: Serial Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_ssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_ssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_ssr`]
module"]
pub type UART_UART_SSR = crate::Reg<uart_uart_ssr::UART_UART_SSR_SPEC>;
#[doc = "Serial Status Register"]
pub mod uart_uart_ssr;
#[doc = "UART_UART_ESCR (rw) register accessor: Extended Communication Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_escr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_escr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_escr`]
module"]
pub type UART_UART_ESCR = crate::Reg<uart_uart_escr::UART_UART_ESCR_SPEC>;
#[doc = "Extended Communication Control Register"]
pub mod uart_uart_escr;
#[doc = "UART_UART_RDR (r) register accessor: Received Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_rdr`]
module"]
pub type UART_UART_RDR = crate::Reg<uart_uart_rdr::UART_UART_RDR_SPEC>;
#[doc = "Received Data Register"]
pub mod uart_uart_rdr;
#[doc = "UART_UART_TDR (w) register accessor: Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_tdr`]
module"]
pub type UART_UART_TDR = crate::Reg<uart_uart_tdr::UART_UART_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod uart_uart_tdr;
#[doc = "UART_UART_BGR (rw) register accessor: Baud Rate Generator Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_bgr`]
module"]
pub type UART_UART_BGR = crate::Reg<uart_uart_bgr::UART_UART_BGR_SPEC>;
#[doc = "Baud Rate Generator Registers"]
pub mod uart_uart_bgr;
#[doc = "UART_UART_FCR1 (rw) register accessor: FIFO Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_fcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_fcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_fcr1`]
module"]
pub type UART_UART_FCR1 = crate::Reg<uart_uart_fcr1::UART_UART_FCR1_SPEC>;
#[doc = "FIFO Control Register 1"]
pub mod uart_uart_fcr1;
#[doc = "UART_UART_FCR0 (rw) register accessor: FIFO Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_fcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_fcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_fcr0`]
module"]
pub type UART_UART_FCR0 = crate::Reg<uart_uart_fcr0::UART_UART_FCR0_SPEC>;
#[doc = "FIFO Control Register 0"]
pub mod uart_uart_fcr0;
#[doc = "UART_UART_FBYTE1 (rw) register accessor: FIFO Byte Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_fbyte1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_fbyte1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_fbyte1`]
module"]
pub type UART_UART_FBYTE1 = crate::Reg<uart_uart_fbyte1::UART_UART_FBYTE1_SPEC>;
#[doc = "FIFO Byte Register 1"]
pub mod uart_uart_fbyte1;
#[doc = "UART_UART_FBYTE2 (rw) register accessor: FIFO Byte Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_fbyte2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_fbyte2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_uart_fbyte2`]
module"]
pub type UART_UART_FBYTE2 = crate::Reg<uart_uart_fbyte2::UART_UART_FBYTE2_SPEC>;
#[doc = "FIFO Byte Register 2"]
pub mod uart_uart_fbyte2;
#[doc = "CSIO_CSIO_SCR (rw) register accessor: Serial Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_scr`]
module"]
pub type CSIO_CSIO_SCR = crate::Reg<csio_csio_scr::CSIO_CSIO_SCR_SPEC>;
#[doc = "Serial Control Register"]
pub mod csio_csio_scr;
#[doc = "CSIO_CSIO_SMR (rw) register accessor: Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_smr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_smr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_smr`]
module"]
pub type CSIO_CSIO_SMR = crate::Reg<csio_csio_smr::CSIO_CSIO_SMR_SPEC>;
#[doc = "Serial Mode Register"]
pub mod csio_csio_smr;
#[doc = "CSIO_CSIO_SSR (rw) register accessor: Serial Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_ssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_ssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_ssr`]
module"]
pub type CSIO_CSIO_SSR = crate::Reg<csio_csio_ssr::CSIO_CSIO_SSR_SPEC>;
#[doc = "Serial Status Register"]
pub mod csio_csio_ssr;
#[doc = "CSIO_CSIO_ESCR (rw) register accessor: Extended Communication Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_escr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_escr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_escr`]
module"]
pub type CSIO_CSIO_ESCR = crate::Reg<csio_csio_escr::CSIO_CSIO_ESCR_SPEC>;
#[doc = "Extended Communication Control Register"]
pub mod csio_csio_escr;
#[doc = "CSIO_CSIO_RDR (r) register accessor: Received Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_rdr`]
module"]
pub type CSIO_CSIO_RDR = crate::Reg<csio_csio_rdr::CSIO_CSIO_RDR_SPEC>;
#[doc = "Received Data Register"]
pub mod csio_csio_rdr;
#[doc = "CSIO_CSIO_TDR (w) register accessor: Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_tdr`]
module"]
pub type CSIO_CSIO_TDR = crate::Reg<csio_csio_tdr::CSIO_CSIO_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod csio_csio_tdr;
#[doc = "CSIO_CSIO_BGR (rw) register accessor: Baud Rate Generator Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_bgr`]
module"]
pub type CSIO_CSIO_BGR = crate::Reg<csio_csio_bgr::CSIO_CSIO_BGR_SPEC>;
#[doc = "Baud Rate Generator Registers"]
pub mod csio_csio_bgr;
#[doc = "CSIO_CSIO_FCR1 (rw) register accessor: FIFO Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_fcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_fcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_fcr1`]
module"]
pub type CSIO_CSIO_FCR1 = crate::Reg<csio_csio_fcr1::CSIO_CSIO_FCR1_SPEC>;
#[doc = "FIFO Control Register 1"]
pub mod csio_csio_fcr1;
#[doc = "CSIO_CSIO_FCR0 (rw) register accessor: FIFO Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_fcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_fcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_fcr0`]
module"]
pub type CSIO_CSIO_FCR0 = crate::Reg<csio_csio_fcr0::CSIO_CSIO_FCR0_SPEC>;
#[doc = "FIFO Control Register 0"]
pub mod csio_csio_fcr0;
#[doc = "CSIO_CSIO_FBYTE1 (rw) register accessor: FIFO Byte Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_fbyte1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_fbyte1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_fbyte1`]
module"]
pub type CSIO_CSIO_FBYTE1 = crate::Reg<csio_csio_fbyte1::CSIO_CSIO_FBYTE1_SPEC>;
#[doc = "FIFO Byte Register 1"]
pub mod csio_csio_fbyte1;
#[doc = "CSIO_CSIO_FBYTE2 (rw) register accessor: FIFO Byte Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_fbyte2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_fbyte2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csio_csio_fbyte2`]
module"]
pub type CSIO_CSIO_FBYTE2 = crate::Reg<csio_csio_fbyte2::CSIO_CSIO_FBYTE2_SPEC>;
#[doc = "FIFO Byte Register 2"]
pub mod csio_csio_fbyte2;
#[doc = "LIN_LIN_SCR (rw) register accessor: Serial Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_scr`]
module"]
pub type LIN_LIN_SCR = crate::Reg<lin_lin_scr::LIN_LIN_SCR_SPEC>;
#[doc = "Serial Control Register"]
pub mod lin_lin_scr;
#[doc = "LIN_LIN_SMR (rw) register accessor: Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_smr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_smr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_smr`]
module"]
pub type LIN_LIN_SMR = crate::Reg<lin_lin_smr::LIN_LIN_SMR_SPEC>;
#[doc = "Serial Mode Register"]
pub mod lin_lin_smr;
#[doc = "LIN_LIN_SSR (rw) register accessor: Serial Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_ssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_ssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_ssr`]
module"]
pub type LIN_LIN_SSR = crate::Reg<lin_lin_ssr::LIN_LIN_SSR_SPEC>;
#[doc = "Serial Status Register"]
pub mod lin_lin_ssr;
#[doc = "LIN_LIN_ESCR (rw) register accessor: Extended Communication Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_escr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_escr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_escr`]
module"]
pub type LIN_LIN_ESCR = crate::Reg<lin_lin_escr::LIN_LIN_ESCR_SPEC>;
#[doc = "Extended Communication Control Register"]
pub mod lin_lin_escr;
#[doc = "LIN_LIN_RDR (r) register accessor: Received Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_rdr`]
module"]
pub type LIN_LIN_RDR = crate::Reg<lin_lin_rdr::LIN_LIN_RDR_SPEC>;
#[doc = "Received Data Register"]
pub mod lin_lin_rdr;
#[doc = "LIN_LIN_TDR (w) register accessor: Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_tdr`]
module"]
pub type LIN_LIN_TDR = crate::Reg<lin_lin_tdr::LIN_LIN_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod lin_lin_tdr;
#[doc = "LIN_LIN_BGR (rw) register accessor: Baud Rate Generator Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_bgr`]
module"]
pub type LIN_LIN_BGR = crate::Reg<lin_lin_bgr::LIN_LIN_BGR_SPEC>;
#[doc = "Baud Rate Generator Registers"]
pub mod lin_lin_bgr;
#[doc = "LIN_LIN_FCR1 (rw) register accessor: FIFO Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_fcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_fcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_fcr1`]
module"]
pub type LIN_LIN_FCR1 = crate::Reg<lin_lin_fcr1::LIN_LIN_FCR1_SPEC>;
#[doc = "FIFO Control Register 1"]
pub mod lin_lin_fcr1;
#[doc = "LIN_LIN_FCR0 (rw) register accessor: FIFO Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_fcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_fcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_fcr0`]
module"]
pub type LIN_LIN_FCR0 = crate::Reg<lin_lin_fcr0::LIN_LIN_FCR0_SPEC>;
#[doc = "FIFO Control Register 0"]
pub mod lin_lin_fcr0;
#[doc = "LIN_LIN_FBYTE1 (rw) register accessor: FIFO Byte Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_fbyte1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_fbyte1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_fbyte1`]
module"]
pub type LIN_LIN_FBYTE1 = crate::Reg<lin_lin_fbyte1::LIN_LIN_FBYTE1_SPEC>;
#[doc = "FIFO Byte Register 1"]
pub mod lin_lin_fbyte1;
#[doc = "LIN_LIN_FBYTE2 (rw) register accessor: FIFO Byte Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_fbyte2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_fbyte2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lin_lin_fbyte2`]
module"]
pub type LIN_LIN_FBYTE2 = crate::Reg<lin_lin_fbyte2::LIN_LIN_FBYTE2_SPEC>;
#[doc = "FIFO Byte Register 2"]
pub mod lin_lin_fbyte2;
#[doc = "I2C_I2C_IBCR (rw) register accessor: I2C Bus Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ibcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ibcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_ibcr`]
module"]
pub type I2C_I2C_IBCR = crate::Reg<i2c_i2c_ibcr::I2C_I2C_IBCR_SPEC>;
#[doc = "I2C Bus Control Register"]
pub mod i2c_i2c_ibcr;
#[doc = "I2C_I2C_SMR (rw) register accessor: Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_smr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_smr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_smr`]
module"]
pub type I2C_I2C_SMR = crate::Reg<i2c_i2c_smr::I2C_I2C_SMR_SPEC>;
#[doc = "Serial Mode Register"]
pub mod i2c_i2c_smr;
#[doc = "I2C_I2C_IBSR (rw) register accessor: I2C Bus Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ibsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ibsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_ibsr`]
module"]
pub type I2C_I2C_IBSR = crate::Reg<i2c_i2c_ibsr::I2C_I2C_IBSR_SPEC>;
#[doc = "I2C Bus Status Register"]
pub mod i2c_i2c_ibsr;
#[doc = "I2C_I2C_SSR (rw) register accessor: Serial Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_ssr`]
module"]
pub type I2C_I2C_SSR = crate::Reg<i2c_i2c_ssr::I2C_I2C_SSR_SPEC>;
#[doc = "Serial Status Register"]
pub mod i2c_i2c_ssr;
#[doc = "I2C_I2C_RDR (r) register accessor: Received Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_rdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_rdr`]
module"]
pub type I2C_I2C_RDR = crate::Reg<i2c_i2c_rdr::I2C_I2C_RDR_SPEC>;
#[doc = "Received Data Register"]
pub mod i2c_i2c_rdr;
#[doc = "I2C_I2C_TDR (w) register accessor: Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_tdr`]
module"]
pub type I2C_I2C_TDR = crate::Reg<i2c_i2c_tdr::I2C_I2C_TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod i2c_i2c_tdr;
#[doc = "I2C_I2C_ISMK (rw) register accessor: 7-bit Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ismk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ismk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_ismk`]
module"]
pub type I2C_I2C_ISMK = crate::Reg<i2c_i2c_ismk::I2C_I2C_ISMK_SPEC>;
#[doc = "7-bit Slave Address Mask Register"]
pub mod i2c_i2c_ismk;
#[doc = "I2C_I2C_ISBA (rw) register accessor: 7-bit Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_isba::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_isba::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_isba`]
module"]
pub type I2C_I2C_ISBA = crate::Reg<i2c_i2c_isba::I2C_I2C_ISBA_SPEC>;
#[doc = "7-bit Slave Address Register"]
pub mod i2c_i2c_isba;
#[doc = "I2C_I2C_BGR (rw) register accessor: Baud Rate Generator Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_bgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_bgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_bgr`]
module"]
pub type I2C_I2C_BGR = crate::Reg<i2c_i2c_bgr::I2C_I2C_BGR_SPEC>;
#[doc = "Baud Rate Generator Registers"]
pub mod i2c_i2c_bgr;
#[doc = "I2C_I2C_FCR1 (rw) register accessor: FIFO Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_fcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_fcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_fcr1`]
module"]
pub type I2C_I2C_FCR1 = crate::Reg<i2c_i2c_fcr1::I2C_I2C_FCR1_SPEC>;
#[doc = "FIFO Control Register 1"]
pub mod i2c_i2c_fcr1;
#[doc = "I2C_I2C_FCR0 (rw) register accessor: FIFO Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_fcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_fcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_fcr0`]
module"]
pub type I2C_I2C_FCR0 = crate::Reg<i2c_i2c_fcr0::I2C_I2C_FCR0_SPEC>;
#[doc = "FIFO Control Register 0"]
pub mod i2c_i2c_fcr0;
#[doc = "I2C_I2C_FBYTE1 (rw) register accessor: FIFO Byte Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_fbyte1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_fbyte1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_fbyte1`]
module"]
pub type I2C_I2C_FBYTE1 = crate::Reg<i2c_i2c_fbyte1::I2C_I2C_FBYTE1_SPEC>;
#[doc = "FIFO Byte Register 1"]
pub mod i2c_i2c_fbyte1;
#[doc = "I2C_I2C_FBYTE2 (rw) register accessor: FIFO Byte Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_fbyte2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_fbyte2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_i2c_fbyte2`]
module"]
pub type I2C_I2C_FBYTE2 = crate::Reg<i2c_i2c_fbyte2::I2C_I2C_FBYTE2_SPEC>;
#[doc = "FIFO Byte Register 2"]
pub mod i2c_i2c_fbyte2;
