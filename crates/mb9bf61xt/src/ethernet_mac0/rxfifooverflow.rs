#[doc = "Register `rxfifooverflow` reader"]
pub type R = crate::R<RXFIFOOVERFLOW_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXFIFOOVERFLOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of missed received frames due to FIFO overflow.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifooverflow::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIFOOVERFLOW_SPEC;
impl crate::RegisterSpec for RXFIFOOVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifooverflow::R`](R) reader structure"]
impl crate::Readable for RXFIFOOVERFLOW_SPEC {}
#[doc = "`reset()` method sets rxfifooverflow to value 0"]
impl crate::Resettable for RXFIFOOVERFLOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
