#[doc = "Register `txexecessdef_g` reader"]
pub type R = crate::R<TXEXECESSDEF_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXEXECESSDEF_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames aborted due to excessive deferral error (deferred for more than two max-sized frame times).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txexecessdef_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEXECESSDEF_G_SPEC;
impl crate::RegisterSpec for TXEXECESSDEF_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txexecessdef_g::R`](R) reader structure"]
impl crate::Readable for TXEXECESSDEF_G_SPEC {}
#[doc = "`reset()` method sets txexecessdef_g to value 0"]
impl crate::Resettable for TXEXECESSDEF_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
