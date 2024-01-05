#[doc = "Register `GATEC4` reader"]
pub type R = crate::R<GATEC4_SPEC>;
#[doc = "Register `GATEC4` writer"]
pub type W = crate::W<GATEC4_SPEC>;
#[doc = "Field `EDGE4` reader - Select Start Effective Level for PPG4"]
pub type EDGE4_R = crate::BitReader;
#[doc = "Field `EDGE4` writer - Select Start Effective Level for PPG4"]
pub type EDGE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG4` reader - Select a trigger for PPG4"]
pub type STRG4_R = crate::BitReader;
#[doc = "Field `STRG4` writer - Select a trigger for PPG4"]
pub type STRG4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE6` reader - Select Start Effective Level for PPG6"]
pub type EDGE6_R = crate::BitReader;
#[doc = "Field `EDGE6` writer - Select Start Effective Level for PPG6"]
pub type EDGE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG6` reader - Select a trigger for PPG6"]
pub type STRG6_R = crate::BitReader;
#[doc = "Field `STRG6` writer - Select a trigger for PPG6"]
pub type STRG6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Start Effective Level for PPG4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG4"]
    #[inline(always)]
    pub fn strg4(&self) -> STRG4_R {
        STRG4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select a trigger for PPG6"]
    #[inline(always)]
    pub fn strg6(&self) -> STRG6_R {
        STRG6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Start Effective Level for PPG4"]
    #[inline(always)]
    #[must_use]
    pub fn edge4(&mut self) -> EDGE4_W<GATEC4_SPEC> {
        EDGE4_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG4"]
    #[inline(always)]
    #[must_use]
    pub fn strg4(&mut self) -> STRG4_W<GATEC4_SPEC> {
        STRG4_W::new(self, 1)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG6"]
    #[inline(always)]
    #[must_use]
    pub fn edge6(&mut self) -> EDGE6_W<GATEC4_SPEC> {
        EDGE6_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select a trigger for PPG6"]
    #[inline(always)]
    #[must_use]
    pub fn strg6(&mut self) -> STRG6_W<GATEC4_SPEC> {
        STRG6_W::new(self, 5)
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
#[doc = "PPG Gate Function Control Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GATEC4_SPEC;
impl crate::RegisterSpec for GATEC4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gatec4::R`](R) reader structure"]
impl crate::Readable for GATEC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gatec4::W`](W) writer structure"]
impl crate::Writable for GATEC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATEC4 to value 0"]
impl crate::Resettable for GATEC4_SPEC {
    const RESET_VALUE: u8 = 0;
}
