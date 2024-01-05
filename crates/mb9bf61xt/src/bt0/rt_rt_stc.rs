#[doc = "Register `RT_STC` reader"]
pub type R = crate::R<RT_RT_STC_SPEC>;
#[doc = "Register `RT_STC` writer"]
pub type W = crate::W<RT_RT_STC_SPEC>;
#[doc = "Field `UDIR` reader - Underflow interrupt request bit"]
pub type UDIR_R = crate::BitReader;
#[doc = "Field `UDIR` writer - Underflow interrupt request bit"]
pub type UDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGIR` reader - Trigger interrupt request bit"]
pub type TGIR_R = crate::BitReader;
#[doc = "Field `TGIR` writer - Trigger interrupt request bit"]
pub type TGIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIE` reader - Underflow interrupt request enable bit"]
pub type UDIE_R = crate::BitReader;
#[doc = "Field `UDIE` writer - Underflow interrupt request enable bit"]
pub type UDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn udir(&mut self) -> UDIR_W<RT_RT_STC_SPEC> {
        UDIR_W::new(self, 0)
    }
    #[doc = "Bit 2 - Trigger interrupt request bit"]
    #[inline(always)]
    #[must_use]
    pub fn tgir(&mut self) -> TGIR_W<RT_RT_STC_SPEC> {
        TGIR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Underflow interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn udie(&mut self) -> UDIE_W<RT_RT_STC_SPEC> {
        UDIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt request enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tgie(&mut self) -> TGIE_W<RT_RT_STC_SPEC> {
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
#[doc = "Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_stc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_stc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RT_RT_STC_SPEC;
impl crate::RegisterSpec for RT_RT_STC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rt_rt_stc::R`](R) reader structure"]
impl crate::Readable for RT_RT_STC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rt_rt_stc::W`](W) writer structure"]
impl crate::Writable for RT_RT_STC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RT_STC to value 0"]
impl crate::Resettable for RT_RT_STC_SPEC {
    const RESET_VALUE: u8 = 0;
}
