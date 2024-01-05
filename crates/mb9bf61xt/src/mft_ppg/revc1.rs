#[doc = "Register `REVC1` reader"]
pub type R = crate::R<REVC1_SPEC>;
#[doc = "Register `REVC1` writer"]
pub type W = crate::W<REVC1_SPEC>;
#[doc = "Field `REV16` reader - PPG16 Output Reverse Enable bit"]
pub type REV16_R = crate::BitReader;
#[doc = "Field `REV16` writer - PPG16 Output Reverse Enable bit"]
pub type REV16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV17` reader - PPG17 Output Reverse Enable bit"]
pub type REV17_R = crate::BitReader;
#[doc = "Field `REV17` writer - PPG17 Output Reverse Enable bit"]
pub type REV17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV18` reader - PPG18 Output Reverse Enable bit"]
pub type REV18_R = crate::BitReader;
#[doc = "Field `REV18` writer - PPG18 Output Reverse Enable bit"]
pub type REV18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV19` reader - PPG19 Output Reverse Enable bit"]
pub type REV19_R = crate::BitReader;
#[doc = "Field `REV19` writer - PPG19 Output Reverse Enable bit"]
pub type REV19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV20` reader - PPG20 Output Reverse Enable bit"]
pub type REV20_R = crate::BitReader;
#[doc = "Field `REV20` writer - PPG20 Output Reverse Enable bit"]
pub type REV20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV21` reader - PPG21 Output Reverse Enable bit"]
pub type REV21_R = crate::BitReader;
#[doc = "Field `REV21` writer - PPG21 Output Reverse Enable bit"]
pub type REV21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV22` reader - PPG22 Output Reverse Enable bit"]
pub type REV22_R = crate::BitReader;
#[doc = "Field `REV22` writer - PPG22 Output Reverse Enable bit"]
pub type REV22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV23` reader - PPG23 Output Reverse Enable bit"]
pub type REV23_R = crate::BitReader;
#[doc = "Field `REV23` writer - PPG23 Output Reverse Enable bit"]
pub type REV23_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPG16 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev16(&self) -> REV16_R {
        REV16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PPG17 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev17(&self) -> REV17_R {
        REV17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PPG18 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev18(&self) -> REV18_R {
        REV18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PPG19 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev19(&self) -> REV19_R {
        REV19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PPG20 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev20(&self) -> REV20_R {
        REV20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PPG21 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev21(&self) -> REV21_R {
        REV21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PPG22 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev22(&self) -> REV22_R {
        REV22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PPG23 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev23(&self) -> REV23_R {
        REV23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPG16 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev16(&mut self) -> REV16_W<REVC1_SPEC> {
        REV16_W::new(self, 0)
    }
    #[doc = "Bit 1 - PPG17 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev17(&mut self) -> REV17_W<REVC1_SPEC> {
        REV17_W::new(self, 1)
    }
    #[doc = "Bit 2 - PPG18 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev18(&mut self) -> REV18_W<REVC1_SPEC> {
        REV18_W::new(self, 2)
    }
    #[doc = "Bit 3 - PPG19 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev19(&mut self) -> REV19_W<REVC1_SPEC> {
        REV19_W::new(self, 3)
    }
    #[doc = "Bit 4 - PPG20 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev20(&mut self) -> REV20_W<REVC1_SPEC> {
        REV20_W::new(self, 4)
    }
    #[doc = "Bit 5 - PPG21 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev21(&mut self) -> REV21_W<REVC1_SPEC> {
        REV21_W::new(self, 5)
    }
    #[doc = "Bit 6 - PPG22 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev22(&mut self) -> REV22_W<REVC1_SPEC> {
        REV22_W::new(self, 6)
    }
    #[doc = "Bit 7 - PPG23 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev23(&mut self) -> REV23_W<REVC1_SPEC> {
        REV23_W::new(self, 7)
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
#[doc = "Output Reverse Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REVC1_SPEC;
impl crate::RegisterSpec for REVC1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`revc1::R`](R) reader structure"]
impl crate::Readable for REVC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`revc1::W`](W) writer structure"]
impl crate::Writable for REVC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REVC1 to value 0"]
impl crate::Resettable for REVC1_SPEC {
    const RESET_VALUE: u16 = 0;
}
