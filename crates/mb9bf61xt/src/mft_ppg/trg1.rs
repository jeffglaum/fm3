#[doc = "Register `TRG1` reader"]
pub type R = crate::R<TRG1_SPEC>;
#[doc = "Register `TRG1` writer"]
pub type W = crate::W<TRG1_SPEC>;
#[doc = "Field `PEN16` reader - PPG16 Start Trigger bit"]
pub type PEN16_R = crate::BitReader;
#[doc = "Field `PEN16` writer - PPG16 Start Trigger bit"]
pub type PEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN17` reader - PPG17 Start Trigger bit"]
pub type PEN17_R = crate::BitReader;
#[doc = "Field `PEN17` writer - PPG17 Start Trigger bit"]
pub type PEN17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN18` reader - PPG18 Start Trigger bit"]
pub type PEN18_R = crate::BitReader;
#[doc = "Field `PEN18` writer - PPG18 Start Trigger bit"]
pub type PEN18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN19` reader - PPG19 Start Trigger bit"]
pub type PEN19_R = crate::BitReader;
#[doc = "Field `PEN19` writer - PPG19 Start Trigger bit"]
pub type PEN19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN20` reader - PPG20 Start Trigger bit"]
pub type PEN20_R = crate::BitReader;
#[doc = "Field `PEN20` writer - PPG20 Start Trigger bit"]
pub type PEN20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN21` reader - PPG21 Start Trigger bit"]
pub type PEN21_R = crate::BitReader;
#[doc = "Field `PEN21` writer - PPG21 Start Trigger bit"]
pub type PEN21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN22` reader - PPG22 Start Trigger bit"]
pub type PEN22_R = crate::BitReader;
#[doc = "Field `PEN22` writer - PPG22 Start Trigger bit"]
pub type PEN22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN23` reader - PPG23 Start Trigger bit"]
pub type PEN23_R = crate::BitReader;
#[doc = "Field `PEN23` writer - PPG23 Start Trigger bit"]
pub type PEN23_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPG16 Start Trigger bit"]
    #[inline(always)]
    pub fn pen16(&self) -> PEN16_R {
        PEN16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PPG17 Start Trigger bit"]
    #[inline(always)]
    pub fn pen17(&self) -> PEN17_R {
        PEN17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PPG18 Start Trigger bit"]
    #[inline(always)]
    pub fn pen18(&self) -> PEN18_R {
        PEN18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PPG19 Start Trigger bit"]
    #[inline(always)]
    pub fn pen19(&self) -> PEN19_R {
        PEN19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PPG20 Start Trigger bit"]
    #[inline(always)]
    pub fn pen20(&self) -> PEN20_R {
        PEN20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PPG21 Start Trigger bit"]
    #[inline(always)]
    pub fn pen21(&self) -> PEN21_R {
        PEN21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PPG22 Start Trigger bit"]
    #[inline(always)]
    pub fn pen22(&self) -> PEN22_R {
        PEN22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PPG23 Start Trigger bit"]
    #[inline(always)]
    pub fn pen23(&self) -> PEN23_R {
        PEN23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPG16 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen16(&mut self) -> PEN16_W<TRG1_SPEC> {
        PEN16_W::new(self, 0)
    }
    #[doc = "Bit 1 - PPG17 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen17(&mut self) -> PEN17_W<TRG1_SPEC> {
        PEN17_W::new(self, 1)
    }
    #[doc = "Bit 2 - PPG18 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen18(&mut self) -> PEN18_W<TRG1_SPEC> {
        PEN18_W::new(self, 2)
    }
    #[doc = "Bit 3 - PPG19 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen19(&mut self) -> PEN19_W<TRG1_SPEC> {
        PEN19_W::new(self, 3)
    }
    #[doc = "Bit 4 - PPG20 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen20(&mut self) -> PEN20_W<TRG1_SPEC> {
        PEN20_W::new(self, 4)
    }
    #[doc = "Bit 5 - PPG21 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen21(&mut self) -> PEN21_W<TRG1_SPEC> {
        PEN21_W::new(self, 5)
    }
    #[doc = "Bit 6 - PPG22 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen22(&mut self) -> PEN22_W<TRG1_SPEC> {
        PEN22_W::new(self, 6)
    }
    #[doc = "Bit 7 - PPG23 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen23(&mut self) -> PEN23_W<TRG1_SPEC> {
        PEN23_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPG Start Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRG1_SPEC;
impl crate::RegisterSpec for TRG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`trg1::R`](R) reader structure"]
impl crate::Readable for TRG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trg1::W`](W) writer structure"]
impl crate::Writable for TRG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TRG1 to value 0"]
impl crate::Resettable for TRG1_SPEC {
    const RESET_VALUE: u16 = 0;
}
