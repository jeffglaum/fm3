#[doc = "Register `TRG` reader"]
pub type R = crate::R<TRG_SPEC>;
#[doc = "Register `TRG` writer"]
pub type W = crate::W<TRG_SPEC>;
#[doc = "Field `PEN00` reader - PPG0 Start Trigger bit"]
pub type PEN00_R = crate::BitReader;
#[doc = "Field `PEN00` writer - PPG0 Start Trigger bit"]
pub type PEN00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN01` reader - PPG1 Start Trigger bit"]
pub type PEN01_R = crate::BitReader;
#[doc = "Field `PEN01` writer - PPG1 Start Trigger bit"]
pub type PEN01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN02` reader - PPG2 Start Trigger bit"]
pub type PEN02_R = crate::BitReader;
#[doc = "Field `PEN02` writer - PPG2 Start Trigger bit"]
pub type PEN02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN03` reader - PPG3 Start Trigger bit"]
pub type PEN03_R = crate::BitReader;
#[doc = "Field `PEN03` writer - PPG3 Start Trigger bit"]
pub type PEN03_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN04` reader - PPG4 Start Trigger bit"]
pub type PEN04_R = crate::BitReader;
#[doc = "Field `PEN04` writer - PPG4 Start Trigger bit"]
pub type PEN04_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN05` reader - PPG5 Start Trigger bit"]
pub type PEN05_R = crate::BitReader;
#[doc = "Field `PEN05` writer - PPG5 Start Trigger bit"]
pub type PEN05_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN06` reader - PPG6 Start Trigger bit"]
pub type PEN06_R = crate::BitReader;
#[doc = "Field `PEN06` writer - PPG6 Start Trigger bit"]
pub type PEN06_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN07` reader - PPG7 Start Trigger bit"]
pub type PEN07_R = crate::BitReader;
#[doc = "Field `PEN07` writer - PPG7 Start Trigger bit"]
pub type PEN07_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN08` reader - PPG8 Start Trigger bit"]
pub type PEN08_R = crate::BitReader;
#[doc = "Field `PEN08` writer - PPG8 Start Trigger bit"]
pub type PEN08_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN09` reader - PPG9 Start Trigger bit"]
pub type PEN09_R = crate::BitReader;
#[doc = "Field `PEN09` writer - PPG9 Start Trigger bit"]
pub type PEN09_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN10` reader - PPG10 Start Trigger bit"]
pub type PEN10_R = crate::BitReader;
#[doc = "Field `PEN10` writer - PPG10 Start Trigger bit"]
pub type PEN10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN11` reader - PPG11 Start Trigger bit"]
pub type PEN11_R = crate::BitReader;
#[doc = "Field `PEN11` writer - PPG11 Start Trigger bit"]
pub type PEN11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN12` reader - PPG12 Start Trigger bit"]
pub type PEN12_R = crate::BitReader;
#[doc = "Field `PEN12` writer - PPG12 Start Trigger bit"]
pub type PEN12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN13` reader - PPG13 Start Trigger bit"]
pub type PEN13_R = crate::BitReader;
#[doc = "Field `PEN13` writer - PPG13 Start Trigger bit"]
pub type PEN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN14` reader - PPG14 Start Trigger bit"]
pub type PEN14_R = crate::BitReader;
#[doc = "Field `PEN14` writer - PPG14 Start Trigger bit"]
pub type PEN14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN15` reader - PPG15 Start Trigger bit"]
pub type PEN15_R = crate::BitReader;
#[doc = "Field `PEN15` writer - PPG15 Start Trigger bit"]
pub type PEN15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPG0 Start Trigger bit"]
    #[inline(always)]
    pub fn pen00(&self) -> PEN00_R {
        PEN00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PPG1 Start Trigger bit"]
    #[inline(always)]
    pub fn pen01(&self) -> PEN01_R {
        PEN01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PPG2 Start Trigger bit"]
    #[inline(always)]
    pub fn pen02(&self) -> PEN02_R {
        PEN02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PPG3 Start Trigger bit"]
    #[inline(always)]
    pub fn pen03(&self) -> PEN03_R {
        PEN03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PPG4 Start Trigger bit"]
    #[inline(always)]
    pub fn pen04(&self) -> PEN04_R {
        PEN04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PPG5 Start Trigger bit"]
    #[inline(always)]
    pub fn pen05(&self) -> PEN05_R {
        PEN05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PPG6 Start Trigger bit"]
    #[inline(always)]
    pub fn pen06(&self) -> PEN06_R {
        PEN06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PPG7 Start Trigger bit"]
    #[inline(always)]
    pub fn pen07(&self) -> PEN07_R {
        PEN07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PPG8 Start Trigger bit"]
    #[inline(always)]
    pub fn pen08(&self) -> PEN08_R {
        PEN08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PPG9 Start Trigger bit"]
    #[inline(always)]
    pub fn pen09(&self) -> PEN09_R {
        PEN09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PPG10 Start Trigger bit"]
    #[inline(always)]
    pub fn pen10(&self) -> PEN10_R {
        PEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PPG11 Start Trigger bit"]
    #[inline(always)]
    pub fn pen11(&self) -> PEN11_R {
        PEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PPG12 Start Trigger bit"]
    #[inline(always)]
    pub fn pen12(&self) -> PEN12_R {
        PEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PPG13 Start Trigger bit"]
    #[inline(always)]
    pub fn pen13(&self) -> PEN13_R {
        PEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPG14 Start Trigger bit"]
    #[inline(always)]
    pub fn pen14(&self) -> PEN14_R {
        PEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PPG15 Start Trigger bit"]
    #[inline(always)]
    pub fn pen15(&self) -> PEN15_R {
        PEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPG0 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen00(&mut self) -> PEN00_W<TRG_SPEC> {
        PEN00_W::new(self, 0)
    }
    #[doc = "Bit 1 - PPG1 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen01(&mut self) -> PEN01_W<TRG_SPEC> {
        PEN01_W::new(self, 1)
    }
    #[doc = "Bit 2 - PPG2 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen02(&mut self) -> PEN02_W<TRG_SPEC> {
        PEN02_W::new(self, 2)
    }
    #[doc = "Bit 3 - PPG3 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen03(&mut self) -> PEN03_W<TRG_SPEC> {
        PEN03_W::new(self, 3)
    }
    #[doc = "Bit 4 - PPG4 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen04(&mut self) -> PEN04_W<TRG_SPEC> {
        PEN04_W::new(self, 4)
    }
    #[doc = "Bit 5 - PPG5 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen05(&mut self) -> PEN05_W<TRG_SPEC> {
        PEN05_W::new(self, 5)
    }
    #[doc = "Bit 6 - PPG6 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen06(&mut self) -> PEN06_W<TRG_SPEC> {
        PEN06_W::new(self, 6)
    }
    #[doc = "Bit 7 - PPG7 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen07(&mut self) -> PEN07_W<TRG_SPEC> {
        PEN07_W::new(self, 7)
    }
    #[doc = "Bit 8 - PPG8 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen08(&mut self) -> PEN08_W<TRG_SPEC> {
        PEN08_W::new(self, 8)
    }
    #[doc = "Bit 9 - PPG9 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen09(&mut self) -> PEN09_W<TRG_SPEC> {
        PEN09_W::new(self, 9)
    }
    #[doc = "Bit 10 - PPG10 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen10(&mut self) -> PEN10_W<TRG_SPEC> {
        PEN10_W::new(self, 10)
    }
    #[doc = "Bit 11 - PPG11 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen11(&mut self) -> PEN11_W<TRG_SPEC> {
        PEN11_W::new(self, 11)
    }
    #[doc = "Bit 12 - PPG12 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen12(&mut self) -> PEN12_W<TRG_SPEC> {
        PEN12_W::new(self, 12)
    }
    #[doc = "Bit 13 - PPG13 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen13(&mut self) -> PEN13_W<TRG_SPEC> {
        PEN13_W::new(self, 13)
    }
    #[doc = "Bit 14 - PPG14 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen14(&mut self) -> PEN14_W<TRG_SPEC> {
        PEN14_W::new(self, 14)
    }
    #[doc = "Bit 15 - PPG15 Start Trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn pen15(&mut self) -> PEN15_W<TRG_SPEC> {
        PEN15_W::new(self, 15)
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
#[doc = "PPG Start Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRG_SPEC;
impl crate::RegisterSpec for TRG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`trg::R`](R) reader structure"]
impl crate::Readable for TRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trg::W`](W) writer structure"]
impl crate::Writable for TRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TRG to value 0"]
impl crate::Resettable for TRG_SPEC {
    const RESET_VALUE: u16 = 0;
}
