#[doc = "Register `mmc_intr_mask_tx` reader"]
pub type R = crate::R<MMC_INTR_MASK_TX_SPEC>;
#[doc = "Register `mmc_intr_mask_tx` writer"]
pub type W = crate::W<MMC_INTR_MASK_TX_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMC_INTR_MASK_TX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_intr_mask_tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_intr_mask_tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_INTR_MASK_TX_SPEC;
impl crate::RegisterSpec for MMC_INTR_MASK_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_intr_mask_tx::R`](R) reader structure"]
impl crate::Readable for MMC_INTR_MASK_TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_intr_mask_tx::W`](W) writer structure"]
impl crate::Writable for MMC_INTR_MASK_TX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mmc_intr_mask_tx to value 0"]
impl crate::Resettable for MMC_INTR_MASK_TX_SPEC {
    const RESET_VALUE: u32 = 0;
}
