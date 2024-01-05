#[doc = "Register `PWM_TMCR2` reader"]
pub type R = crate::R<PWM_PWM_TMCR2_SPEC>;
#[doc = "Register `PWM_TMCR2` writer"]
pub type W = crate::W<PWM_PWM_TMCR2_SPEC>;
#[doc = "Field `CKS3` reader - Count clock selection bit"]
pub type CKS3_R = crate::BitReader;
#[doc = "Field `CKS3` writer - Count clock selection bit"]
pub type CKS3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Count clock selection bit"]
    #[inline(always)]
    pub fn cks3(&self) -> CKS3_R {
        CKS3_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Count clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn cks3(&mut self) -> CKS3_W<PWM_PWM_TMCR2_SPEC> {
        CKS3_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_tmcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_tmcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_PWM_TMCR2_SPEC;
impl crate::RegisterSpec for PWM_PWM_TMCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwm_pwm_tmcr2::R`](R) reader structure"]
impl crate::Readable for PWM_PWM_TMCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm_tmcr2::W`](W) writer structure"]
impl crate::Writable for PWM_PWM_TMCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWM_TMCR2 to value 0"]
impl crate::Resettable for PWM_PWM_TMCR2_SPEC {
    const RESET_VALUE: u8 = 0;
}
