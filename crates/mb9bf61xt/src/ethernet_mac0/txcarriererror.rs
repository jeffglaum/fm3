#[doc = "Register `txcarriererror` reader"]
pub type R = crate::R<TXCARRIERERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXCARRIERERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames aborted due to carrier sense error (no carrier or loss of carrier).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcarriererror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCARRIERERROR_SPEC;
impl crate::RegisterSpec for TXCARRIERERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcarriererror::R`](R) reader structure"]
impl crate::Readable for TXCARRIERERROR_SPEC {}
#[doc = "`reset()` method sets txcarriererror to value 0"]
impl crate::Resettable for TXCARRIERERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
