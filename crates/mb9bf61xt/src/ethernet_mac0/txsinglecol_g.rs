#[doc = "Register `txsinglecol_g` reader"]
pub type R = crate::R<TXSINGLECOL_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXSINGLECOL_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of successfully transmitted frames after a single collision in Half-duplex mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txsinglecol_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXSINGLECOL_G_SPEC;
impl crate::RegisterSpec for TXSINGLECOL_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txsinglecol_g::R`](R) reader structure"]
impl crate::Readable for TXSINGLECOL_G_SPEC {}
#[doc = "`reset()` method sets txsinglecol_g to value 0"]
impl crate::Resettable for TXSINGLECOL_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
