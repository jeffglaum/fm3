#[doc = "Register `txmulticol_g` reader"]
pub type R = crate::R<TXMULTICOL_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMULTICOL_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of successfully transmitted frames after more than a single collision in Half-duplex mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmulticol_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMULTICOL_G_SPEC;
impl crate::RegisterSpec for TXMULTICOL_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmulticol_g::R`](R) reader structure"]
impl crate::Readable for TXMULTICOL_G_SPEC {}
#[doc = "`reset()` method sets txmulticol_g to value 0"]
impl crate::Resettable for TXMULTICOL_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
