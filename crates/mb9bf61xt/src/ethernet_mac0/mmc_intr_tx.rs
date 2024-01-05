#[doc = "Register `mmc_intr_tx` reader"]
pub type R = crate::R<MMC_INTR_TX_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMC_INTR_TX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_intr_tx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_INTR_TX_SPEC;
impl crate::RegisterSpec for MMC_INTR_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_intr_tx::R`](R) reader structure"]
impl crate::Readable for MMC_INTR_TX_SPEC {}
#[doc = "`reset()` method sets mmc_intr_tx to value 0"]
impl crate::Resettable for MMC_INTR_TX_SPEC {
    const RESET_VALUE: u32 = 0;
}
