#[doc = "Register `FCSWD_CTL` reader"]
pub type R = crate::R<FCSWD_CTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FCSWD_CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Frequency detection counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcswd_ctl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCSWD_CTL_SPEC;
impl crate::RegisterSpec for FCSWD_CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcswd_ctl::R`](R) reader structure"]
impl crate::Readable for FCSWD_CTL_SPEC {}
#[doc = "`reset()` method sets FCSWD_CTL to value 0"]
impl crate::Resettable for FCSWD_CTL_SPEC {
    const RESET_VALUE: u16 = 0;
}
