#[doc = "Register `rxipv4_hdrerr_frms` reader"]
pub type R = crate::R<RXIPV4_HDRERR_FRMS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXIPV4_HDRERR_FRMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of IPv4 datagrams received with header errors (checksum, length, or version mismatch) \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv4_hdrerr_frms::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXIPV4_HDRERR_FRMS_SPEC;
impl crate::RegisterSpec for RXIPV4_HDRERR_FRMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv4_hdrerr_frms::R`](R) reader structure"]
impl crate::Readable for RXIPV4_HDRERR_FRMS_SPEC {}
#[doc = "`reset()` method sets rxipv4_hdrerr_frms to value 0"]
impl crate::Resettable for RXIPV4_HDRERR_FRMS_SPEC {
    const RESET_VALUE: u32 = 0;
}
