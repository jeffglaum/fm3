#[doc = "Register `GATEC20` reader"]
pub type R = crate::R<GATEC20_SPEC>;
#[doc = "Register `GATEC20` writer"]
pub type W = crate::W<GATEC20_SPEC>;
#[doc = "Field `EDGE20` reader - Select Start Effective Level for PPG20"]
pub type EDGE20_R = crate::BitReader;
#[doc = "Field `EDGE20` writer - Select Start Effective Level for PPG20"]
pub type EDGE20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG20` reader - Select a trigger for PPG20"]
pub type STRG20_R = crate::BitReader;
#[doc = "Field `STRG20` writer - Select a trigger for PPG20"]
pub type STRG20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE22` reader - Select Start Effective Level for PPG22"]
pub type EDGE22_R = crate::BitReader;
#[doc = "Field `EDGE22` writer - Select Start Effective Level for PPG22"]
pub type EDGE22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG22` reader - Select a trigger for PPG22"]
pub type STRG22_R = crate::BitReader;
#[doc = "Field `STRG22` writer - Select a trigger for PPG22"]
pub type STRG22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Start Effective Level for PPG20"]
    #[inline(always)]
    pub fn edge20(&self) -> EDGE20_R {
        EDGE20_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG20"]
    #[inline(always)]
    pub fn strg20(&self) -> STRG20_R {
        STRG20_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG22"]
    #[inline(always)]
    pub fn edge22(&self) -> EDGE22_R {
        EDGE22_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select a trigger for PPG22"]
    #[inline(always)]
    pub fn strg22(&self) -> STRG22_R {
        STRG22_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Start Effective Level for PPG20"]
    #[inline(always)]
    #[must_use]
    pub fn edge20(&mut self) -> EDGE20_W<GATEC20_SPEC> {
        EDGE20_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG20"]
    #[inline(always)]
    #[must_use]
    pub fn strg20(&mut self) -> STRG20_W<GATEC20_SPEC> {
        STRG20_W::new(self, 1)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG22"]
    #[inline(always)]
    #[must_use]
    pub fn edge22(&mut self) -> EDGE22_W<GATEC20_SPEC> {
        EDGE22_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select a trigger for PPG22"]
    #[inline(always)]
    #[must_use]
    pub fn strg22(&mut self) -> STRG22_W<GATEC20_SPEC> {
        STRG22_W::new(self, 5)
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
#[doc = "PPG Gate Function Control Registers 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GATEC20_SPEC;
impl crate::RegisterSpec for GATEC20_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gatec20::R`](R) reader structure"]
impl crate::Readable for GATEC20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gatec20::W`](W) writer structure"]
impl crate::Writable for GATEC20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATEC20 to value 0"]
impl crate::Resettable for GATEC20_SPEC {
    const RESET_VALUE: u8 = 0;
}
