#[doc = "Register `REVC` reader"]
pub type R = crate::R<REVC_SPEC>;
#[doc = "Register `REVC` writer"]
pub type W = crate::W<REVC_SPEC>;
#[doc = "Field `REV00` reader - PPG0 Output Reverse Enable bit"]
pub type REV00_R = crate::BitReader;
#[doc = "Field `REV00` writer - PPG0 Output Reverse Enable bit"]
pub type REV00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV01` reader - PPG1 Output Reverse Enable bit"]
pub type REV01_R = crate::BitReader;
#[doc = "Field `REV01` writer - PPG1 Output Reverse Enable bit"]
pub type REV01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV02` reader - PPG2 Output Reverse Enable bit"]
pub type REV02_R = crate::BitReader;
#[doc = "Field `REV02` writer - PPG2 Output Reverse Enable bit"]
pub type REV02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV03` reader - PPG3 Output Reverse Enable bit"]
pub type REV03_R = crate::BitReader;
#[doc = "Field `REV03` writer - PPG3 Output Reverse Enable bit"]
pub type REV03_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV04` reader - PPG4 Output Reverse Enable bit"]
pub type REV04_R = crate::BitReader;
#[doc = "Field `REV04` writer - PPG4 Output Reverse Enable bit"]
pub type REV04_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV05` reader - PPG5 Output Reverse Enable bit"]
pub type REV05_R = crate::BitReader;
#[doc = "Field `REV05` writer - PPG5 Output Reverse Enable bit"]
pub type REV05_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV06` reader - PPG6 Output Reverse Enable bit"]
pub type REV06_R = crate::BitReader;
#[doc = "Field `REV06` writer - PPG6 Output Reverse Enable bit"]
pub type REV06_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV07` reader - PPG7 Output Reverse Enable bit"]
pub type REV07_R = crate::BitReader;
#[doc = "Field `REV07` writer - PPG7 Output Reverse Enable bit"]
pub type REV07_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV08` reader - PPG8 Output Reverse Enable bit"]
pub type REV08_R = crate::BitReader;
#[doc = "Field `REV08` writer - PPG8 Output Reverse Enable bit"]
pub type REV08_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV09` reader - PPG9 Output Reverse Enable bit"]
pub type REV09_R = crate::BitReader;
#[doc = "Field `REV09` writer - PPG9 Output Reverse Enable bit"]
pub type REV09_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV10` reader - PPG10 Output Reverse Enable bit"]
pub type REV10_R = crate::BitReader;
#[doc = "Field `REV10` writer - PPG10 Output Reverse Enable bit"]
pub type REV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV11` reader - PPG11 Output Reverse Enable bit"]
pub type REV11_R = crate::BitReader;
#[doc = "Field `REV11` writer - PPG11 Output Reverse Enable bit"]
pub type REV11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV12` reader - PPG12 Output Reverse Enable bit"]
pub type REV12_R = crate::BitReader;
#[doc = "Field `REV12` writer - PPG12 Output Reverse Enable bit"]
pub type REV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV13` reader - PPG13 Output Reverse Enable bit"]
pub type REV13_R = crate::BitReader;
#[doc = "Field `REV13` writer - PPG13 Output Reverse Enable bit"]
pub type REV13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV14` reader - PPG14 Output Reverse Enable bit"]
pub type REV14_R = crate::BitReader;
#[doc = "Field `REV14` writer - PPG14 Output Reverse Enable bit"]
pub type REV14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV15` reader - PPG15 Output Reverse Enable bit"]
pub type REV15_R = crate::BitReader;
#[doc = "Field `REV15` writer - PPG15 Output Reverse Enable bit"]
pub type REV15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPG0 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev00(&self) -> REV00_R {
        REV00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PPG1 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev01(&self) -> REV01_R {
        REV01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PPG2 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev02(&self) -> REV02_R {
        REV02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PPG3 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev03(&self) -> REV03_R {
        REV03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PPG4 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev04(&self) -> REV04_R {
        REV04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PPG5 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev05(&self) -> REV05_R {
        REV05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PPG6 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev06(&self) -> REV06_R {
        REV06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PPG7 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev07(&self) -> REV07_R {
        REV07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PPG8 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev08(&self) -> REV08_R {
        REV08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PPG9 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev09(&self) -> REV09_R {
        REV09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PPG10 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev10(&self) -> REV10_R {
        REV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PPG11 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev11(&self) -> REV11_R {
        REV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PPG12 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev12(&self) -> REV12_R {
        REV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PPG13 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev13(&self) -> REV13_R {
        REV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPG14 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev14(&self) -> REV14_R {
        REV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PPG15 Output Reverse Enable bit"]
    #[inline(always)]
    pub fn rev15(&self) -> REV15_R {
        REV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPG0 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev00(&mut self) -> REV00_W<REVC_SPEC> {
        REV00_W::new(self, 0)
    }
    #[doc = "Bit 1 - PPG1 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev01(&mut self) -> REV01_W<REVC_SPEC> {
        REV01_W::new(self, 1)
    }
    #[doc = "Bit 2 - PPG2 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev02(&mut self) -> REV02_W<REVC_SPEC> {
        REV02_W::new(self, 2)
    }
    #[doc = "Bit 3 - PPG3 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev03(&mut self) -> REV03_W<REVC_SPEC> {
        REV03_W::new(self, 3)
    }
    #[doc = "Bit 4 - PPG4 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev04(&mut self) -> REV04_W<REVC_SPEC> {
        REV04_W::new(self, 4)
    }
    #[doc = "Bit 5 - PPG5 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev05(&mut self) -> REV05_W<REVC_SPEC> {
        REV05_W::new(self, 5)
    }
    #[doc = "Bit 6 - PPG6 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev06(&mut self) -> REV06_W<REVC_SPEC> {
        REV06_W::new(self, 6)
    }
    #[doc = "Bit 7 - PPG7 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev07(&mut self) -> REV07_W<REVC_SPEC> {
        REV07_W::new(self, 7)
    }
    #[doc = "Bit 8 - PPG8 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev08(&mut self) -> REV08_W<REVC_SPEC> {
        REV08_W::new(self, 8)
    }
    #[doc = "Bit 9 - PPG9 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev09(&mut self) -> REV09_W<REVC_SPEC> {
        REV09_W::new(self, 9)
    }
    #[doc = "Bit 10 - PPG10 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev10(&mut self) -> REV10_W<REVC_SPEC> {
        REV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - PPG11 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev11(&mut self) -> REV11_W<REVC_SPEC> {
        REV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - PPG12 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev12(&mut self) -> REV12_W<REVC_SPEC> {
        REV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - PPG13 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev13(&mut self) -> REV13_W<REVC_SPEC> {
        REV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - PPG14 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev14(&mut self) -> REV14_W<REVC_SPEC> {
        REV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - PPG15 Output Reverse Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rev15(&mut self) -> REV15_W<REVC_SPEC> {
        REV15_W::new(self, 15)
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
#[doc = "Output Reverse Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REVC_SPEC;
impl crate::RegisterSpec for REVC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`revc::R`](R) reader structure"]
impl crate::Readable for REVC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`revc::W`](W) writer structure"]
impl crate::Writable for REVC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REVC to value 0"]
impl crate::Resettable for REVC_SPEC {
    const RESET_VALUE: u16 = 0;
}
