#[doc = "Register `GATEC8` reader"]
pub type R = crate::R<GATEC8_SPEC>;
#[doc = "Register `GATEC8` writer"]
pub type W = crate::W<GATEC8_SPEC>;
#[doc = "Field `EDGE8` reader - Select Start Effective Level for PPG8"]
pub type EDGE8_R = crate::BitReader;
#[doc = "Field `EDGE8` writer - Select Start Effective Level for PPG8"]
pub type EDGE8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG8` reader - Select a trigger for PPG8"]
pub type STRG8_R = crate::BitReader;
#[doc = "Field `STRG8` writer - Select a trigger for PPG8"]
pub type STRG8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE10` reader - Select Start Effective Level for PPG10"]
pub type EDGE10_R = crate::BitReader;
#[doc = "Field `EDGE10` writer - Select Start Effective Level for PPG10"]
pub type EDGE10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG10` reader - Select a trigger for PPG10"]
pub type STRG10_R = crate::BitReader;
#[doc = "Field `STRG10` writer - Select a trigger for PPG10"]
pub type STRG10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Start Effective Level for PPG8"]
    #[inline(always)]
    pub fn edge8(&self) -> EDGE8_R {
        EDGE8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG8"]
    #[inline(always)]
    pub fn strg8(&self) -> STRG8_R {
        STRG8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG10"]
    #[inline(always)]
    pub fn edge10(&self) -> EDGE10_R {
        EDGE10_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select a trigger for PPG10"]
    #[inline(always)]
    pub fn strg10(&self) -> STRG10_R {
        STRG10_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Start Effective Level for PPG8"]
    #[inline(always)]
    #[must_use]
    pub fn edge8(&mut self) -> EDGE8_W<GATEC8_SPEC> {
        EDGE8_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG8"]
    #[inline(always)]
    #[must_use]
    pub fn strg8(&mut self) -> STRG8_W<GATEC8_SPEC> {
        STRG8_W::new(self, 1)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG10"]
    #[inline(always)]
    #[must_use]
    pub fn edge10(&mut self) -> EDGE10_W<GATEC8_SPEC> {
        EDGE10_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select a trigger for PPG10"]
    #[inline(always)]
    #[must_use]
    pub fn strg10(&mut self) -> STRG10_W<GATEC8_SPEC> {
        STRG10_W::new(self, 5)
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
#[doc = "PPG Gate Function Control Registers 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GATEC8_SPEC;
impl crate::RegisterSpec for GATEC8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gatec8::R`](R) reader structure"]
impl crate::Readable for GATEC8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gatec8::W`](W) writer structure"]
impl crate::Writable for GATEC8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATEC8 to value 0"]
impl crate::Resettable for GATEC8_SPEC {
    const RESET_VALUE: u8 = 0;
}
