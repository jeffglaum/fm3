#[doc = "Register `rxframecount_gb` reader"]
pub type R = crate::R<RXFRAMECOUNT_GB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFRAMECOUNT_GB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good and bad frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxframecount_gb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFRAMECOUNT_GB_SPEC;
impl crate::RegisterSpec for RXFRAMECOUNT_GB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxframecount_gb::R`](R) reader structure"]
impl crate::Readable for RXFRAMECOUNT_GB_SPEC {}
#[doc = "`reset()` method sets rxframecount_gb to value 0"]
impl crate::Resettable for RXFRAMECOUNT_GB_SPEC {
    const RESET_VALUE: u32 = 0;
}
