#[doc = "Register `txvlanframes_g` reader"]
pub type R = crate::R<TXVLANFRAMES_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXVLANFRAMES_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of good VLAN frames transmitted, exclusive of retried frames. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txvlanframes_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXVLANFRAMES_G_SPEC;
impl crate::RegisterSpec for TXVLANFRAMES_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txvlanframes_g::R`](R) reader structure"]
impl crate::Readable for TXVLANFRAMES_G_SPEC {}
#[doc = "`reset()` method sets txvlanframes_g to value 0"]
impl crate::Resettable for TXVLANFRAMES_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
