#[doc = "Register `RT_PCSR` reader"]
pub type R = crate::R<RT_RT_PCSR_SPEC>;
#[doc = "Register `RT_PCSR` writer"]
pub type W = crate::W<RT_RT_PCSR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RT_RT_PCSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Cycle Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_pcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_pcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RT_RT_PCSR_SPEC;
impl crate::RegisterSpec for RT_RT_PCSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rt_rt_pcsr::R`](R) reader structure"]
impl crate::Readable for RT_RT_PCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rt_rt_pcsr::W`](W) writer structure"]
impl crate::Writable for RT_RT_PCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RT_PCSR to value 0"]
impl crate::Resettable for RT_RT_PCSR_SPEC {
    const RESET_VALUE: u16 = 0;
}
