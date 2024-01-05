#[doc = "Register `txframecount_gb` reader"]
pub type R = crate::R<TXFRAMECOUNT_GB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXFRAMECOUNT_GB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of good and bad frames transmitted, exclusive of retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txframecount_gb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFRAMECOUNT_GB_SPEC;
impl crate::RegisterSpec for TXFRAMECOUNT_GB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txframecount_gb::R`](R) reader structure"]
impl crate::Readable for TXFRAMECOUNT_GB_SPEC {}
#[doc = "`reset()` method sets txframecount_gb to value 0"]
impl crate::Resettable for TXFRAMECOUNT_GB_SPEC {
    const RESET_VALUE: u32 = 0;
}
