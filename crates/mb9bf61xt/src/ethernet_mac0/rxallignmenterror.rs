#[doc = "Register `rxallignmenterror` reader"]
pub type R = crate::R<RXALLIGNMENTERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXALLIGNMENTERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with alignment (dribble) error. Valid only in 10/100 mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxallignmenterror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXALLIGNMENTERROR_SPEC;
impl crate::RegisterSpec for RXALLIGNMENTERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxallignmenterror::R`](R) reader structure"]
impl crate::Readable for RXALLIGNMENTERROR_SPEC {}
#[doc = "`reset()` method sets rxallignmenterror to value 0"]
impl crate::Resettable for RXALLIGNMENTERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
