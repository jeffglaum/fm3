#[doc = "Register `RIWTR` reader"]
pub type R = crate::R<RIWTR_SPEC>;
#[doc = "Field `RIWT` reader - RI Watchdog Timer count"]
pub type RIWT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer count"]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riwtr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIWTR_SPEC;
impl crate::RegisterSpec for RIWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riwtr::R`](R) reader structure"]
impl crate::Readable for RIWTR_SPEC {}
#[doc = "`reset()` method sets RIWTR to value 0"]
impl crate::Resettable for RIWTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
