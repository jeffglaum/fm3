#[doc = "Register `rxudp_err_octets` reader"]
pub type R = crate::R<RXUDP_ERR_OCTETS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUDP_ERR_OCTETS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of bytes received in a UDP segment that had checksum errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxudp_err_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUDP_ERR_OCTETS_SPEC;
impl crate::RegisterSpec for RXUDP_ERR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxudp_err_octets::R`](R) reader structure"]
impl crate::Readable for RXUDP_ERR_OCTETS_SPEC {}
#[doc = "`reset()` method sets rxudp_err_octets to value 0"]
impl crate::Resettable for RXUDP_ERR_OCTETS_SPEC {
    const RESET_VALUE: u32 = 0;
}
