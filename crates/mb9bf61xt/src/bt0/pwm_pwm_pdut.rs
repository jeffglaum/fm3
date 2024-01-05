#[doc = "Register `PWM_PDUT` reader"]
pub type R = crate::R<PWM_PWM_PDUT_SPEC>;
#[doc = "Register `PWM_PDUT` writer"]
pub type W = crate::W<PWM_PWM_PDUT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PWM_PWM_PDUT_SPEC> {
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
#[doc = "PWM Duty Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_pdut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_pdut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_PWM_PDUT_SPEC;
impl crate::RegisterSpec for PWM_PWM_PDUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pwm_pwm_pdut::R`](R) reader structure"]
impl crate::Readable for PWM_PWM_PDUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm_pdut::W`](W) writer structure"]
impl crate::Writable for PWM_PWM_PDUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PWM_PDUT to value 0"]
impl crate::Resettable for PWM_PWM_PDUT_SPEC {
    const RESET_VALUE: u16 = 0;
}
