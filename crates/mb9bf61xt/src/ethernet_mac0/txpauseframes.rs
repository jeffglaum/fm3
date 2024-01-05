#[doc = "Register `txpauseframes` reader"]
pub type R = crate::R<TXPAUSEFRAMES_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXPAUSEFRAMES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good PAUSE frames transmitted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpauseframes::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPAUSEFRAMES_SPEC;
impl crate::RegisterSpec for TXPAUSEFRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpauseframes::R`](R) reader structure"]
impl crate::Readable for TXPAUSEFRAMES_SPEC {}
#[doc = "`reset()` method sets txpauseframes to value 0"]
impl crate::Resettable for TXPAUSEFRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
