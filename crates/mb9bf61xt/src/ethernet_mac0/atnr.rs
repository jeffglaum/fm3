#[doc = "Register `ATNR` reader"]
pub type R = crate::R<ATNR_SPEC>;
#[doc = "Field `ATN` reader - ATN"]
pub type ATN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - ATN"]
    #[inline(always)]
    pub fn atn(&self) -> ATN_R {
        ATN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "Auxiliary Time Stamp - Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATNR_SPEC;
impl crate::RegisterSpec for ATNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atnr::R`](R) reader structure"]
impl crate::Readable for ATNR_SPEC {}
#[doc = "`reset()` method sets ATNR to value 0"]
impl crate::Resettable for ATNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
