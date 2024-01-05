#[doc = "Register `TIMER1MIS` reader"]
pub type R = crate::R<TIMER1MIS_SPEC>;
#[doc = "Field `TIMER1MIS` reader - Masked Interrupt Status bit"]
pub type TIMER1MIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked Interrupt Status bit"]
    #[inline(always)]
    pub fn timer1mis(&self) -> TIMER1MIS_R {
        TIMER1MIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1MIS_SPEC;
impl crate::RegisterSpec for TIMER1MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1mis::R`](R) reader structure"]
impl crate::Readable for TIMER1MIS_SPEC {}
#[doc = "`reset()` method sets TIMER1MIS to value 0"]
impl crate::Resettable for TIMER1MIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
