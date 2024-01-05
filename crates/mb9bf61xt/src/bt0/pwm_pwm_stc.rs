#[doc = "Register `PWM_STC` reader"]
pub type R = crate::R<PWM_PWM_STC_SPEC>;
#[doc = "Register `PWM_STC` writer"]
pub type W = crate::W<PWM_PWM_STC_SPEC>;
#[doc = "Field `UDIR` reader - Underflow interrupt request bit"]
pub type UDIR_R = crate::BitReader;
#[doc = "Field `UDIR` writer - Underflow interrupt request bit"]
pub type UDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIR` reader - Duty match interrupt request bit"]
pub type DTIR_R = crate::BitReader;
#[doc = "Field `DTIR` writer - Duty match interrupt request bit"]
pub type DTIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGIR` reader - Trigger interrupt request bit"]
pub type TGIR_R = crate::BitReader;
#[doc = "Field `TGIR` writer - Trigger interrupt request bit"]
pub type TGIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIE` reader - Underflow interrupt request enable bit"]
pub type UDIE_R = crate::BitReader;
#[doc = "Field `UDIE` writer - Underflow interrupt request enable bit"]
pub type UDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIE` reader - Duty match interrupt request enable bit"]
pub type DTIE_R = crate::BitReader;
#[doc = "Field `DTIE` writer - Duty match interrupt request enable bit"]
pub type DTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGIE` reader - Trigger interrupt request enable bit"]
pub type TGIE_R = crate::BitReader;
#[doc = "Field `TGIE` writer - Trigger interrupt request enable bit"]
pub type TGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Underflow interrupt request bit"]
    #[inline(always)]
    pub fn udir(&self) -> UDIR_R {
        UDIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Duty match interrupt request bit"]
    #[inline(always)]
    pub fn dtir(&self) -> DTIR_R {
        DTIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger interrupt request bit"]
    #[inline(always)]
    pub fn tgir(&self) -> TGIR_R {
        TGIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Underflow interrupt request enable bit"]
    #[inline(always)]
    pub fn udie(&self) -> UDIE_R {
        UDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Duty match interrupt request enable bit"]
    #[inline(always)]
    pub fn dtie(&self) -> DTIE_R {
        DTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt request enable bit"]
    #[inline(always)]
    pub fn tgie(&self) -> TGIE_R {
        TGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underflow interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn udir(&mut self) -> UDIR_W<PWM_PWM_STC_SPEC> {
        UDIR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Duty match interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtir(&mut self) -> DTIR_W<PWM_PWM_STC_SPEC> {
        DTIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Trigger interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn tgir(&mut self) -> TGIR_W<PWM_PWM_STC_SPEC> {
        TGIR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Underflow interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn udie(&mut self) -> UDIE_W<PWM_PWM_STC_SPEC> {
        UDIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Duty match interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtie(&mut self) -> DTIE_W<PWM_PWM_STC_SPEC> {
        DTIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tgie(&mut self) -> TGIE_W<PWM_PWM_STC_SPEC> {
        TGIE_W::new(self, 6)
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
#[doc = "Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_stc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_stc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_PWM_STC_SPEC;
impl crate::RegisterSpec for PWM_PWM_STC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwm_pwm_stc::R`](R) reader structure"]
impl crate::Readable for PWM_PWM_STC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_pwm_stc::W`](W) writer structure"]
impl crate::Writable for PWM_PWM_STC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWM_STC to value 0"]
impl crate::Resettable for PWM_PWM_STC_SPEC {
    const RESET_VALUE: u8 = 0;
}
