#[doc = "Register `txexesscol` reader"]
pub type R = crate::R<TXEXESSCOL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXEXESSCOL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames aborted due to excessive (16) collision errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txexesscol::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEXESSCOL_SPEC;
impl crate::RegisterSpec for TXEXESSCOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txexesscol::R`](R) reader structure"]
impl crate::Readable for TXEXESSCOL_SPEC {}
#[doc = "`reset()` method sets txexesscol to value 0"]
impl crate::Resettable for TXEXESSCOL_SPEC {
    const RESET_VALUE: u32 = 0;
}
