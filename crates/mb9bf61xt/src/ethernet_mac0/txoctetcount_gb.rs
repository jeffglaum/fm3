#[doc = "Register `txoctetcount_gb` reader"]
pub type R = crate::R<TXOCTETCOUNT_GB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXOCTETCOUNT_GB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txoctetcount_gb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXOCTETCOUNT_GB_SPEC;
impl crate::RegisterSpec for TXOCTETCOUNT_GB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoctetcount_gb::R`](R) reader structure"]
impl crate::Readable for TXOCTETCOUNT_GB_SPEC {}
#[doc = "`reset()` method sets txoctetcount_gb to value 0"]
impl crate::Resettable for TXOCTETCOUNT_GB_SPEC {
    const RESET_VALUE: u32 = 0;
}
