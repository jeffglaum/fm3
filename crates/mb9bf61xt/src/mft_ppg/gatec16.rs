#[doc = "Register `GATEC16` reader"]
pub type R = crate::R<GATEC16_SPEC>;
#[doc = "Register `GATEC16` writer"]
pub type W = crate::W<GATEC16_SPEC>;
#[doc = "Field `EDGE16` reader - Select Start Effective Level for PPG16"]
pub type EDGE16_R = crate::BitReader;
#[doc = "Field `EDGE16` writer - Select Start Effective Level for PPG16"]
pub type EDGE16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG16` reader - Select a trigger for PPG16"]
pub type STRG16_R = crate::BitReader;
#[doc = "Field `STRG16` writer - Select a trigger for PPG16"]
pub type STRG16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE18` reader - Select Start Effective Level for PPG18"]
pub type EDGE18_R = crate::BitReader;
#[doc = "Field `EDGE18` writer - Select Start Effective Level for PPG18"]
pub type EDGE18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRG18` reader - Select a trigger for PPG18"]
pub type STRG18_R = crate::BitReader;
#[doc = "Field `STRG18` writer - Select a trigger for PPG18"]
pub type STRG18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Start Effective Level for PPG16"]
    #[inline(always)]
    pub fn edge16(&self) -> EDGE16_R {
        EDGE16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG16"]
    #[inline(always)]
    pub fn strg16(&self) -> STRG16_R {
        STRG16_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG18"]
    #[inline(always)]
    pub fn edge18(&self) -> EDGE18_R {
        EDGE18_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select a trigger for PPG18"]
    #[inline(always)]
    pub fn strg18(&self) -> STRG18_R {
        STRG18_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Start Effective Level for PPG16"]
    #[inline(always)]
    #[must_use]
    pub fn edge16(&mut self) -> EDGE16_W<GATEC16_SPEC> {
        EDGE16_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select a trigger for PPG16"]
    #[inline(always)]
    #[must_use]
    pub fn strg16(&mut self) -> STRG16_W<GATEC16_SPEC> {
        STRG16_W::new(self, 1)
    }
    #[doc = "Bit 4 - Select Start Effective Level for PPG18"]
    #[inline(always)]
    #[must_use]
    pub fn edge18(&mut self) -> EDGE18_W<GATEC16_SPEC> {
        EDGE18_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select a trigger for PPG18"]
    #[inline(always)]
    #[must_use]
    pub fn strg18(&mut self) -> STRG18_W<GATEC16_SPEC> {
        STRG18_W::new(self, 5)
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
#[doc = "PPG Gate Function Control Registers 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GATEC16_SPEC;
impl crate::RegisterSpec for GATEC16_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gatec16::R`](R) reader structure"]
impl crate::Readable for GATEC16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gatec16::W`](W) writer structure"]
impl crate::Writable for GATEC16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GATEC16 to value 0"]
impl crate::Resettable for GATEC16_SPEC {
    const RESET_VALUE: u8 = 0;
}
