#[doc = "Register `txlatecol` reader"]
pub type R = crate::R<TXLATECOL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXLATECOL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames aborted due to late collision error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlatecol::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLATECOL_SPEC;
impl crate::RegisterSpec for TXLATECOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlatecol::R`](R) reader structure"]
impl crate::Readable for TXLATECOL_SPEC {}
#[doc = "`reset()` method sets txlatecol to value 0"]
impl crate::Resettable for TXLATECOL_SPEC {
    const RESET_VALUE: u32 = 0;
}
