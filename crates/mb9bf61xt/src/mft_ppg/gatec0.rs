#[doc = "Register `GATEC0` reader"]
pub type R = crate::R<GATEC0_SPEC>;
#[doc = "Register `GATEC0` writer"]
pub type W = crate::W<GATEC0_SPEC>;
#[doc = "Field `EDGE0` reader - Select Start Effective Level for PPG0"]
pub type EDGE0_R = crate::BitReader;
#[doc = "Field `EDGE0` writer - Select Start Effective Level for PPG0"]
pub type EDGE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG0` reader - Select a trigger for PPG0"]
pub type STRG0_R = crate::BitReader;
#[doc = "Field `STRG0` writer - Select a trigger for PPG0"]
pub type STRG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE2` reader - Select Start Effective Level for PPG2"]
pub type EDGE2_R = crate::BitReader;
#[doc = "Field `EDGE2` writer - Select Start Effective Level for PPG2"]
pub type EDGE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG2` reader - Select a trigger for PPG2"]
pub type STRG2_R = crate::BitReader;
#[doc = "Field `STRG2` writer - Select a trigger for PPG2"]
pub type STRG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Start Effective Level for PPG0"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG0"]
    #[inline(always)]
    pub fn strg0(&self) -> STRG0_R {
        STRG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select a trigger for PPG2"]
    #[inline(always)]
    pub fn strg2(&self) -> STRG2_R {
        STRG2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Start Effective Level for PPG0"]
    #[inline(always)]
    #[must_use]
    pub fn edge0(&mut self) -> EDGE0_W<GATEC0_SPEC> {
        EDGE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG0"]
    #[inline(always)]
    #[must_use]
    pub fn strg0(&mut self) -> STRG0_W<GATEC0_SPEC> {
        STRG0_W::new(self, 1)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2(&mut self) -> EDGE2_W<GATEC0_SPEC> {
        EDGE2_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select a trigger for PPG2"]
    #[inline(always)]
    #[must_use]
    pub fn strg2(&mut self) -> STRG2_W<GATEC0_SPEC> {
        STRG2_W::new(self, 5)
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
#[doc = "PPG Gate Function Control Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GATEC0_SPEC;
impl crate::RegisterSpec for GATEC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gatec0::R`](R) reader structure"]
impl crate::Readable for GATEC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gatec0::W`](W) writer structure"]
impl crate::Writable for GATEC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATEC0 to value 0"]
impl crate::Resettable for GATEC0_SPEC {
    const RESET_VALUE: u8 = 0;
}
