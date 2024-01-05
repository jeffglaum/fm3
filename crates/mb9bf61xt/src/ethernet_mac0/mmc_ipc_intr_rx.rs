#[doc = "Register `mmc_ipc_intr_rx` reader"]
pub type R = crate::R<MMC_IPC_INTR_RX_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMC_IPC_INTR_RX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_intr_rx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_IPC_INTR_RX_SPEC;
impl crate::RegisterSpec for MMC_IPC_INTR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_ipc_intr_rx::R`](R) reader structure"]
impl crate::Readable for MMC_IPC_INTR_RX_SPEC {}
#[doc = "`reset()` method sets mmc_ipc_intr_rx to value 0"]
impl crate::Resettable for MMC_IPC_INTR_RX_SPEC {
    const RESET_VALUE: u32 = 0;
}
