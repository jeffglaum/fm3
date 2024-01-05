#[doc = "Register `PWC_STC` reader"]
pub type R = crate::R<PWC_PWC_STC_SPEC>;
#[doc = "Register `PWC_STC` writer"]
pub type W = crate::W<PWC_PWC_STC_SPEC>;
#[doc = "Field `OVIR` reader - Overflow interrupt request bit"]
pub type OVIR_R = crate::BitReader;
#[doc = "Field `OVIR` writer - Overflow interrupt request bit"]
pub type OVIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDIR` reader - Measurement completion interrupt request bit"]
pub type EDIR_R = crate::BitReader;
#[doc = "Field `OVIE` reader - Overflow interrupt request enable bit"]
pub type OVIE_R = crate::BitReader;
#[doc = "Field `OVIE` writer - Overflow interrupt request enable bit"]
pub type OVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDIE` reader - Measurement completion interrupt request enable bit"]
pub type EDIE_R = crate::BitReader;
#[doc = "Field `EDIE` writer - Measurement completion interrupt request enable bit"]
pub type EDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error flag bit"]
pub type ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overflow interrupt request bit"]
    #[inline(always)]
    pub fn ovir(&self) -> OVIR_R {
        OVIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Measurement completion interrupt request bit"]
    #[inline(always)]
    pub fn edir(&self) -> EDIR_R {
        EDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt request enable bit"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Measurement completion interrupt request enable bit"]
    #[inline(always)]
    pub fn edie(&self) -> EDIE_R {
        EDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error flag bit"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn ovir(&mut self) -> OVIR_W<PWC_PWC_STC_SPEC> {
        OVIR_W::new(self, 0)
    }
    #[doc = "Bit 4 - Overflow interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OVIE_W<PWC_PWC_STC_SPEC> {
        OVIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Measurement completion interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn edie(&mut self) -> EDIE_W<PWC_PWC_STC_SPEC> {
        EDIE_W::new(self, 6)
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
#[doc = "Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc_pwc_stc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc_pwc_stc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWC_PWC_STC_SPEC;
impl crate::RegisterSpec for PWC_PWC_STC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwc_pwc_stc::R`](R) reader structure"]
impl crate::Readable for PWC_PWC_STC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwc_pwc_stc::W`](W) writer structure"]
impl crate::Writable for PWC_PWC_STC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWC_STC to value 0"]
impl crate::Resettable for PWC_PWC_STC_SPEC {
    const RESET_VALUE: u8 = 0;
}
