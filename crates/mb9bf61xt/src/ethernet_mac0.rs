#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcr: MCR,
    mffr: MFFR,
    mhtrh: MHTRH,
    mhtrl: MHTRL,
    gar: GAR,
    gdr: GDR,
    fcr: FCR,
    vtr: VTR,
    _reserved8: [u8; 0x08],
    rwffr: RWFFR,
    pmtr: PMTR,
    lpicsr: LPICSR,
    lpitcr: LPITCR,
    isr: ISR,
    imr: IMR,
    mar0h: MAR0H,
    mar0l: MAR0L,
    mar1h: MAR1H,
    mar1l: MAR1L,
    mar2h: MAR2H,
    mar2l: MAR2L,
    mar3h: MAR3H,
    mar3l: MAR3L,
    mar4h: MAR4H,
    mar4l: MAR4L,
    mar5h: MAR5H,
    mar5l: MAR5L,
    mar6h: MAR6H,
    mar6l: MAR6L,
    mar7h: MAR7H,
    mar7l: MAR7L,
    mar8h: MAR8H,
    mar8l: MAR8L,
    mar9h: MAR9H,
    mar9l: MAR9L,
    mar10h: MAR10H,
    mar10l: MAR10L,
    mar11h: MAR11H,
    mar11l: MAR11L,
    mar12h: MAR12H,
    mar12l: MAR12L,
    mar13h: MAR13H,
    mar13l: MAR13L,
    mar14h: MAR14H,
    mar14l: MAR14L,
    mar15h: MAR15H,
    mar15l: MAR15L,
    _reserved46: [u8; 0x18],
    rgsr: RGSR,
    _reserved47: [u8; 0x24],
    mmc_cntl: MMC_CNTL,
    mmc_intr_rx: MMC_INTR_RX,
    mmc_intr_tx: MMC_INTR_TX,
    mmc_intr_mask_rx: MMC_INTR_MASK_RX,
    mmc_intr_mask_tx: MMC_INTR_MASK_TX,
    txoctetcount_gb: TXOCTETCOUNT_GB,
    txframecount_gb: TXFRAMECOUNT_GB,
    txbroadcastframes_g: TXBROADCASTFRAMES_G,
    txmulticastframes_g: TXMULTICASTFRAMES_G,
    tx64octets_gb: TX64OCTETS_GB,
    tx65to127octets_gb: TX65TO127OCTETS_GB,
    tx128to255octets_gb: TX128TO255OCTETS_GB,
    tx256to511octets_gb: TX256TO511OCTETS_GB,
    tx512to1023octets_gb: TX512TO1023OCTETS_GB,
    tx1024tomaxoctets_gb: TX1024TOMAXOCTETS_GB,
    txunicastframes_gb: TXUNICASTFRAMES_GB,
    txmulticastframes_gb: TXMULTICASTFRAMES_GB,
    txbroadcastframes_gb: TXBROADCASTFRAMES_GB,
    txunderflowerror: TXUNDERFLOWERROR,
    txsinglecol_g: TXSINGLECOL_G,
    txmulticol_g: TXMULTICOL_G,
    txdeferred: TXDEFERRED,
    txlatecol: TXLATECOL,
    txexesscol: TXEXESSCOL,
    txcarriererror: TXCARRIERERROR,
    txoctetcount_g: TXOCTETCOUNT_G,
    txframecount_g: TXFRAMECOUNT_G,
    txexecessdef_g: TXEXECESSDEF_G,
    txpauseframes: TXPAUSEFRAMES,
    txvlanframes_g: TXVLANFRAMES_G,
    _reserved77: [u8; 0x08],
    rxframecount_gb: RXFRAMECOUNT_GB,
    rxoctetcount_gb: RXOCTETCOUNT_GB,
    rxoctetcount_g: RXOCTETCOUNT_G,
    rxbroadcastframes_g: RXBROADCASTFRAMES_G,
    rxmulticastframes_g: RXMULTICASTFRAMES_G,
    rxcrcerror: RXCRCERROR,
    rxallignmenterror: RXALLIGNMENTERROR,
    rxrunterror: RXRUNTERROR,
    rxjabbererror: RXJABBERERROR,
    rxundersize_g: RXUNDERSIZE_G,
    rxoversize_g: RXOVERSIZE_G,
    rx64octets_gb: RX64OCTETS_GB,
    rx65to127octets_gb: RX65TO127OCTETS_GB,
    rx128to255octets_gb: RX128TO255OCTETS_GB,
    rx256to511octets_gb: RX256TO511OCTETS_GB,
    rx512to1023octets_gb: RX512TO1023OCTETS_GB,
    rx1024tomaxoctets_gb: RX1024TOMAXOCTETS_GB,
    rxunicastframes_g: RXUNICASTFRAMES_G,
    rxlengtherror: RXLENGTHERROR,
    rxoutofrangetype: RXOUTOFRANGETYPE,
    rxpauseframes: RXPAUSEFRAMES,
    rxfifooverflow: RXFIFOOVERFLOW,
    rxvlanframes_gb: RXVLANFRAMES_GB,
    rxwatchdogerror: RXWATCHDOGERROR,
    _reserved101: [u8; 0x20],
    mmc_ipc_intr_mask_rx: MMC_IPC_INTR_MASK_RX,
    _reserved102: [u8; 0x04],
    mmc_ipc_intr_rx: MMC_IPC_INTR_RX,
    _reserved103: [u8; 0x04],
    rxipv4_gd_frms: RXIPV4_GD_FRMS,
    rxipv4_hdrerr_frms: RXIPV4_HDRERR_FRMS,
    rxipv4_nopay_frms: RXIPV4_NOPAY_FRMS,
    rxipv4_frag_frms: RXIPV4_FRAG_FRMS,
    rxipv4_udsbl_frms: RXIPV4_UDSBL_FRMS,
    rxipv6_gd_frms: RXIPV6_GD_FRMS,
    rxipv6_hdrerr_frms: RXIPV6_HDRERR_FRMS,
    rxipv6_nopay_frms: RXIPV6_NOPAY_FRMS,
    rxudp_gd_frms: RXUDP_GD_FRMS,
    rxudp_err_frms: RXUDP_ERR_FRMS,
    rxtcp_gd_frms: RXTCP_GD_FRMS,
    rxtcp_err_frms: RXTCP_ERR_FRMS,
    rxicmp_gd_frms: RXICMP_GD_FRMS,
    rxicmp_err_frms: RXICMP_ERR_FRMS,
    _reserved117: [u8; 0x08],
    rxipv4_gd_octets: RXIPV4_GD_OCTETS,
    rxipv4_hdrerr_octets: RXIPV4_HDRERR_OCTETS,
    rxipv4_nopay_octets: RXIPV4_NOPAY_OCTETS,
    rxipv4_frag_octets: RXIPV4_FRAG_OCTETS,
    rxipv4_udsbl_octets: RXIPV4_UDSBL_OCTETS,
    rxipv6_gd_octets: RXIPV6_GD_OCTETS,
    rxipv6_hdrerr_octets: RXIPV6_HDRERR_OCTETS,
    rxipv6_nopay_octets: RXIPV6_NOPAY_OCTETS,
    rxudp_gd_octets: RXUDP_GD_OCTETS,
    rxudp_err_octets: RXUDP_ERR_OCTETS,
    rxtcp_gd_octets: RXTCP_GD_OCTETS,
    rxtcp_err_octets: RXTCP_ERR_OCTETS,
    rxicmp_gd_octets: RXICMP_GD_OCTETS,
    rxicmp_err_octets: RXICMP_ERR_OCTETS,
    _reserved131: [u8; 0x0478],
    tscr: TSCR,
    ssir: SSIR,
    stsr: STSR,
    stnr: STNR,
    stsur: STSUR,
    stnur: STNUR,
    tsar: TSAR,
    ttsr: TTSR,
    ttnr: TTNR,
    sthwsr: STHWSR,
    tsr: TSR,
    ppscr: PPSCR,
    atnr: ATNR,
    atsr: ATSR,
    _reserved145: [u8; 0xc8],
    mar16h: MAR16H,
    mar16l: MAR16L,
    mar17h: MAR17H,
    mar17l: MAR17L,
    mar18h: MAR18H,
    mar18l: MAR18L,
    mar19h: MAR19H,
    mar19l: MAR19L,
    mar20h: MAR20H,
    mar20l: MAR20L,
    mar21h: MAR21H,
    mar21l: MAR21L,
    mar22h: MAR22H,
    mar22l: MAR22L,
    mar23h: MAR23H,
    mar23l: MAR23L,
    mar24h: MAR24H,
    mar24l: MAR24L,
    mar25h: MAR25H,
    mar25l: MAR25L,
    mar26h: MAR26H,
    mar26l: MAR26L,
    mar27h: MAR27H,
    mar27l: MAR27L,
    mar28h: MAR28H,
    mar28l: MAR28L,
    mar29h: MAR29H,
    mar29l: MAR29L,
    mar30h: MAR30H,
    mar30l: MAR30L,
    mar31h: MAR31H,
    mar31l: MAR31L,
    _reserved177: [u8; 0x0780],
    bmr: BMR,
    tpdr: TPDR,
    rpdr: RPDR,
    rdlar: RDLAR,
    tdlar: TDLAR,
    sr: SR,
    omr: OMR,
    ier: IER,
    mfbocr: MFBOCR,
    riwtr: RIWTR,
    _reserved187: [u8; 0x04],
    ahbsr: AHBSR,
    _reserved188: [u8; 0x18],
    chtdr: CHTDR,
    chrdr: CHRDR,
    chtbar: CHTBAR,
    chrbar: CHRBAR,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x04 - MAC Frame Filter Register"]
    #[inline(always)]
    pub const fn mffr(&self) -> &MFFR {
        &self.mffr
    }
    #[doc = "0x08 - MAC Hash Table Register (High)"]
    #[inline(always)]
    pub const fn mhtrh(&self) -> &MHTRH {
        &self.mhtrh
    }
    #[doc = "0x0c - MAC Hash Table Register (Low)"]
    #[inline(always)]
    pub const fn mhtrl(&self) -> &MHTRL {
        &self.mhtrl
    }
    #[doc = "0x10 - GMII/MII Address Register"]
    #[inline(always)]
    pub const fn gar(&self) -> &GAR {
        &self.gar
    }
    #[doc = "0x14 - GMII/MII Data Register"]
    #[inline(always)]
    pub const fn gdr(&self) -> &GDR {
        &self.gdr
    }
    #[doc = "0x18 - Flow Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x1c - VLAN TAG Register"]
    #[inline(always)]
    pub const fn vtr(&self) -> &VTR {
        &self.vtr
    }
    #[doc = "0x28 - Remote Wake-up Frame Filter Register"]
    #[inline(always)]
    pub const fn rwffr(&self) -> &RWFFR {
        &self.rwffr
    }
    #[doc = "0x2c - PMT Register"]
    #[inline(always)]
    pub const fn pmtr(&self) -> &PMTR {
        &self.pmtr
    }
    #[doc = "0x30 - LPI Control and Status Register"]
    #[inline(always)]
    pub const fn lpicsr(&self) -> &LPICSR {
        &self.lpicsr
    }
    #[doc = "0x34 - LPI Timers Control Register"]
    #[inline(always)]
    pub const fn lpitcr(&self) -> &LPITCR {
        &self.lpitcr
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x3c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x40 - MAC Address0 Register (High)"]
    #[inline(always)]
    pub const fn mar0h(&self) -> &MAR0H {
        &self.mar0h
    }
    #[doc = "0x44 - MAC Address0 Register (Low)"]
    #[inline(always)]
    pub const fn mar0l(&self) -> &MAR0L {
        &self.mar0l
    }
    #[doc = "0x48 - MAC Address1 Register -High"]
    #[inline(always)]
    pub const fn mar1h(&self) -> &MAR1H {
        &self.mar1h
    }
    #[doc = "0x4c - MAC Address1 Register -Low"]
    #[inline(always)]
    pub const fn mar1l(&self) -> &MAR1L {
        &self.mar1l
    }
    #[doc = "0x50 - MAC Address2 Register -High"]
    #[inline(always)]
    pub const fn mar2h(&self) -> &MAR2H {
        &self.mar2h
    }
    #[doc = "0x54 - MAC Address2 Register -Low"]
    #[inline(always)]
    pub const fn mar2l(&self) -> &MAR2L {
        &self.mar2l
    }
    #[doc = "0x58 - MAC Address3 Register -High"]
    #[inline(always)]
    pub const fn mar3h(&self) -> &MAR3H {
        &self.mar3h
    }
    #[doc = "0x5c - MAC Address3 Register -Low"]
    #[inline(always)]
    pub const fn mar3l(&self) -> &MAR3L {
        &self.mar3l
    }
    #[doc = "0x60 - MAC Address4 Register -High"]
    #[inline(always)]
    pub const fn mar4h(&self) -> &MAR4H {
        &self.mar4h
    }
    #[doc = "0x64 - MAC Address4 Register -Low"]
    #[inline(always)]
    pub const fn mar4l(&self) -> &MAR4L {
        &self.mar4l
    }
    #[doc = "0x68 - MAC Address5 Register -High"]
    #[inline(always)]
    pub const fn mar5h(&self) -> &MAR5H {
        &self.mar5h
    }
    #[doc = "0x6c - MAC Address5 Register -Low"]
    #[inline(always)]
    pub const fn mar5l(&self) -> &MAR5L {
        &self.mar5l
    }
    #[doc = "0x70 - MAC Address6 Register -High"]
    #[inline(always)]
    pub const fn mar6h(&self) -> &MAR6H {
        &self.mar6h
    }
    #[doc = "0x74 - MAC Address6 Register -Low"]
    #[inline(always)]
    pub const fn mar6l(&self) -> &MAR6L {
        &self.mar6l
    }
    #[doc = "0x78 - MAC Address7 Register -High"]
    #[inline(always)]
    pub const fn mar7h(&self) -> &MAR7H {
        &self.mar7h
    }
    #[doc = "0x7c - MAC Address7 Register -Low"]
    #[inline(always)]
    pub const fn mar7l(&self) -> &MAR7L {
        &self.mar7l
    }
    #[doc = "0x80 - MAC Address8 Register -High"]
    #[inline(always)]
    pub const fn mar8h(&self) -> &MAR8H {
        &self.mar8h
    }
    #[doc = "0x84 - MAC Address8 Register -Low"]
    #[inline(always)]
    pub const fn mar8l(&self) -> &MAR8L {
        &self.mar8l
    }
    #[doc = "0x88 - MAC Address9 Register -High"]
    #[inline(always)]
    pub const fn mar9h(&self) -> &MAR9H {
        &self.mar9h
    }
    #[doc = "0x8c - MAC Address9 Register -Low"]
    #[inline(always)]
    pub const fn mar9l(&self) -> &MAR9L {
        &self.mar9l
    }
    #[doc = "0x90 - MAC Address10 Register -High"]
    #[inline(always)]
    pub const fn mar10h(&self) -> &MAR10H {
        &self.mar10h
    }
    #[doc = "0x94 - MAC Address10 Register -Low"]
    #[inline(always)]
    pub const fn mar10l(&self) -> &MAR10L {
        &self.mar10l
    }
    #[doc = "0x98 - MAC Address11 Register -High"]
    #[inline(always)]
    pub const fn mar11h(&self) -> &MAR11H {
        &self.mar11h
    }
    #[doc = "0x9c - MAC Address11 Register -Low"]
    #[inline(always)]
    pub const fn mar11l(&self) -> &MAR11L {
        &self.mar11l
    }
    #[doc = "0xa0 - MAC Address12 Register -High"]
    #[inline(always)]
    pub const fn mar12h(&self) -> &MAR12H {
        &self.mar12h
    }
    #[doc = "0xa4 - MAC Address12 Register -Low"]
    #[inline(always)]
    pub const fn mar12l(&self) -> &MAR12L {
        &self.mar12l
    }
    #[doc = "0xa8 - MAC Address13 Register -High"]
    #[inline(always)]
    pub const fn mar13h(&self) -> &MAR13H {
        &self.mar13h
    }
    #[doc = "0xac - MAC Address13 Register -Low"]
    #[inline(always)]
    pub const fn mar13l(&self) -> &MAR13L {
        &self.mar13l
    }
    #[doc = "0xb0 - MAC Address14 Register -High"]
    #[inline(always)]
    pub const fn mar14h(&self) -> &MAR14H {
        &self.mar14h
    }
    #[doc = "0xb4 - MAC Address14 Register -Low"]
    #[inline(always)]
    pub const fn mar14l(&self) -> &MAR14L {
        &self.mar14l
    }
    #[doc = "0xb8 - MAC Address15 Register -High"]
    #[inline(always)]
    pub const fn mar15h(&self) -> &MAR15H {
        &self.mar15h
    }
    #[doc = "0xbc - MAC Address15 Register -Low"]
    #[inline(always)]
    pub const fn mar15l(&self) -> &MAR15L {
        &self.mar15l
    }
    #[doc = "0xd8 - RGMII Status Register)"]
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    #[doc = "0x100 - MMC Control Register"]
    #[inline(always)]
    pub const fn mmc_cntl(&self) -> &MMC_CNTL {
        &self.mmc_cntl
    }
    #[doc = "0x104 - Receive Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_intr_rx(&self) -> &MMC_INTR_RX {
        &self.mmc_intr_rx
    }
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_intr_tx(&self) -> &MMC_INTR_TX {
        &self.mmc_intr_tx
    }
    #[doc = "0x10c - MMC Receive Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_intr_mask_rx(&self) -> &MMC_INTR_MASK_RX {
        &self.mmc_intr_mask_rx
    }
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_intr_mask_tx(&self) -> &MMC_INTR_MASK_TX {
        &self.mmc_intr_mask_tx
    }
    #[doc = "0x114 - \"Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames\""]
    #[inline(always)]
    pub const fn txoctetcount_gb(&self) -> &TXOCTETCOUNT_GB {
        &self.txoctetcount_gb
    }
    #[doc = "0x118 - \"Number of good and bad frames transmitted, exclusive of retried frames\""]
    #[inline(always)]
    pub const fn txframecount_gb(&self) -> &TXFRAMECOUNT_GB {
        &self.txframecount_gb
    }
    #[doc = "0x11c - Number of good broadcast frames transmitted"]
    #[inline(always)]
    pub const fn txbroadcastframes_g(&self) -> &TXBROADCASTFRAMES_G {
        &self.txbroadcastframes_g
    }
    #[doc = "0x120 - Number of good multicast frames transmitted"]
    #[inline(always)]
    pub const fn txmulticastframes_g(&self) -> &TXMULTICASTFRAMES_G {
        &self.txmulticastframes_g
    }
    #[doc = "0x124 - \"Number of good and bad frames transmitted with length of 64 bytes, exclusive of preamble and retried frames\""]
    #[inline(always)]
    pub const fn tx64octets_gb(&self) -> &TX64OCTETS_GB {
        &self.tx64octets_gb
    }
    #[doc = "0x128 - \"Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames\""]
    #[inline(always)]
    pub const fn tx65to127octets_gb(&self) -> &TX65TO127OCTETS_GB {
        &self.tx65to127octets_gb
    }
    #[doc = "0x12c - \"Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames\""]
    #[inline(always)]
    pub const fn tx128to255octets_gb(&self) -> &TX128TO255OCTETS_GB {
        &self.tx128to255octets_gb
    }
    #[doc = "0x130 - \"Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames\""]
    #[inline(always)]
    pub const fn tx256to511octets_gb(&self) -> &TX256TO511OCTETS_GB {
        &self.tx256to511octets_gb
    }
    #[doc = "0x134 - \"Number of good and bad frames transmitted with length between 512 and 1023 (inclusive) bytes, exclusive of preamble and retried frames\""]
    #[inline(always)]
    pub const fn tx512to1023octets_gb(&self) -> &TX512TO1023OCTETS_GB {
        &self.tx512to1023octets_gb
    }
    #[doc = "0x138 - \"Number of good and bad frames transmitted with length between 1024 and Maxsize (inclusive) bytes, exclusive of preamble and retried frames\""]
    #[inline(always)]
    pub const fn tx1024tomaxoctets_gb(&self) -> &TX1024TOMAXOCTETS_GB {
        &self.tx1024tomaxoctets_gb
    }
    #[doc = "0x13c - Number of good and bad unicast frames transmitted"]
    #[inline(always)]
    pub const fn txunicastframes_gb(&self) -> &TXUNICASTFRAMES_GB {
        &self.txunicastframes_gb
    }
    #[doc = "0x140 - Number of good and bad multicast frames transmitted"]
    #[inline(always)]
    pub const fn txmulticastframes_gb(&self) -> &TXMULTICASTFRAMES_GB {
        &self.txmulticastframes_gb
    }
    #[doc = "0x144 - Number of good and bad broadcast frames transmitted"]
    #[inline(always)]
    pub const fn txbroadcastframes_gb(&self) -> &TXBROADCASTFRAMES_GB {
        &self.txbroadcastframes_gb
    }
    #[doc = "0x148 - Number of frames aborted due to frame underflow error"]
    #[inline(always)]
    pub const fn txunderflowerror(&self) -> &TXUNDERFLOWERROR {
        &self.txunderflowerror
    }
    #[doc = "0x14c - Number of successfully transmitted frames after a single collision in Half-duplex mode"]
    #[inline(always)]
    pub const fn txsinglecol_g(&self) -> &TXSINGLECOL_G {
        &self.txsinglecol_g
    }
    #[doc = "0x150 - Number of successfully transmitted frames after more than a single collision in Half-duplex mode"]
    #[inline(always)]
    pub const fn txmulticol_g(&self) -> &TXMULTICOL_G {
        &self.txmulticol_g
    }
    #[doc = "0x154 - Number of successfully transmitted frames after a deferral in Half-duplex mode."]
    #[inline(always)]
    pub const fn txdeferred(&self) -> &TXDEFERRED {
        &self.txdeferred
    }
    #[doc = "0x158 - Number of frames aborted due to late collision error."]
    #[inline(always)]
    pub const fn txlatecol(&self) -> &TXLATECOL {
        &self.txlatecol
    }
    #[doc = "0x15c - Number of frames aborted due to excessive (16) collision errors."]
    #[inline(always)]
    pub const fn txexesscol(&self) -> &TXEXESSCOL {
        &self.txexesscol
    }
    #[doc = "0x160 - Number of frames aborted due to carrier sense error (no carrier or loss of carrier)."]
    #[inline(always)]
    pub const fn txcarriererror(&self) -> &TXCARRIERERROR {
        &self.txcarriererror
    }
    #[doc = "0x164 - \"Number of bytes transmitted, exclusive of preamble, in good frames only. \""]
    #[inline(always)]
    pub const fn txoctetcount_g(&self) -> &TXOCTETCOUNT_G {
        &self.txoctetcount_g
    }
    #[doc = "0x168 - Number of good frames transmitted."]
    #[inline(always)]
    pub const fn txframecount_g(&self) -> &TXFRAMECOUNT_G {
        &self.txframecount_g
    }
    #[doc = "0x16c - Number of frames aborted due to excessive deferral error (deferred for more than two max-sized frame times)."]
    #[inline(always)]
    pub const fn txexecessdef_g(&self) -> &TXEXECESSDEF_G {
        &self.txexecessdef_g
    }
    #[doc = "0x170 - Number of good PAUSE frames transmitted."]
    #[inline(always)]
    pub const fn txpauseframes(&self) -> &TXPAUSEFRAMES {
        &self.txpauseframes
    }
    #[doc = "0x174 - \"Number of good VLAN frames transmitted, exclusive of retried frames. \""]
    #[inline(always)]
    pub const fn txvlanframes_g(&self) -> &TXVLANFRAMES_G {
        &self.txvlanframes_g
    }
    #[doc = "0x180 - Number of good and bad frames received."]
    #[inline(always)]
    pub const fn rxframecount_gb(&self) -> &RXFRAMECOUNT_GB {
        &self.rxframecount_gb
    }
    #[doc = "0x184 - \"Number of bytes received, exclusive of preamble, in good and bad frames. \""]
    #[inline(always)]
    pub const fn rxoctetcount_gb(&self) -> &RXOCTETCOUNT_GB {
        &self.rxoctetcount_gb
    }
    #[doc = "0x188 - \"Number of bytes received, exclusive of preamble, only in good frames. \""]
    #[inline(always)]
    pub const fn rxoctetcount_g(&self) -> &RXOCTETCOUNT_G {
        &self.rxoctetcount_g
    }
    #[doc = "0x18c - Number of good broadcast frames received."]
    #[inline(always)]
    pub const fn rxbroadcastframes_g(&self) -> &RXBROADCASTFRAMES_G {
        &self.rxbroadcastframes_g
    }
    #[doc = "0x190 - Number of good multicast frames received."]
    #[inline(always)]
    pub const fn rxmulticastframes_g(&self) -> &RXMULTICASTFRAMES_G {
        &self.rxmulticastframes_g
    }
    #[doc = "0x194 - Number of frames received with CRC error."]
    #[inline(always)]
    pub const fn rxcrcerror(&self) -> &RXCRCERROR {
        &self.rxcrcerror
    }
    #[doc = "0x198 - Number of frames received with alignment (dribble) error. Valid only in 10/100 mode."]
    #[inline(always)]
    pub const fn rxallignmenterror(&self) -> &RXALLIGNMENTERROR {
        &self.rxallignmenterror
    }
    #[doc = "0x19c - Number of frames received with runt (64 bytes and CRC error) error."]
    #[inline(always)]
    pub const fn rxrunterror(&self) -> &RXRUNTERROR {
        &self.rxrunterror
    }
    #[doc = "0x1a0 - Number of frames received with length greater than 1518 bytes with CRC error."]
    #[inline(always)]
    pub const fn rxjabbererror(&self) -> &RXJABBERERROR {
        &self.rxjabbererror
    }
    #[doc = "0x1a4 - \"Number of frames received with length less than 64 bytes, without any errors. \""]
    #[inline(always)]
    pub const fn rxundersize_g(&self) -> &RXUNDERSIZE_G {
        &self.rxundersize_g
    }
    #[doc = "0x1a8 - Number of frames received with length greater than the maxsize without error."]
    #[inline(always)]
    pub const fn rxoversize_g(&self) -> &RXOVERSIZE_G {
        &self.rxoversize_g
    }
    #[doc = "0x1ac - \"Number of good and bad frames received with length 64 bytes, exclusive of preamble. \""]
    #[inline(always)]
    pub const fn rx64octets_gb(&self) -> &RX64OCTETS_GB {
        &self.rx64octets_gb
    }
    #[doc = "0x1b0 - \"Number of good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble. \""]
    #[inline(always)]
    pub const fn rx65to127octets_gb(&self) -> &RX65TO127OCTETS_GB {
        &self.rx65to127octets_gb
    }
    #[doc = "0x1b4 - \"Number of good and bad frames received with length between 128 and 255 (inclusive) bytes, exclusive of preamble. \""]
    #[inline(always)]
    pub const fn rx128to255octets_gb(&self) -> &RX128TO255OCTETS_GB {
        &self.rx128to255octets_gb
    }
    #[doc = "0x1b8 - \"Number of good and bad frames received with length between 256 and 511 (inclusive) bytes, exclusive of preamble. \""]
    #[inline(always)]
    pub const fn rx256to511octets_gb(&self) -> &RX256TO511OCTETS_GB {
        &self.rx256to511octets_gb
    }
    #[doc = "0x1bc - \"Number of good and bad frames received with length between 512 and 1023 (inclusive) bytes, exclusive of preamble. \""]
    #[inline(always)]
    pub const fn rx512to1023octets_gb(&self) -> &RX512TO1023OCTETS_GB {
        &self.rx512to1023octets_gb
    }
    #[doc = "0x1c0 - \"Number of good and bad frames received with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble. \""]
    #[inline(always)]
    pub const fn rx1024tomaxoctets_gb(&self) -> &RX1024TOMAXOCTETS_GB {
        &self.rx1024tomaxoctets_gb
    }
    #[doc = "0x1c4 - Number of good unicast frames received."]
    #[inline(always)]
    pub const fn rxunicastframes_g(&self) -> &RXUNICASTFRAMES_G {
        &self.rxunicastframes_g
    }
    #[doc = "0x1c8 - \"Number of frames received with length error (Length type field is not the frame size), for all frames with valid length field. \""]
    #[inline(always)]
    pub const fn rxlengtherror(&self) -> &RXLENGTHERROR {
        &self.rxlengtherror
    }
    #[doc = "0x1cc - Number of frames received with length/type field not equal to the valid frame size (>1500)"]
    #[inline(always)]
    pub const fn rxoutofrangetype(&self) -> &RXOUTOFRANGETYPE {
        &self.rxoutofrangetype
    }
    #[doc = "0x1d0 - Number of good and valid PAUSE frames received."]
    #[inline(always)]
    pub const fn rxpauseframes(&self) -> &RXPAUSEFRAMES {
        &self.rxpauseframes
    }
    #[doc = "0x1d4 - Number of missed received frames due to FIFO overflow."]
    #[inline(always)]
    pub const fn rxfifooverflow(&self) -> &RXFIFOOVERFLOW {
        &self.rxfifooverflow
    }
    #[doc = "0x1d8 - Number of good and bad VLAN frames received."]
    #[inline(always)]
    pub const fn rxvlanframes_gb(&self) -> &RXVLANFRAMES_GB {
        &self.rxvlanframes_gb
    }
    #[doc = "0x1dc - Number of frames received with error due to watchdog timeout error (frames with a data load larger than 2048 bytes)."]
    #[inline(always)]
    pub const fn rxwatchdogerror(&self) -> &RXWATCHDOGERROR {
        &self.rxwatchdogerror
    }
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_ipc_intr_mask_rx(&self) -> &MMC_IPC_INTR_MASK_RX {
        &self.mmc_ipc_intr_mask_rx
    }
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_ipc_intr_rx(&self) -> &MMC_IPC_INTR_RX {
        &self.mmc_ipc_intr_rx
    }
    #[doc = "0x210 - \"Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload \""]
    #[inline(always)]
    pub const fn rxipv4_gd_frms(&self) -> &RXIPV4_GD_FRMS {
        &self.rxipv4_gd_frms
    }
    #[doc = "0x214 - \"Number of IPv4 datagrams received with header errors (checksum, length, or version mismatch) \""]
    #[inline(always)]
    pub const fn rxipv4_hdrerr_frms(&self) -> &RXIPV4_HDRERR_FRMS {
        &self.rxipv4_hdrerr_frms
    }
    #[doc = "0x218 - \"Number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine \""]
    #[inline(always)]
    pub const fn rxipv4_nopay_frms(&self) -> &RXIPV4_NOPAY_FRMS {
        &self.rxipv4_nopay_frms
    }
    #[doc = "0x21c - Number of good IPv4 datagrams with fragmentation"]
    #[inline(always)]
    pub const fn rxipv4_frag_frms(&self) -> &RXIPV4_FRAG_FRMS {
        &self.rxipv4_frag_frms
    }
    #[doc = "0x220 - Number of good IPv4 datagrams received that had a UDP payload with checksum disabled"]
    #[inline(always)]
    pub const fn rxipv4_udsbl_frms(&self) -> &RXIPV4_UDSBL_FRMS {
        &self.rxipv4_udsbl_frms
    }
    #[doc = "0x224 - \"Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads \""]
    #[inline(always)]
    pub const fn rxipv6_gd_frms(&self) -> &RXIPV6_GD_FRMS {
        &self.rxipv6_gd_frms
    }
    #[doc = "0x228 - Number of IPv6 datagrams received with header errors (length or version mismatch)"]
    #[inline(always)]
    pub const fn rxipv6_hdrerr_frms(&self) -> &RXIPV6_HDRERR_FRMS {
        &self.rxipv6_hdrerr_frms
    }
    #[doc = "0x22c - \"Number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers \""]
    #[inline(always)]
    pub const fn rxipv6_nopay_frms(&self) -> &RXIPV6_NOPAY_FRMS {
        &self.rxipv6_nopay_frms
    }
    #[doc = "0x230 - Number of good IP datagrams with a good UDP payload. This counter is not updated when the rxipv4_udsbl_frms counter is incremented."]
    #[inline(always)]
    pub const fn rxudp_gd_frms(&self) -> &RXUDP_GD_FRMS {
        &self.rxudp_gd_frms
    }
    #[doc = "0x234 - Number of good IP datagrams whose UDP payload has a checksum error"]
    #[inline(always)]
    pub const fn rxudp_err_frms(&self) -> &RXUDP_ERR_FRMS {
        &self.rxudp_err_frms
    }
    #[doc = "0x238 - Number of good IP datagrams with a good TCP payload"]
    #[inline(always)]
    pub const fn rxtcp_gd_frms(&self) -> &RXTCP_GD_FRMS {
        &self.rxtcp_gd_frms
    }
    #[doc = "0x23c - Number of good IP datagrams whose TCP payload has a checksum error"]
    #[inline(always)]
    pub const fn rxtcp_err_frms(&self) -> &RXTCP_ERR_FRMS {
        &self.rxtcp_err_frms
    }
    #[doc = "0x240 - Number of good IP datagrams with a good ICMP payload"]
    #[inline(always)]
    pub const fn rxicmp_gd_frms(&self) -> &RXICMP_GD_FRMS {
        &self.rxicmp_gd_frms
    }
    #[doc = "0x244 - Number of good IP datagrams whose ICMP payload has a checksum error"]
    #[inline(always)]
    pub const fn rxicmp_err_frms(&self) -> &RXICMP_ERR_FRMS {
        &self.rxicmp_err_frms
    }
    #[doc = "0x250 - \"Number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. (Ethernet header, FCS, pad, or IP pad bytes are not included in this counter or in the octet counters listed below). \""]
    #[inline(always)]
    pub const fn rxipv4_gd_octets(&self) -> &RXIPV4_GD_OCTETS {
        &self.rxipv4_gd_octets
    }
    #[doc = "0x254 - \"Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter. \""]
    #[inline(always)]
    pub const fn rxipv4_hdrerr_octets(&self) -> &RXIPV4_HDRERR_OCTETS {
        &self.rxipv4_hdrerr_octets
    }
    #[doc = "0x258 - \"Number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 header's Length field is used to update this counter. \""]
    #[inline(always)]
    pub const fn rxipv4_nopay_octets(&self) -> &RXIPV4_NOPAY_OCTETS {
        &self.rxipv4_nopay_octets
    }
    #[doc = "0x25c - Number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 header's Length field is used to update this counter."]
    #[inline(always)]
    pub const fn rxipv4_frag_octets(&self) -> &RXIPV4_FRAG_OCTETS {
        &self.rxipv4_frag_octets
    }
    #[doc = "0x260 - Number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count IP Header bytes."]
    #[inline(always)]
    pub const fn rxipv4_udsbl_octets(&self) -> &RXIPV4_UDSBL_OCTETS {
        &self.rxipv4_udsbl_octets
    }
    #[doc = "0x264 - \"Number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data\""]
    #[inline(always)]
    pub const fn rxipv6_gd_octets(&self) -> &RXIPV6_GD_OCTETS {
        &self.rxipv6_gd_octets
    }
    #[doc = "0x268 - \"Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter. \""]
    #[inline(always)]
    pub const fn rxipv6_hdrerr_octets(&self) -> &RXIPV6_HDRERR_OCTETS {
        &self.rxipv6_hdrerr_octets
    }
    #[doc = "0x26c - \"Number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 header's Length field is used to update this counter. \""]
    #[inline(always)]
    pub const fn rxipv6_nopay_octets(&self) -> &RXIPV6_NOPAY_OCTETS {
        &self.rxipv6_nopay_octets
    }
    #[doc = "0x270 - Number of bytes received in a good UDP segment. This counter (and the counters below) does not count IP header bytes."]
    #[inline(always)]
    pub const fn rxudp_gd_octets(&self) -> &RXUDP_GD_OCTETS {
        &self.rxudp_gd_octets
    }
    #[doc = "0x274 - Number of bytes received in a UDP segment that had checksum errors"]
    #[inline(always)]
    pub const fn rxudp_err_octets(&self) -> &RXUDP_ERR_OCTETS {
        &self.rxudp_err_octets
    }
    #[doc = "0x278 - Number of bytes received in a good TCP segment"]
    #[inline(always)]
    pub const fn rxtcp_gd_octets(&self) -> &RXTCP_GD_OCTETS {
        &self.rxtcp_gd_octets
    }
    #[doc = "0x27c - Number of bytes received in a TCP segment with checksum errors"]
    #[inline(always)]
    pub const fn rxtcp_err_octets(&self) -> &RXTCP_ERR_OCTETS {
        &self.rxtcp_err_octets
    }
    #[doc = "0x280 - Number of bytes received in a good ICMP segment"]
    #[inline(always)]
    pub const fn rxicmp_gd_octets(&self) -> &RXICMP_GD_OCTETS {
        &self.rxicmp_gd_octets
    }
    #[doc = "0x284 - Number of bytes received in an ICMP segment with checksum errors"]
    #[inline(always)]
    pub const fn rxicmp_err_octets(&self) -> &RXICMP_ERR_OCTETS {
        &self.rxicmp_err_octets
    }
    #[doc = "0x700 - Time Stamp Control Register"]
    #[inline(always)]
    pub const fn tscr(&self) -> &TSCR {
        &self.tscr
    }
    #[doc = "0x704 - Sub-Second Increment Register"]
    #[inline(always)]
    pub const fn ssir(&self) -> &SSIR {
        &self.ssir
    }
    #[doc = "0x708 - System Time - Seconds Register"]
    #[inline(always)]
    pub const fn stsr(&self) -> &STSR {
        &self.stsr
    }
    #[doc = "0x70c - System Time - Nanoseconds Register"]
    #[inline(always)]
    pub const fn stnr(&self) -> &STNR {
        &self.stnr
    }
    #[doc = "0x710 - System Time - Seconds Update Register"]
    #[inline(always)]
    pub const fn stsur(&self) -> &STSUR {
        &self.stsur
    }
    #[doc = "0x714 - System Time - Nanoseconds Update Register"]
    #[inline(always)]
    pub const fn stnur(&self) -> &STNUR {
        &self.stnur
    }
    #[doc = "0x718 - Time Stamp Addend Register"]
    #[inline(always)]
    pub const fn tsar(&self) -> &TSAR {
        &self.tsar
    }
    #[doc = "0x71c - Target Time Seconds Register"]
    #[inline(always)]
    pub const fn ttsr(&self) -> &TTSR {
        &self.ttsr
    }
    #[doc = "0x720 - Target Time Nanoseconds Register"]
    #[inline(always)]
    pub const fn ttnr(&self) -> &TTNR {
        &self.ttnr
    }
    #[doc = "0x724 - System Time - Higher Word Seconds Register"]
    #[inline(always)]
    pub const fn sthwsr(&self) -> &STHWSR {
        &self.sthwsr
    }
    #[doc = "0x728 - Time Stamp Status Register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &TSR {
        &self.tsr
    }
    #[doc = "0x72c - PPS Control Register"]
    #[inline(always)]
    pub const fn ppscr(&self) -> &PPSCR {
        &self.ppscr
    }
    #[doc = "0x730 - Auxiliary Time Stamp - Nanoseconds Register"]
    #[inline(always)]
    pub const fn atnr(&self) -> &ATNR {
        &self.atnr
    }
    #[doc = "0x734 - Auxiliary Time Stamp - Seconds Register"]
    #[inline(always)]
    pub const fn atsr(&self) -> &ATSR {
        &self.atsr
    }
    #[doc = "0x800 - MAC Address16 Register -High"]
    #[inline(always)]
    pub const fn mar16h(&self) -> &MAR16H {
        &self.mar16h
    }
    #[doc = "0x804 - MAC Address16 Register -Low"]
    #[inline(always)]
    pub const fn mar16l(&self) -> &MAR16L {
        &self.mar16l
    }
    #[doc = "0x808 - MAC Address17 Register -High"]
    #[inline(always)]
    pub const fn mar17h(&self) -> &MAR17H {
        &self.mar17h
    }
    #[doc = "0x80c - MAC Address17 Register -Low"]
    #[inline(always)]
    pub const fn mar17l(&self) -> &MAR17L {
        &self.mar17l
    }
    #[doc = "0x810 - MAC Address18 Register -High"]
    #[inline(always)]
    pub const fn mar18h(&self) -> &MAR18H {
        &self.mar18h
    }
    #[doc = "0x814 - MAC Address18 Register -Low"]
    #[inline(always)]
    pub const fn mar18l(&self) -> &MAR18L {
        &self.mar18l
    }
    #[doc = "0x818 - MAC Address19 Register -High"]
    #[inline(always)]
    pub const fn mar19h(&self) -> &MAR19H {
        &self.mar19h
    }
    #[doc = "0x81c - MAC Address19 Register -Low"]
    #[inline(always)]
    pub const fn mar19l(&self) -> &MAR19L {
        &self.mar19l
    }
    #[doc = "0x820 - MAC Address20 Register -High"]
    #[inline(always)]
    pub const fn mar20h(&self) -> &MAR20H {
        &self.mar20h
    }
    #[doc = "0x824 - MAC Address20 Register -Low"]
    #[inline(always)]
    pub const fn mar20l(&self) -> &MAR20L {
        &self.mar20l
    }
    #[doc = "0x828 - MAC Address21 Register -High"]
    #[inline(always)]
    pub const fn mar21h(&self) -> &MAR21H {
        &self.mar21h
    }
    #[doc = "0x82c - MAC Address21 Register -Low"]
    #[inline(always)]
    pub const fn mar21l(&self) -> &MAR21L {
        &self.mar21l
    }
    #[doc = "0x830 - MAC Address22 Register -High"]
    #[inline(always)]
    pub const fn mar22h(&self) -> &MAR22H {
        &self.mar22h
    }
    #[doc = "0x834 - MAC Address22 Register -Low"]
    #[inline(always)]
    pub const fn mar22l(&self) -> &MAR22L {
        &self.mar22l
    }
    #[doc = "0x838 - MAC Address23 Register -High"]
    #[inline(always)]
    pub const fn mar23h(&self) -> &MAR23H {
        &self.mar23h
    }
    #[doc = "0x83c - MAC Address23 Register -Low"]
    #[inline(always)]
    pub const fn mar23l(&self) -> &MAR23L {
        &self.mar23l
    }
    #[doc = "0x840 - MAC Address24 Register -High"]
    #[inline(always)]
    pub const fn mar24h(&self) -> &MAR24H {
        &self.mar24h
    }
    #[doc = "0x844 - MAC Address24 Register -Low"]
    #[inline(always)]
    pub const fn mar24l(&self) -> &MAR24L {
        &self.mar24l
    }
    #[doc = "0x848 - MAC Address25 Register -High"]
    #[inline(always)]
    pub const fn mar25h(&self) -> &MAR25H {
        &self.mar25h
    }
    #[doc = "0x84c - MAC Address25 Register -Low"]
    #[inline(always)]
    pub const fn mar25l(&self) -> &MAR25L {
        &self.mar25l
    }
    #[doc = "0x850 - MAC Address26 Register -High"]
    #[inline(always)]
    pub const fn mar26h(&self) -> &MAR26H {
        &self.mar26h
    }
    #[doc = "0x854 - MAC Address26 Register -Low"]
    #[inline(always)]
    pub const fn mar26l(&self) -> &MAR26L {
        &self.mar26l
    }
    #[doc = "0x858 - MAC Address27 Register -High"]
    #[inline(always)]
    pub const fn mar27h(&self) -> &MAR27H {
        &self.mar27h
    }
    #[doc = "0x85c - MAC Address27 Register -Low"]
    #[inline(always)]
    pub const fn mar27l(&self) -> &MAR27L {
        &self.mar27l
    }
    #[doc = "0x860 - MAC Address28 Register -High"]
    #[inline(always)]
    pub const fn mar28h(&self) -> &MAR28H {
        &self.mar28h
    }
    #[doc = "0x864 - MAC Address28 Register -Low"]
    #[inline(always)]
    pub const fn mar28l(&self) -> &MAR28L {
        &self.mar28l
    }
    #[doc = "0x868 - MAC Address29 Register -High"]
    #[inline(always)]
    pub const fn mar29h(&self) -> &MAR29H {
        &self.mar29h
    }
    #[doc = "0x86c - MAC Address29 Register -Low"]
    #[inline(always)]
    pub const fn mar29l(&self) -> &MAR29L {
        &self.mar29l
    }
    #[doc = "0x870 - MAC Address30 Register -High"]
    #[inline(always)]
    pub const fn mar30h(&self) -> &MAR30H {
        &self.mar30h
    }
    #[doc = "0x874 - MAC Address30 Register -Low"]
    #[inline(always)]
    pub const fn mar30l(&self) -> &MAR30L {
        &self.mar30l
    }
    #[doc = "0x878 - MAC Address31 Register -High"]
    #[inline(always)]
    pub const fn mar31h(&self) -> &MAR31H {
        &self.mar31h
    }
    #[doc = "0x87c - MAC Address31 Register -Low"]
    #[inline(always)]
    pub const fn mar31l(&self) -> &MAR31L {
        &self.mar31l
    }
    #[doc = "0x1000 - Bus Mode Register"]
    #[inline(always)]
    pub const fn bmr(&self) -> &BMR {
        &self.bmr
    }
    #[doc = "0x1004 - Transmit Poll Demand Register)"]
    #[inline(always)]
    pub const fn tpdr(&self) -> &TPDR {
        &self.tpdr
    }
    #[doc = "0x1008 - Receive Poll Demand Register"]
    #[inline(always)]
    pub const fn rpdr(&self) -> &RPDR {
        &self.rpdr
    }
    #[doc = "0x100c - Receive Descriptor List Address Register)"]
    #[inline(always)]
    pub const fn rdlar(&self) -> &RDLAR {
        &self.rdlar
    }
    #[doc = "0x1010 - Transmit Descriptor List Address Register"]
    #[inline(always)]
    pub const fn tdlar(&self) -> &TDLAR {
        &self.tdlar
    }
    #[doc = "0x1014 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x1018 - Operation Mode Register"]
    #[inline(always)]
    pub const fn omr(&self) -> &OMR {
        &self.omr
    }
    #[doc = "0x101c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x1020 - Missed Frame and Buffer Overflow Counter Register"]
    #[inline(always)]
    pub const fn mfbocr(&self) -> &MFBOCR {
        &self.mfbocr
    }
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    #[inline(always)]
    pub const fn riwtr(&self) -> &RIWTR {
        &self.riwtr
    }
    #[doc = "0x102c - AHB Status Register"]
    #[inline(always)]
    pub const fn ahbsr(&self) -> &AHBSR {
        &self.ahbsr
    }
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    #[inline(always)]
    pub const fn chtdr(&self) -> &CHTDR {
        &self.chtdr
    }
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    #[inline(always)]
    pub const fn chrdr(&self) -> &CHRDR {
        &self.chrdr
    }
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    #[inline(always)]
    pub const fn chtbar(&self) -> &CHTBAR {
        &self.chtbar
    }
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    #[inline(always)]
    pub const fn chrbar(&self) -> &CHRBAR {
        &self.chrbar
    }
}
#[doc = "MCR (rw) register accessor: MAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "MAC Configuration Register"]
pub mod mcr;
#[doc = "MFFR (rw) register accessor: MAC Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mffr`]
module"]
pub type MFFR = crate::Reg<mffr::MFFR_SPEC>;
#[doc = "MAC Frame Filter Register"]
pub mod mffr;
#[doc = "MHTRH (rw) register accessor: MAC Hash Table Register (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mhtrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mhtrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mhtrh`]
module"]
pub type MHTRH = crate::Reg<mhtrh::MHTRH_SPEC>;
#[doc = "MAC Hash Table Register (High)"]
pub mod mhtrh;
#[doc = "MHTRL (rw) register accessor: MAC Hash Table Register (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mhtrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mhtrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mhtrl`]
module"]
pub type MHTRL = crate::Reg<mhtrl::MHTRL_SPEC>;
#[doc = "MAC Hash Table Register (Low)"]
pub mod mhtrl;
#[doc = "GAR (rw) register accessor: GMII/MII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gar`]
module"]
pub type GAR = crate::Reg<gar::GAR_SPEC>;
#[doc = "GMII/MII Address Register"]
pub mod gar;
#[doc = "GDR (rw) register accessor: GMII/MII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdr`]
module"]
pub type GDR = crate::Reg<gdr::GDR_SPEC>;
#[doc = "GMII/MII Data Register"]
pub mod gdr;
#[doc = "FCR (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "Flow Control Register"]
pub mod fcr;
#[doc = "VTR (rw) register accessor: VLAN TAG Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vtr`]
module"]
pub type VTR = crate::Reg<vtr::VTR_SPEC>;
#[doc = "VLAN TAG Register"]
pub mod vtr;
#[doc = "RWFFR (rw) register accessor: Remote Wake-up Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwffr`]
module"]
pub type RWFFR = crate::Reg<rwffr::RWFFR_SPEC>;
#[doc = "Remote Wake-up Frame Filter Register"]
pub mod rwffr;
#[doc = "PMTR (rw) register accessor: PMT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmtr`]
module"]
pub type PMTR = crate::Reg<pmtr::PMTR_SPEC>;
#[doc = "PMT Register"]
pub mod pmtr;
#[doc = "LPICSR (rw) register accessor: LPI Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpicsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpicsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpicsr`]
module"]
pub type LPICSR = crate::Reg<lpicsr::LPICSR_SPEC>;
#[doc = "LPI Control and Status Register"]
pub mod lpicsr;
#[doc = "LPITCR (rw) register accessor: LPI Timers Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpitcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpitcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpitcr`]
module"]
pub type LPITCR = crate::Reg<lpitcr::LPITCR_SPEC>;
#[doc = "LPI Timers Control Register"]
pub mod lpitcr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IMR (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAR0H (rw) register accessor: MAC Address0 Register (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mar0h`]
module"]
pub type MAR0H = crate::Reg<mar0h::MAR0H_SPEC>;
#[doc = "MAC Address0 Register (High)"]
pub mod mar0h;
#[doc = "MAR0L (rw) register accessor: MAC Address0 Register (Low)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mar0l`]
module"]
pub type MAR0L = crate::Reg<mar0l::MAR0L_SPEC>;
#[doc = "MAC Address0 Register (Low)"]
pub mod mar0l;
#[doc = "MAR1H (rw) register accessor: MAC Address1 Register -High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mar1h`]
module"]
pub type MAR1H = crate::Reg<mar1h::MAR1H_SPEC>;
#[doc = "MAC Address1 Register -High"]
pub mod mar1h;
#[doc = "MAR1L (rw) register accessor: MAC Address1 Register -Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mar1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mar1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mar1l`]
module"]
pub type MAR1L = crate::Reg<mar1l::MAR1L_SPEC>;
#[doc = "MAC Address1 Register -Low"]
pub mod mar1l;
pub use mar1h as mar2h;
pub use mar1h as mar3h;
pub use mar1h as mar4h;
pub use mar1h as mar5h;
pub use mar1h as mar6h;
pub use mar1h as mar7h;
pub use mar1h as mar8h;
pub use mar1h as mar9h;
pub use mar1h as mar10h;
pub use mar1h as mar11h;
pub use mar1h as mar12h;
pub use mar1h as mar13h;
pub use mar1h as mar14h;
pub use mar1h as mar15h;
pub use mar1l as mar2l;
pub use mar1l as mar3l;
pub use mar1l as mar4l;
pub use mar1l as mar5l;
pub use mar1l as mar6l;
pub use mar1l as mar7l;
pub use mar1l as mar8l;
pub use mar1l as mar9l;
pub use mar1l as mar10l;
pub use mar1l as mar11l;
pub use mar1l as mar12l;
pub use mar1l as mar13l;
pub use mar1l as mar14l;
pub use mar1l as mar15l;
pub use MAR1H as MAR2H;
pub use MAR1H as MAR3H;
pub use MAR1H as MAR4H;
pub use MAR1H as MAR5H;
pub use MAR1H as MAR6H;
pub use MAR1H as MAR7H;
pub use MAR1H as MAR8H;
pub use MAR1H as MAR9H;
pub use MAR1H as MAR10H;
pub use MAR1H as MAR11H;
pub use MAR1H as MAR12H;
pub use MAR1H as MAR13H;
pub use MAR1H as MAR14H;
pub use MAR1H as MAR15H;
pub use MAR1L as MAR2L;
pub use MAR1L as MAR3L;
pub use MAR1L as MAR4L;
pub use MAR1L as MAR5L;
pub use MAR1L as MAR6L;
pub use MAR1L as MAR7L;
pub use MAR1L as MAR8L;
pub use MAR1L as MAR9L;
pub use MAR1L as MAR10L;
pub use MAR1L as MAR11L;
pub use MAR1L as MAR12L;
pub use MAR1L as MAR13L;
pub use MAR1L as MAR14L;
pub use MAR1L as MAR15L;
#[doc = "mmc_cntl (rw) register accessor: MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_cntl`]
module"]
pub type MMC_CNTL = crate::Reg<mmc_cntl::MMC_CNTL_SPEC>;
#[doc = "MMC Control Register"]
pub mod mmc_cntl;
#[doc = "mmc_intr_rx (r) register accessor: Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_intr_rx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_rx`]
module"]
pub type MMC_INTR_RX = crate::Reg<mmc_intr_rx::MMC_INTR_RX_SPEC>;
#[doc = "Receive Interrupt Register"]
pub mod mmc_intr_rx;
#[doc = "mmc_intr_tx (r) register accessor: MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_intr_tx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_tx`]
module"]
pub type MMC_INTR_TX = crate::Reg<mmc_intr_tx::MMC_INTR_TX_SPEC>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_intr_tx;
#[doc = "mmc_intr_mask_rx (rw) register accessor: MMC Receive Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_intr_mask_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_intr_mask_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_mask_rx`]
module"]
pub type MMC_INTR_MASK_RX = crate::Reg<mmc_intr_mask_rx::MMC_INTR_MASK_RX_SPEC>;
#[doc = "MMC Receive Interrupt Mask Register"]
pub mod mmc_intr_mask_rx;
#[doc = "mmc_intr_mask_tx (rw) register accessor: MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_intr_mask_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_intr_mask_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_mask_tx`]
module"]
pub type MMC_INTR_MASK_TX = crate::Reg<mmc_intr_mask_tx::MMC_INTR_MASK_TX_SPEC>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_intr_mask_tx;
#[doc = "txoctetcount_gb (r) register accessor: \"Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txoctetcount_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoctetcount_gb`]
module"]
pub type TXOCTETCOUNT_GB = crate::Reg<txoctetcount_gb::TXOCTETCOUNT_GB_SPEC>;
#[doc = "\"Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames\""]
pub mod txoctetcount_gb;
#[doc = "txframecount_gb (r) register accessor: \"Number of good and bad frames transmitted, exclusive of retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txframecount_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txframecount_gb`]
module"]
pub type TXFRAMECOUNT_GB = crate::Reg<txframecount_gb::TXFRAMECOUNT_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted, exclusive of retried frames\""]
pub mod txframecount_gb;
#[doc = "txbroadcastframes_g (r) register accessor: Number of good broadcast frames transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbroadcastframes_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbroadcastframes_g`]
module"]
pub type TXBROADCASTFRAMES_G = crate::Reg<txbroadcastframes_g::TXBROADCASTFRAMES_G_SPEC>;
#[doc = "Number of good broadcast frames transmitted"]
pub mod txbroadcastframes_g;
#[doc = "txmulticastframes_g (r) register accessor: Number of good multicast frames transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmulticastframes_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmulticastframes_g`]
module"]
pub type TXMULTICASTFRAMES_G = crate::Reg<txmulticastframes_g::TXMULTICASTFRAMES_G_SPEC>;
#[doc = "Number of good multicast frames transmitted"]
pub mod txmulticastframes_g;
#[doc = "tx64octets_gb (r) register accessor: \"Number of good and bad frames transmitted with length of 64 bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx64octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx64octets_gb`]
module"]
pub type TX64OCTETS_GB = crate::Reg<tx64octets_gb::TX64OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted with length of 64 bytes, exclusive of preamble and retried frames\""]
pub mod tx64octets_gb;
#[doc = "tx65to127octets_gb (r) register accessor: \"Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx65to127octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx65to127octets_gb`]
module"]
pub type TX65TO127OCTETS_GB = crate::Reg<tx65to127octets_gb::TX65TO127OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames\""]
pub mod tx65to127octets_gb;
#[doc = "tx128to255octets_gb (r) register accessor: \"Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx128to255octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx128to255octets_gb`]
module"]
pub type TX128TO255OCTETS_GB = crate::Reg<tx128to255octets_gb::TX128TO255OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames\""]
pub mod tx128to255octets_gb;
#[doc = "tx256to511octets_gb (r) register accessor: \"Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx256to511octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx256to511octets_gb`]
module"]
pub type TX256TO511OCTETS_GB = crate::Reg<tx256to511octets_gb::TX256TO511OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames\""]
pub mod tx256to511octets_gb;
#[doc = "tx512to1023octets_gb (r) register accessor: \"Number of good and bad frames transmitted with length between 512 and 1023 (inclusive) bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx512to1023octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx512to1023octets_gb`]
module"]
pub type TX512TO1023OCTETS_GB = crate::Reg<tx512to1023octets_gb::TX512TO1023OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted with length between 512 and 1023 (inclusive) bytes, exclusive of preamble and retried frames\""]
pub mod tx512to1023octets_gb;
#[doc = "tx1024tomaxoctets_gb (r) register accessor: \"Number of good and bad frames transmitted with length between 1024 and Maxsize (inclusive) bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx1024tomaxoctets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx1024tomaxoctets_gb`]
module"]
pub type TX1024TOMAXOCTETS_GB = crate::Reg<tx1024tomaxoctets_gb::TX1024TOMAXOCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames transmitted with length between 1024 and Maxsize (inclusive) bytes, exclusive of preamble and retried frames\""]
pub mod tx1024tomaxoctets_gb;
#[doc = "txunicastframes_gb (r) register accessor: Number of good and bad unicast frames transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txunicastframes_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txunicastframes_gb`]
module"]
pub type TXUNICASTFRAMES_GB = crate::Reg<txunicastframes_gb::TXUNICASTFRAMES_GB_SPEC>;
#[doc = "Number of good and bad unicast frames transmitted"]
pub mod txunicastframes_gb;
#[doc = "txmulticastframes_gb (r) register accessor: Number of good and bad multicast frames transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmulticastframes_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmulticastframes_gb`]
module"]
pub type TXMULTICASTFRAMES_GB = crate::Reg<txmulticastframes_gb::TXMULTICASTFRAMES_GB_SPEC>;
#[doc = "Number of good and bad multicast frames transmitted"]
pub mod txmulticastframes_gb;
#[doc = "txbroadcastframes_gb (r) register accessor: Number of good and bad broadcast frames transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbroadcastframes_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbroadcastframes_gb`]
module"]
pub type TXBROADCASTFRAMES_GB = crate::Reg<txbroadcastframes_gb::TXBROADCASTFRAMES_GB_SPEC>;
#[doc = "Number of good and bad broadcast frames transmitted"]
pub mod txbroadcastframes_gb;
#[doc = "txunderflowerror (r) register accessor: Number of frames aborted due to frame underflow error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txunderflowerror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txunderflowerror`]
module"]
pub type TXUNDERFLOWERROR = crate::Reg<txunderflowerror::TXUNDERFLOWERROR_SPEC>;
#[doc = "Number of frames aborted due to frame underflow error"]
pub mod txunderflowerror;
#[doc = "txsinglecol_g (r) register accessor: Number of successfully transmitted frames after a single collision in Half-duplex mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txsinglecol_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txsinglecol_g`]
module"]
pub type TXSINGLECOL_G = crate::Reg<txsinglecol_g::TXSINGLECOL_G_SPEC>;
#[doc = "Number of successfully transmitted frames after a single collision in Half-duplex mode"]
pub mod txsinglecol_g;
#[doc = "txmulticol_g (r) register accessor: Number of successfully transmitted frames after more than a single collision in Half-duplex mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmulticol_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmulticol_g`]
module"]
pub type TXMULTICOL_G = crate::Reg<txmulticol_g::TXMULTICOL_G_SPEC>;
#[doc = "Number of successfully transmitted frames after more than a single collision in Half-duplex mode"]
pub mod txmulticol_g;
#[doc = "txdeferred (r) register accessor: Number of successfully transmitted frames after a deferral in Half-duplex mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdeferred::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdeferred`]
module"]
pub type TXDEFERRED = crate::Reg<txdeferred::TXDEFERRED_SPEC>;
#[doc = "Number of successfully transmitted frames after a deferral in Half-duplex mode."]
pub mod txdeferred;
#[doc = "txlatecol (r) register accessor: Number of frames aborted due to late collision error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlatecol::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlatecol`]
module"]
pub type TXLATECOL = crate::Reg<txlatecol::TXLATECOL_SPEC>;
#[doc = "Number of frames aborted due to late collision error."]
pub mod txlatecol;
#[doc = "txexesscol (r) register accessor: Number of frames aborted due to excessive (16) collision errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txexesscol::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txexesscol`]
module"]
pub type TXEXESSCOL = crate::Reg<txexesscol::TXEXESSCOL_SPEC>;
#[doc = "Number of frames aborted due to excessive (16) collision errors."]
pub mod txexesscol;
#[doc = "txcarriererror (r) register accessor: Number of frames aborted due to carrier sense error (no carrier or loss of carrier).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcarriererror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcarriererror`]
module"]
pub type TXCARRIERERROR = crate::Reg<txcarriererror::TXCARRIERERROR_SPEC>;
#[doc = "Number of frames aborted due to carrier sense error (no carrier or loss of carrier)."]
pub mod txcarriererror;
#[doc = "txoctetcount_g (r) register accessor: \"Number of bytes transmitted, exclusive of preamble, in good frames only. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txoctetcount_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoctetcount_g`]
module"]
pub type TXOCTETCOUNT_G = crate::Reg<txoctetcount_g::TXOCTETCOUNT_G_SPEC>;
#[doc = "\"Number of bytes transmitted, exclusive of preamble, in good frames only. \""]
pub mod txoctetcount_g;
#[doc = "txframecount_g (r) register accessor: Number of good frames transmitted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txframecount_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txframecount_g`]
module"]
pub type TXFRAMECOUNT_G = crate::Reg<txframecount_g::TXFRAMECOUNT_G_SPEC>;
#[doc = "Number of good frames transmitted."]
pub mod txframecount_g;
#[doc = "txexecessdef_g (r) register accessor: Number of frames aborted due to excessive deferral error (deferred for more than two max-sized frame times).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txexecessdef_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txexecessdef_g`]
module"]
pub type TXEXECESSDEF_G = crate::Reg<txexecessdef_g::TXEXECESSDEF_G_SPEC>;
#[doc = "Number of frames aborted due to excessive deferral error (deferred for more than two max-sized frame times)."]
pub mod txexecessdef_g;
#[doc = "txpauseframes (r) register accessor: Number of good PAUSE frames transmitted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpauseframes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpauseframes`]
module"]
pub type TXPAUSEFRAMES = crate::Reg<txpauseframes::TXPAUSEFRAMES_SPEC>;
#[doc = "Number of good PAUSE frames transmitted."]
pub mod txpauseframes;
#[doc = "txvlanframes_g (r) register accessor: \"Number of good VLAN frames transmitted, exclusive of retried frames. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvlanframes_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txvlanframes_g`]
module"]
pub type TXVLANFRAMES_G = crate::Reg<txvlanframes_g::TXVLANFRAMES_G_SPEC>;
#[doc = "\"Number of good VLAN frames transmitted, exclusive of retried frames. \""]
pub mod txvlanframes_g;
#[doc = "rxframecount_gb (r) register accessor: Number of good and bad frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxframecount_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxframecount_gb`]
module"]
pub type RXFRAMECOUNT_GB = crate::Reg<rxframecount_gb::RXFRAMECOUNT_GB_SPEC>;
#[doc = "Number of good and bad frames received."]
pub mod rxframecount_gb;
#[doc = "rxoctetcount_gb (r) register accessor: \"Number of bytes received, exclusive of preamble, in good and bad frames. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoctetcount_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoctetcount_gb`]
module"]
pub type RXOCTETCOUNT_GB = crate::Reg<rxoctetcount_gb::RXOCTETCOUNT_GB_SPEC>;
#[doc = "\"Number of bytes received, exclusive of preamble, in good and bad frames. \""]
pub mod rxoctetcount_gb;
#[doc = "rxoctetcount_g (r) register accessor: \"Number of bytes received, exclusive of preamble, only in good frames. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoctetcount_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoctetcount_g`]
module"]
pub type RXOCTETCOUNT_G = crate::Reg<rxoctetcount_g::RXOCTETCOUNT_G_SPEC>;
#[doc = "\"Number of bytes received, exclusive of preamble, only in good frames. \""]
pub mod rxoctetcount_g;
#[doc = "rxbroadcastframes_g (r) register accessor: Number of good broadcast frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbroadcastframes_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbroadcastframes_g`]
module"]
pub type RXBROADCASTFRAMES_G = crate::Reg<rxbroadcastframes_g::RXBROADCASTFRAMES_G_SPEC>;
#[doc = "Number of good broadcast frames received."]
pub mod rxbroadcastframes_g;
#[doc = "rxmulticastframes_g (r) register accessor: Number of good multicast frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmulticastframes_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmulticastframes_g`]
module"]
pub type RXMULTICASTFRAMES_G = crate::Reg<rxmulticastframes_g::RXMULTICASTFRAMES_G_SPEC>;
#[doc = "Number of good multicast frames received."]
pub mod rxmulticastframes_g;
#[doc = "rxcrcerror (r) register accessor: Number of frames received with CRC error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcerror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrcerror`]
module"]
pub type RXCRCERROR = crate::Reg<rxcrcerror::RXCRCERROR_SPEC>;
#[doc = "Number of frames received with CRC error."]
pub mod rxcrcerror;
#[doc = "rxallignmenterror (r) register accessor: Number of frames received with alignment (dribble) error. Valid only in 10/100 mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxallignmenterror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxallignmenterror`]
module"]
pub type RXALLIGNMENTERROR = crate::Reg<rxallignmenterror::RXALLIGNMENTERROR_SPEC>;
#[doc = "Number of frames received with alignment (dribble) error. Valid only in 10/100 mode."]
pub mod rxallignmenterror;
#[doc = "rxrunterror (r) register accessor: Number of frames received with runt (64 bytes and CRC error) error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxrunterror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxrunterror`]
module"]
pub type RXRUNTERROR = crate::Reg<rxrunterror::RXRUNTERROR_SPEC>;
#[doc = "Number of frames received with runt (64 bytes and CRC error) error."]
pub mod rxrunterror;
#[doc = "rxjabbererror (r) register accessor: Number of frames received with length greater than 1518 bytes with CRC error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxjabbererror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxjabbererror`]
module"]
pub type RXJABBERERROR = crate::Reg<rxjabbererror::RXJABBERERROR_SPEC>;
#[doc = "Number of frames received with length greater than 1518 bytes with CRC error."]
pub mod rxjabbererror;
#[doc = "rxundersize_g (r) register accessor: \"Number of frames received with length less than 64 bytes, without any errors. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxundersize_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxundersize_g`]
module"]
pub type RXUNDERSIZE_G = crate::Reg<rxundersize_g::RXUNDERSIZE_G_SPEC>;
#[doc = "\"Number of frames received with length less than 64 bytes, without any errors. \""]
pub mod rxundersize_g;
#[doc = "rxoversize_g (r) register accessor: Number of frames received with length greater than the maxsize without error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoversize_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoversize_g`]
module"]
pub type RXOVERSIZE_G = crate::Reg<rxoversize_g::RXOVERSIZE_G_SPEC>;
#[doc = "Number of frames received with length greater than the maxsize without error."]
pub mod rxoversize_g;
#[doc = "rx64octets_gb (r) register accessor: \"Number of good and bad frames received with length 64 bytes, exclusive of preamble. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx64octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx64octets_gb`]
module"]
pub type RX64OCTETS_GB = crate::Reg<rx64octets_gb::RX64OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames received with length 64 bytes, exclusive of preamble. \""]
pub mod rx64octets_gb;
#[doc = "rx65to127octets_gb (r) register accessor: \"Number of good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx65to127octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx65to127octets_gb`]
module"]
pub type RX65TO127OCTETS_GB = crate::Reg<rx65to127octets_gb::RX65TO127OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble. \""]
pub mod rx65to127octets_gb;
#[doc = "rx128to255octets_gb (r) register accessor: \"Number of good and bad frames received with length between 128 and 255 (inclusive) bytes, exclusive of preamble. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx128to255octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx128to255octets_gb`]
module"]
pub type RX128TO255OCTETS_GB = crate::Reg<rx128to255octets_gb::RX128TO255OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames received with length between 128 and 255 (inclusive) bytes, exclusive of preamble. \""]
pub mod rx128to255octets_gb;
#[doc = "rx256to511octets_gb (r) register accessor: \"Number of good and bad frames received with length between 256 and 511 (inclusive) bytes, exclusive of preamble. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx256to511octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx256to511octets_gb`]
module"]
pub type RX256TO511OCTETS_GB = crate::Reg<rx256to511octets_gb::RX256TO511OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames received with length between 256 and 511 (inclusive) bytes, exclusive of preamble. \""]
pub mod rx256to511octets_gb;
#[doc = "rx512to1023octets_gb (r) register accessor: \"Number of good and bad frames received with length between 512 and 1023 (inclusive) bytes, exclusive of preamble. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx512to1023octets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx512to1023octets_gb`]
module"]
pub type RX512TO1023OCTETS_GB = crate::Reg<rx512to1023octets_gb::RX512TO1023OCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames received with length between 512 and 1023 (inclusive) bytes, exclusive of preamble. \""]
pub mod rx512to1023octets_gb;
#[doc = "rx1024tomaxoctets_gb (r) register accessor: \"Number of good and bad frames received with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx1024tomaxoctets_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx1024tomaxoctets_gb`]
module"]
pub type RX1024TOMAXOCTETS_GB = crate::Reg<rx1024tomaxoctets_gb::RX1024TOMAXOCTETS_GB_SPEC>;
#[doc = "\"Number of good and bad frames received with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble. \""]
pub mod rx1024tomaxoctets_gb;
#[doc = "rxunicastframes_g (r) register accessor: Number of good unicast frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxunicastframes_g::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxunicastframes_g`]
module"]
pub type RXUNICASTFRAMES_G = crate::Reg<rxunicastframes_g::RXUNICASTFRAMES_G_SPEC>;
#[doc = "Number of good unicast frames received."]
pub mod rxunicastframes_g;
#[doc = "rxlengtherror (r) register accessor: \"Number of frames received with length error (Length type field is not the frame size), for all frames with valid length field. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlengtherror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlengtherror`]
module"]
pub type RXLENGTHERROR = crate::Reg<rxlengtherror::RXLENGTHERROR_SPEC>;
#[doc = "\"Number of frames received with length error (Length type field is not the frame size), for all frames with valid length field. \""]
pub mod rxlengtherror;
#[doc = "rxoutofrangetype (r) register accessor: Number of frames received with length/type field not equal to the valid frame size (>1500)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoutofrangetype::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoutofrangetype`]
module"]
pub type RXOUTOFRANGETYPE = crate::Reg<rxoutofrangetype::RXOUTOFRANGETYPE_SPEC>;
#[doc = "Number of frames received with length/type field not equal to the valid frame size (>1500)"]
pub mod rxoutofrangetype;
#[doc = "rxpauseframes (r) register accessor: Number of good and valid PAUSE frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpauseframes::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxpauseframes`]
module"]
pub type RXPAUSEFRAMES = crate::Reg<rxpauseframes::RXPAUSEFRAMES_SPEC>;
#[doc = "Number of good and valid PAUSE frames received."]
pub mod rxpauseframes;
#[doc = "rxfifooverflow (r) register accessor: Number of missed received frames due to FIFO overflow.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifooverflow::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifooverflow`]
module"]
pub type RXFIFOOVERFLOW = crate::Reg<rxfifooverflow::RXFIFOOVERFLOW_SPEC>;
#[doc = "Number of missed received frames due to FIFO overflow."]
pub mod rxfifooverflow;
#[doc = "rxvlanframes_gb (r) register accessor: Number of good and bad VLAN frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxvlanframes_gb::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxvlanframes_gb`]
module"]
pub type RXVLANFRAMES_GB = crate::Reg<rxvlanframes_gb::RXVLANFRAMES_GB_SPEC>;
#[doc = "Number of good and bad VLAN frames received."]
pub mod rxvlanframes_gb;
#[doc = "rxwatchdogerror (r) register accessor: Number of frames received with error due to watchdog timeout error (frames with a data load larger than 2048 bytes).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxwatchdogerror::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxwatchdogerror`]
module"]
pub type RXWATCHDOGERROR = crate::Reg<rxwatchdogerror::RXWATCHDOGERROR_SPEC>;
#[doc = "Number of frames received with error due to watchdog timeout error (frames with a data load larger than 2048 bytes)."]
pub mod rxwatchdogerror;
#[doc = "mmc_ipc_intr_mask_rx (rw) register accessor: MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_intr_mask_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_ipc_intr_mask_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_intr_mask_rx`]
module"]
pub type MMC_IPC_INTR_MASK_RX = crate::Reg<mmc_ipc_intr_mask_rx::MMC_IPC_INTR_MASK_RX_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_intr_mask_rx;
#[doc = "mmc_ipc_intr_rx (r) register accessor: MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_intr_rx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_intr_rx`]
module"]
pub type MMC_IPC_INTR_RX = crate::Reg<mmc_ipc_intr_rx::MMC_IPC_INTR_RX_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_intr_rx;
#[doc = "rxipv4_gd_frms (r) register accessor: \"Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_gd_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_gd_frms`]
module"]
pub type RXIPV4_GD_FRMS = crate::Reg<rxipv4_gd_frms::RXIPV4_GD_FRMS_SPEC>;
#[doc = "\"Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload \""]
pub mod rxipv4_gd_frms;
#[doc = "rxipv4_hdrerr_frms (r) register accessor: \"Number of IPv4 datagrams received with header errors (checksum, length, or version mismatch) \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_hdrerr_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_hdrerr_frms`]
module"]
pub type RXIPV4_HDRERR_FRMS = crate::Reg<rxipv4_hdrerr_frms::RXIPV4_HDRERR_FRMS_SPEC>;
#[doc = "\"Number of IPv4 datagrams received with header errors (checksum, length, or version mismatch) \""]
pub mod rxipv4_hdrerr_frms;
#[doc = "rxipv4_nopay_frms (r) register accessor: \"Number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_nopay_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_nopay_frms`]
module"]
pub type RXIPV4_NOPAY_FRMS = crate::Reg<rxipv4_nopay_frms::RXIPV4_NOPAY_FRMS_SPEC>;
#[doc = "\"Number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine \""]
pub mod rxipv4_nopay_frms;
#[doc = "rxipv4_frag_frms (r) register accessor: Number of good IPv4 datagrams with fragmentation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_frag_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_frag_frms`]
module"]
pub type RXIPV4_FRAG_FRMS = crate::Reg<rxipv4_frag_frms::RXIPV4_FRAG_FRMS_SPEC>;
#[doc = "Number of good IPv4 datagrams with fragmentation"]
pub mod rxipv4_frag_frms;
#[doc = "rxipv4_udsbl_frms (r) register accessor: Number of good IPv4 datagrams received that had a UDP payload with checksum disabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udsbl_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_udsbl_frms`]
module"]
pub type RXIPV4_UDSBL_FRMS = crate::Reg<rxipv4_udsbl_frms::RXIPV4_UDSBL_FRMS_SPEC>;
#[doc = "Number of good IPv4 datagrams received that had a UDP payload with checksum disabled"]
pub mod rxipv4_udsbl_frms;
#[doc = "rxipv6_gd_frms (r) register accessor: \"Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_gd_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_gd_frms`]
module"]
pub type RXIPV6_GD_FRMS = crate::Reg<rxipv6_gd_frms::RXIPV6_GD_FRMS_SPEC>;
#[doc = "\"Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads \""]
pub mod rxipv6_gd_frms;
#[doc = "rxipv6_hdrerr_frms (r) register accessor: Number of IPv6 datagrams received with header errors (length or version mismatch)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_hdrerr_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_hdrerr_frms`]
module"]
pub type RXIPV6_HDRERR_FRMS = crate::Reg<rxipv6_hdrerr_frms::RXIPV6_HDRERR_FRMS_SPEC>;
#[doc = "Number of IPv6 datagrams received with header errors (length or version mismatch)"]
pub mod rxipv6_hdrerr_frms;
#[doc = "rxipv6_nopay_frms (r) register accessor: \"Number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_nopay_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_nopay_frms`]
module"]
pub type RXIPV6_NOPAY_FRMS = crate::Reg<rxipv6_nopay_frms::RXIPV6_NOPAY_FRMS_SPEC>;
#[doc = "\"Number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers \""]
pub mod rxipv6_nopay_frms;
#[doc = "rxudp_gd_frms (r) register accessor: Number of good IP datagrams with a good UDP payload. This counter is not updated when the rxipv4_udsbl_frms counter is incremented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_gd_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_gd_frms`]
module"]
pub type RXUDP_GD_FRMS = crate::Reg<rxudp_gd_frms::RXUDP_GD_FRMS_SPEC>;
#[doc = "Number of good IP datagrams with a good UDP payload. This counter is not updated when the rxipv4_udsbl_frms counter is incremented."]
pub mod rxudp_gd_frms;
#[doc = "rxudp_err_frms (r) register accessor: Number of good IP datagrams whose UDP payload has a checksum error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_err_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_err_frms`]
module"]
pub type RXUDP_ERR_FRMS = crate::Reg<rxudp_err_frms::RXUDP_ERR_FRMS_SPEC>;
#[doc = "Number of good IP datagrams whose UDP payload has a checksum error"]
pub mod rxudp_err_frms;
#[doc = "rxtcp_gd_frms (r) register accessor: Number of good IP datagrams with a good TCP payload\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_gd_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_gd_frms`]
module"]
pub type RXTCP_GD_FRMS = crate::Reg<rxtcp_gd_frms::RXTCP_GD_FRMS_SPEC>;
#[doc = "Number of good IP datagrams with a good TCP payload"]
pub mod rxtcp_gd_frms;
#[doc = "rxtcp_err_frms (r) register accessor: Number of good IP datagrams whose TCP payload has a checksum error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_err_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_err_frms`]
module"]
pub type RXTCP_ERR_FRMS = crate::Reg<rxtcp_err_frms::RXTCP_ERR_FRMS_SPEC>;
#[doc = "Number of good IP datagrams whose TCP payload has a checksum error"]
pub mod rxtcp_err_frms;
#[doc = "rxicmp_gd_frms (r) register accessor: Number of good IP datagrams with a good ICMP payload\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_gd_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_gd_frms`]
module"]
pub type RXICMP_GD_FRMS = crate::Reg<rxicmp_gd_frms::RXICMP_GD_FRMS_SPEC>;
#[doc = "Number of good IP datagrams with a good ICMP payload"]
pub mod rxicmp_gd_frms;
#[doc = "rxicmp_err_frms (r) register accessor: Number of good IP datagrams whose ICMP payload has a checksum error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_err_frms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_err_frms`]
module"]
pub type RXICMP_ERR_FRMS = crate::Reg<rxicmp_err_frms::RXICMP_ERR_FRMS_SPEC>;
#[doc = "Number of good IP datagrams whose ICMP payload has a checksum error"]
pub mod rxicmp_err_frms;
#[doc = "rxipv4_gd_octets (r) register accessor: \"Number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. (Ethernet header, FCS, pad, or IP pad bytes are not included in this counter or in the octet counters listed below). \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_gd_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_gd_octets`]
module"]
pub type RXIPV4_GD_OCTETS = crate::Reg<rxipv4_gd_octets::RXIPV4_GD_OCTETS_SPEC>;
#[doc = "\"Number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. (Ethernet header, FCS, pad, or IP pad bytes are not included in this counter or in the octet counters listed below). \""]
pub mod rxipv4_gd_octets;
#[doc = "rxipv4_hdrerr_octets (r) register accessor: \"Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_hdrerr_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_hdrerr_octets`]
module"]
pub type RXIPV4_HDRERR_OCTETS = crate::Reg<rxipv4_hdrerr_octets::RXIPV4_HDRERR_OCTETS_SPEC>;
#[doc = "\"Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter. \""]
pub mod rxipv4_hdrerr_octets;
#[doc = "rxipv4_nopay_octets (r) register accessor: \"Number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 header's Length field is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_nopay_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_nopay_octets`]
module"]
pub type RXIPV4_NOPAY_OCTETS = crate::Reg<rxipv4_nopay_octets::RXIPV4_NOPAY_OCTETS_SPEC>;
#[doc = "\"Number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 header's Length field is used to update this counter. \""]
pub mod rxipv4_nopay_octets;
#[doc = "rxipv4_frag_octets (r) register accessor: Number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 header's Length field is used to update this counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_frag_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_frag_octets`]
module"]
pub type RXIPV4_FRAG_OCTETS = crate::Reg<rxipv4_frag_octets::RXIPV4_FRAG_OCTETS_SPEC>;
#[doc = "Number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 header's Length field is used to update this counter."]
pub mod rxipv4_frag_octets;
#[doc = "rxipv4_udsbl_octets (r) register accessor: Number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count IP Header bytes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_udsbl_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv4_udsbl_octets`]
module"]
pub type RXIPV4_UDSBL_OCTETS = crate::Reg<rxipv4_udsbl_octets::RXIPV4_UDSBL_OCTETS_SPEC>;
#[doc = "Number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count IP Header bytes."]
pub mod rxipv4_udsbl_octets;
#[doc = "rxipv6_gd_octets (r) register accessor: \"Number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_gd_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_gd_octets`]
module"]
pub type RXIPV6_GD_OCTETS = crate::Reg<rxipv6_gd_octets::RXIPV6_GD_OCTETS_SPEC>;
#[doc = "\"Number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data\""]
pub mod rxipv6_gd_octets;
#[doc = "rxipv6_hdrerr_octets (r) register accessor: \"Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_hdrerr_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_hdrerr_octets`]
module"]
pub type RXIPV6_HDRERR_OCTETS = crate::Reg<rxipv6_hdrerr_octets::RXIPV6_HDRERR_OCTETS_SPEC>;
#[doc = "\"Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter. \""]
pub mod rxipv6_hdrerr_octets;
#[doc = "rxipv6_nopay_octets (r) register accessor: \"Number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 header's Length field is used to update this counter. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_nopay_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxipv6_nopay_octets`]
module"]
pub type RXIPV6_NOPAY_OCTETS = crate::Reg<rxipv6_nopay_octets::RXIPV6_NOPAY_OCTETS_SPEC>;
#[doc = "\"Number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 header's Length field is used to update this counter. \""]
pub mod rxipv6_nopay_octets;
#[doc = "rxudp_gd_octets (r) register accessor: Number of bytes received in a good UDP segment. This counter (and the counters below) does not count IP header bytes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_gd_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_gd_octets`]
module"]
pub type RXUDP_GD_OCTETS = crate::Reg<rxudp_gd_octets::RXUDP_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in a good UDP segment. This counter (and the counters below) does not count IP header bytes."]
pub mod rxudp_gd_octets;
#[doc = "rxudp_err_octets (r) register accessor: Number of bytes received in a UDP segment that had checksum errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_err_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxudp_err_octets`]
module"]
pub type RXUDP_ERR_OCTETS = crate::Reg<rxudp_err_octets::RXUDP_ERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in a UDP segment that had checksum errors"]
pub mod rxudp_err_octets;
#[doc = "rxtcp_gd_octets (r) register accessor: Number of bytes received in a good TCP segment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_gd_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_gd_octets`]
module"]
pub type RXTCP_GD_OCTETS = crate::Reg<rxtcp_gd_octets::RXTCP_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in a good TCP segment"]
pub mod rxtcp_gd_octets;
#[doc = "rxtcp_err_octets (r) register accessor: Number of bytes received in a TCP segment with checksum errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtcp_err_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtcp_err_octets`]
module"]
pub type RXTCP_ERR_OCTETS = crate::Reg<rxtcp_err_octets::RXTCP_ERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in a TCP segment with checksum errors"]
pub mod rxtcp_err_octets;
#[doc = "rxicmp_gd_octets (r) register accessor: Number of bytes received in a good ICMP segment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_gd_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_gd_octets`]
module"]
pub type RXICMP_GD_OCTETS = crate::Reg<rxicmp_gd_octets::RXICMP_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in a good ICMP segment"]
pub mod rxicmp_gd_octets;
#[doc = "rxicmp_err_octets (r) register accessor: Number of bytes received in an ICMP segment with checksum errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxicmp_err_octets::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxicmp_err_octets`]
module"]
pub type RXICMP_ERR_OCTETS = crate::Reg<rxicmp_err_octets::RXICMP_ERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in an ICMP segment with checksum errors"]
pub mod rxicmp_err_octets;
#[doc = "RGSR (r) register accessor: RGMII Status Register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`]
module"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "RGMII Status Register)"]
pub mod rgsr;
#[doc = "TSCR (rw) register accessor: Time Stamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscr`]
module"]
pub type TSCR = crate::Reg<tscr::TSCR_SPEC>;
#[doc = "Time Stamp Control Register"]
pub mod tscr;
#[doc = "SSIR (rw) register accessor: Sub-Second Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssir`]
module"]
pub type SSIR = crate::Reg<ssir::SSIR_SPEC>;
#[doc = "Sub-Second Increment Register"]
pub mod ssir;
#[doc = "STSR (r) register accessor: System Time - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stsr`]
module"]
pub type STSR = crate::Reg<stsr::STSR_SPEC>;
#[doc = "System Time - Seconds Register"]
pub mod stsr;
#[doc = "STNR (r) register accessor: System Time - Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stnr`]
module"]
pub type STNR = crate::Reg<stnr::STNR_SPEC>;
#[doc = "System Time - Nanoseconds Register"]
pub mod stnr;
#[doc = "STSUR (rw) register accessor: System Time - Seconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stsur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stsur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stsur`]
module"]
pub type STSUR = crate::Reg<stsur::STSUR_SPEC>;
#[doc = "System Time - Seconds Update Register"]
pub mod stsur;
#[doc = "STNUR (rw) register accessor: System Time - Nanoseconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stnur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stnur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stnur`]
module"]
pub type STNUR = crate::Reg<stnur::STNUR_SPEC>;
#[doc = "System Time - Nanoseconds Update Register"]
pub mod stnur;
#[doc = "TSAR (rw) register accessor: Time Stamp Addend Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsar`]
module"]
pub type TSAR = crate::Reg<tsar::TSAR_SPEC>;
#[doc = "Time Stamp Addend Register"]
pub mod tsar;
#[doc = "TTSR (rw) register accessor: Target Time Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttsr`]
module"]
pub type TTSR = crate::Reg<ttsr::TTSR_SPEC>;
#[doc = "Target Time Seconds Register"]
pub mod ttsr;
#[doc = "TTNR (rw) register accessor: Target Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttnr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttnr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttnr`]
module"]
pub type TTNR = crate::Reg<ttnr::TTNR_SPEC>;
#[doc = "Target Time Nanoseconds Register"]
pub mod ttnr;
#[doc = "STHWSR (rw) register accessor: System Time - Higher Word Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthwsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthwsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthwsr`]
module"]
pub type STHWSR = crate::Reg<sthwsr::STHWSR_SPEC>;
#[doc = "System Time - Higher Word Seconds Register"]
pub mod sthwsr;
#[doc = "TSR (r) register accessor: Time Stamp Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`]
module"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Time Stamp Status Register"]
pub mod tsr;
#[doc = "PPSCR (rw) register accessor: PPS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppscr`]
module"]
pub type PPSCR = crate::Reg<ppscr::PPSCR_SPEC>;
#[doc = "PPS Control Register"]
pub mod ppscr;
#[doc = "ATNR (r) register accessor: Auxiliary Time Stamp - Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atnr`]
module"]
pub type ATNR = crate::Reg<atnr::ATNR_SPEC>;
#[doc = "Auxiliary Time Stamp - Nanoseconds Register"]
pub mod atnr;
#[doc = "ATSR (r) register accessor: Auxiliary Time Stamp - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atsr`]
module"]
pub type ATSR = crate::Reg<atsr::ATSR_SPEC>;
#[doc = "Auxiliary Time Stamp - Seconds Register"]
pub mod atsr;
pub use mar1h as mar16h;
pub use mar1h as mar17h;
pub use mar1h as mar18h;
pub use mar1h as mar19h;
pub use mar1h as mar20h;
pub use mar1h as mar21h;
pub use mar1h as mar22h;
pub use mar1h as mar23h;
pub use mar1h as mar24h;
pub use mar1h as mar25h;
pub use mar1h as mar26h;
pub use mar1h as mar27h;
pub use mar1h as mar28h;
pub use mar1h as mar29h;
pub use mar1h as mar30h;
pub use mar1h as mar31h;
pub use mar1l as mar16l;
pub use mar1l as mar17l;
pub use mar1l as mar18l;
pub use mar1l as mar19l;
pub use mar1l as mar20l;
pub use mar1l as mar21l;
pub use mar1l as mar22l;
pub use mar1l as mar23l;
pub use mar1l as mar24l;
pub use mar1l as mar25l;
pub use mar1l as mar26l;
pub use mar1l as mar27l;
pub use mar1l as mar28l;
pub use mar1l as mar29l;
pub use mar1l as mar30l;
pub use mar1l as mar31l;
pub use MAR1H as MAR16H;
pub use MAR1H as MAR17H;
pub use MAR1H as MAR18H;
pub use MAR1H as MAR19H;
pub use MAR1H as MAR20H;
pub use MAR1H as MAR21H;
pub use MAR1H as MAR22H;
pub use MAR1H as MAR23H;
pub use MAR1H as MAR24H;
pub use MAR1H as MAR25H;
pub use MAR1H as MAR26H;
pub use MAR1H as MAR27H;
pub use MAR1H as MAR28H;
pub use MAR1H as MAR29H;
pub use MAR1H as MAR30H;
pub use MAR1H as MAR31H;
pub use MAR1L as MAR16L;
pub use MAR1L as MAR17L;
pub use MAR1L as MAR18L;
pub use MAR1L as MAR19L;
pub use MAR1L as MAR20L;
pub use MAR1L as MAR21L;
pub use MAR1L as MAR22L;
pub use MAR1L as MAR23L;
pub use MAR1L as MAR24L;
pub use MAR1L as MAR25L;
pub use MAR1L as MAR26L;
pub use MAR1L as MAR27L;
pub use MAR1L as MAR28L;
pub use MAR1L as MAR29L;
pub use MAR1L as MAR30L;
pub use MAR1L as MAR31L;
#[doc = "BMR (rw) register accessor: Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmr`]
module"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "Bus Mode Register"]
pub mod bmr;
#[doc = "TPDR (rw) register accessor: Transmit Poll Demand Register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpdr`]
module"]
pub type TPDR = crate::Reg<tpdr::TPDR_SPEC>;
#[doc = "Transmit Poll Demand Register)"]
pub mod tpdr;
#[doc = "RPDR (rw) register accessor: Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpdr`]
module"]
pub type RPDR = crate::Reg<rpdr::RPDR_SPEC>;
#[doc = "Receive Poll Demand Register"]
pub mod rpdr;
#[doc = "RDLAR (rw) register accessor: Receive Descriptor List Address Register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdlar`]
module"]
pub type RDLAR = crate::Reg<rdlar::RDLAR_SPEC>;
#[doc = "Receive Descriptor List Address Register)"]
pub mod rdlar;
#[doc = "TDLAR (rw) register accessor: Transmit Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdlar`]
module"]
pub type TDLAR = crate::Reg<tdlar::TDLAR_SPEC>;
#[doc = "Transmit Descriptor List Address Register"]
pub mod tdlar;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "OMR (rw) register accessor: Operation Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omr`]
module"]
pub type OMR = crate::Reg<omr::OMR_SPEC>;
#[doc = "Operation Mode Register"]
pub mod omr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "MFBOCR (r) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfbocr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfbocr`]
module"]
pub type MFBOCR = crate::Reg<mfbocr::MFBOCR_SPEC>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod mfbocr;
#[doc = "RIWTR (r) register accessor: Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riwtr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@riwtr`]
module"]
pub type RIWTR = crate::Reg<riwtr::RIWTR_SPEC>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod riwtr;
#[doc = "AHBSR (r) register accessor: AHB Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbsr`]
module"]
pub type AHBSR = crate::Reg<ahbsr::AHBSR_SPEC>;
#[doc = "AHB Status Register"]
pub mod ahbsr;
#[doc = "CHTDR (r) register accessor: Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chtdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chtdr`]
module"]
pub type CHTDR = crate::Reg<chtdr::CHTDR_SPEC>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod chtdr;
#[doc = "CHRDR (r) register accessor: Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chrdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chrdr`]
module"]
pub type CHRDR = crate::Reg<chrdr::CHRDR_SPEC>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod chrdr;
#[doc = "CHTBAR (r) register accessor: Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chtbar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chtbar`]
module"]
pub type CHTBAR = crate::Reg<chtbar::CHTBAR_SPEC>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod chtbar;
#[doc = "CHRBAR (r) register accessor: Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chrbar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chrbar`]
module"]
pub type CHRBAR = crate::Reg<chrbar::CHRBAR_SPEC>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod chrbar;
