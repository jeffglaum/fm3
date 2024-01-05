#[doc = "Register `TIMER1RIS` reader"]
pub type R = crate::R<TIMER1RIS_SPEC>;
#[doc = "Field `TIMER1RIS` reader - Interrupt Status Register bit"]
pub type TIMER1RIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status Register bit"]
    #[inline(always)]
    pub fn timer1ris(&self) -> TIMER1RIS_R {
        TIMER1RIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1RIS_SPEC;
impl crate::RegisterSpec for TIMER1RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1ris::R`](R) reader structure"]
impl crate::Readable for TIMER1RIS_SPEC {}
#[doc = "`reset()` method sets TIMER1RIS to value 0"]
impl crate::Resettable for TIMER1RIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
