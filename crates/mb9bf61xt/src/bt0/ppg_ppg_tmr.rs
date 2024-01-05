#[doc = "Register `PPG_TMR` reader"]
pub type R = crate::R<PPG_PPG_TMR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PPG_PPG_TMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_tmr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPG_PPG_TMR_SPEC;
impl crate::RegisterSpec for PPG_PPG_TMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ppg_ppg_tmr::R`](R) reader structure"]
impl crate::Readable for PPG_PPG_TMR_SPEC {}
#[doc = "`reset()` method sets PPG_TMR to value 0"]
impl crate::Resettable for PPG_PPG_TMR_SPEC {
    const RESET_VALUE: u16 = 0;
}
