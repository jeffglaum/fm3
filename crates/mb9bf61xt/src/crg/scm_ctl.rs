#[doc = "Register `SCM_CTL` reader"]
pub type R = crate::R<SCM_CTL_SPEC>;
#[doc = "Register `SCM_CTL` writer"]
pub type W = crate::W<SCM_CTL_SPEC>;
#[doc = "Field `MOSCE` reader - Main clock oscillation enable bit"]
pub type MOSCE_R = crate::BitReader;
#[doc = "Field `MOSCE` writer - Main clock oscillation enable bit"]
pub type MOSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOSCE` reader - Sub clock oscillation enable bit"]
pub type SOSCE_R = crate::BitReader;
#[doc = "Field `SOSCE` writer - Sub clock oscillation enable bit"]
pub type SOSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLE` reader - PLL oscillation enable bit"]
pub type PLLE_R = crate::BitReader;
#[doc = "Field `PLLE` writer - PLL oscillation enable bit"]
pub type PLLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCS` reader - Master clock switch control bits"]
pub type RCS_R = crate::FieldReader;
#[doc = "Field `RCS` writer - Master clock switch control bits"]
pub type RCS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Main clock oscillation enable bit"]
    #[inline(always)]
    pub fn mosce(&self) -> MOSCE_R {
        MOSCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Sub clock oscillation enable bit"]
    #[inline(always)]
    pub fn sosce(&self) -> SOSCE_R {
        SOSCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL oscillation enable bit"]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Master clock switch control bits"]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Main clock oscillation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mosce(&mut self) -> MOSCE_W<SCM_CTL_SPEC> {
        MOSCE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Sub clock oscillation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sosce(&mut self) -> SOSCE_W<SCM_CTL_SPEC> {
        SOSCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - PLL oscillation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn plle(&mut self) -> PLLE_W<SCM_CTL_SPEC> {
        PLLE_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Master clock switch control bits"]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RCS_W<SCM_CTL_SPEC> {
        RCS_W::new(self, 5)
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
#[doc = "System Clock Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scm_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCM_CTL_SPEC;
impl crate::RegisterSpec for SCM_CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scm_ctl::R`](R) reader structure"]
impl crate::Readable for SCM_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scm_ctl::W`](W) writer structure"]
impl crate::Writable for SCM_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCM_CTL to value 0"]
impl crate::Resettable for SCM_CTL_SPEC {
    const RESET_VALUE: u8 = 0;
}
