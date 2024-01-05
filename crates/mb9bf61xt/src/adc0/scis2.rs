#[doc = "Register `SCIS2` reader"]
pub type R = crate::R<SCIS2_SPEC>;
#[doc = "Register `SCIS2` writer"]
pub type W = crate::W<SCIS2_SPEC>;
#[doc = "Field `AN16` reader - Bit0 of SCIS2"]
pub type AN16_R = crate::BitReader;
#[doc = "Field `AN16` writer - Bit0 of SCIS2"]
pub type AN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN17` reader - Bit1 of SCIS2"]
pub type AN17_R = crate::BitReader;
#[doc = "Field `AN17` writer - Bit1 of SCIS2"]
pub type AN17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN18` reader - Bit2 of SCIS2"]
pub type AN18_R = crate::BitReader;
#[doc = "Field `AN18` writer - Bit2 of SCIS2"]
pub type AN18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN19` reader - Bit3 of SCIS2"]
pub type AN19_R = crate::BitReader;
#[doc = "Field `AN19` writer - Bit3 of SCIS2"]
pub type AN19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN20` reader - Bit4 of SCIS2"]
pub type AN20_R = crate::BitReader;
#[doc = "Field `AN20` writer - Bit4 of SCIS2"]
pub type AN20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN21` reader - Bit5 of SCIS2"]
pub type AN21_R = crate::BitReader;
#[doc = "Field `AN21` writer - Bit5 of SCIS2"]
pub type AN21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN22` reader - Bit6 of SCIS2"]
pub type AN22_R = crate::BitReader;
#[doc = "Field `AN22` writer - Bit6 of SCIS2"]
pub type AN22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN23` reader - Bit7 of SCIS2"]
pub type AN23_R = crate::BitReader;
#[doc = "Field `AN23` writer - Bit7 of SCIS2"]
pub type AN23_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of SCIS2"]
    #[inline(always)]
    pub fn an16(&self) -> AN16_R {
        AN16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS2"]
    #[inline(always)]
    pub fn an17(&self) -> AN17_R {
        AN17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of SCIS2"]
    #[inline(always)]
    pub fn an18(&self) -> AN18_R {
        AN18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of SCIS2"]
    #[inline(always)]
    pub fn an19(&self) -> AN19_R {
        AN19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of SCIS2"]
    #[inline(always)]
    pub fn an20(&self) -> AN20_R {
        AN20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of SCIS2"]
    #[inline(always)]
    pub fn an21(&self) -> AN21_R {
        AN21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of SCIS2"]
    #[inline(always)]
    pub fn an22(&self) -> AN22_R {
        AN22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of SCIS2"]
    #[inline(always)]
    pub fn an23(&self) -> AN23_R {
        AN23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an16(&mut self) -> AN16_W<SCIS2_SPEC> {
        AN16_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an17(&mut self) -> AN17_W<SCIS2_SPEC> {
        AN17_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an18(&mut self) -> AN18_W<SCIS2_SPEC> {
        AN18_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an19(&mut self) -> AN19_W<SCIS2_SPEC> {
        AN19_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an20(&mut self) -> AN20_W<SCIS2_SPEC> {
        AN20_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an21(&mut self) -> AN21_W<SCIS2_SPEC> {
        AN21_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an22(&mut self) -> AN22_W<SCIS2_SPEC> {
        AN22_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of SCIS2"]
    #[inline(always)]
    #[must_use]
    pub fn an23(&mut self) -> AN23_W<SCIS2_SPEC> {
        AN23_W::new(self, 7)
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
#[doc = "Scan Conversion Input Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIS2_SPEC;
impl crate::RegisterSpec for SCIS2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scis2::R`](R) reader structure"]
impl crate::Readable for SCIS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scis2::W`](W) writer structure"]
impl crate::Writable for SCIS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCIS2 to value 0"]
impl crate::Resettable for SCIS2_SPEC {
    const RESET_VALUE: u8 = 0;
}
