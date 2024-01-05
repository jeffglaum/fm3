#[doc = "Register `tx128to255octets_gb` reader"]
pub type R = crate::R<TX128TO255OCTETS_GB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX128TO255OCTETS_GB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx128to255octets_gb::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX128TO255OCTETS_GB_SPEC;
impl crate::RegisterSpec for TX128TO255OCTETS_GB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx128to255octets_gb::R`](R) reader structure"]
impl crate::Readable for TX128TO255OCTETS_GB_SPEC {}
#[doc = "`reset()` method sets tx128to255octets_gb to value 0"]
impl crate::Resettable for TX128TO255OCTETS_GB_SPEC {
    const RESET_VALUE: u32 = 0;
}
