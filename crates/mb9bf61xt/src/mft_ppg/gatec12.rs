#[doc = "Register `GATEC12` reader"]
pub type R = crate::R<GATEC12_SPEC>;
#[doc = "Register `GATEC12` writer"]
pub type W = crate::W<GATEC12_SPEC>;
#[doc = "Field `EDGE12` reader - Select Start Effective Level for PPG12"]
pub type EDGE12_R = crate::BitReader;
#[doc = "Field `EDGE12` writer - Select Start Effective Level for PPG12"]
pub type EDGE12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG12` reader - Select a trigger for PPG12"]
pub type STRG12_R = crate::BitReader;
#[doc = "Field `STRG12` writer - Select a trigger for PPG12"]
pub type STRG12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE14` reader - Select Start Effective Level for PPG14"]
pub type EDGE14_R = crate::BitReader;
#[doc = "Field `EDGE14` writer - Select Start Effective Level for PPG14"]
pub type EDGE14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG14` reader - Select a trigger for PPG14"]
pub type STRG14_R = crate::BitReader;
#[doc = "Field `STRG14` writer - Select a trigger for PPG14"]
pub type STRG14_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Start Effective Level for PPG12"]
    #[inline(always)]
    pub fn edge12(&self) -> EDGE12_R {
        EDGE12_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG12"]
    #[inline(always)]
    pub fn strg12(&self) -> STRG12_R {
        STRG12_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG14"]
    #[inline(always)]
    pub fn edge14(&self) -> EDGE14_R {
        EDGE14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select a trigger for PPG14"]
    #[inline(always)]
    pub fn strg14(&self) -> STRG14_R {
        STRG14_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Start Effective Level for PPG12"]
    #[inline(always)]
    #[must_use]
    pub fn edge12(&mut self) -> EDGE12_W<GATEC12_SPEC> {
        EDGE12_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG12"]
    #[inline(always)]
    #[must_use]
    pub fn strg12(&mut self) -> STRG12_W<GATEC12_SPEC> {
        STRG12_W::new(self, 1)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG14"]
    #[inline(always)]
    #[must_use]
    pub fn edge14(&mut self) -> EDGE14_W<GATEC12_SPEC> {
        EDGE14_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select a trigger for PPG14"]
    #[inline(always)]
    #[must_use]
    pub fn strg14(&mut self) -> STRG14_W<GATEC12_SPEC> {
        STRG14_W::new(self, 5)
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
#[doc = "PPG Gate Function Control Registers 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GATEC12_SPEC;
impl crate::RegisterSpec for GATEC12_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gatec12::R`](R) reader structure"]
impl crate::Readable for GATEC12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gatec12::W`](W) writer structure"]
impl crate::Writable for GATEC12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATEC12 to value 0"]
impl crate::Resettable for GATEC12_SPEC {
    const RESET_VALUE: u8 = 0;
}
