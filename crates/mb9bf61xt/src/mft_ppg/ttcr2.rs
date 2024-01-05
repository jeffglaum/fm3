#[doc = "Register `TTCR2` reader"]
pub type R = crate::R<TTCR2_SPEC>;
#[doc = "Register `TTCR2` writer"]
pub type W = crate::W<TTCR2_SPEC>;
#[doc = "Field `STR2` reader - 8-bit UP counter operation enable bit for comparison"]
pub type STR2_R = crate::BitReader;
#[doc = "Field `STR2` writer - 8-bit UP counter operation enable bit for comparison"]
pub type STR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONI2` reader - 8-bit UP counter operation state monitor bit for comparison"]
pub type MONI2_R = crate::BitReader;
#[doc = "Field `CS2` reader - 8-bit UP counter clock select bits for comparison"]
pub type CS2_R = crate::FieldReader;
#[doc = "Field `CS2` writer - 8-bit UP counter clock select bits for comparison"]
pub type CS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRG16O` reader - PPG16 trigger stop bit"]
pub type TRG16O_R = crate::BitReader;
#[doc = "Field `TRG16O` writer - PPG16 trigger stop bit"]
pub type TRG16O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG18O` reader - PPG18 trigger stop bit"]
pub type TRG18O_R = crate::BitReader;
#[doc = "Field `TRG18O` writer - PPG18 trigger stop bit"]
pub type TRG18O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG20O` reader - PPG20 trigger stop bit"]
pub type TRG20O_R = crate::BitReader;
#[doc = "Field `TRG20O` writer - PPG20 trigger stop bit"]
pub type TRG20O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG22O` reader - PPG22 trigger stop bit"]
pub type TRG22O_R = crate::BitReader;
#[doc = "Field `TRG22O` writer - PPG22 trigger stop bit"]
pub type TRG22O_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8-bit UP counter operation enable bit for comparison"]
    #[inline(always)]
    pub fn str2(&self) -> STR2_R {
        STR2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 8-bit UP counter operation state monitor bit for comparison"]
    #[inline(always)]
    pub fn moni2(&self) -> MONI2_R {
        MONI2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 8-bit UP counter clock select bits for comparison"]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - PPG16 trigger stop bit"]
    #[inline(always)]
    pub fn trg16o(&self) -> TRG16O_R {
        TRG16O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PPG18 trigger stop bit"]
    #[inline(always)]
    pub fn trg18o(&self) -> TRG18O_R {
        TRG18O_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPG20 trigger stop bit"]
    #[inline(always)]
    pub fn trg20o(&self) -> TRG20O_R {
        TRG20O_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PPG22 trigger stop bit"]
    #[inline(always)]
    pub fn trg22o(&self) -> TRG22O_R {
        TRG22O_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8-bit UP counter operation enable bit for comparison"]
    #[inline(always)]
    #[must_use]
    pub fn str2(&mut self) -> STR2_W<TTCR2_SPEC> {
        STR2_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 8-bit UP counter clock select bits for comparison"]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<TTCR2_SPEC> {
        CS2_W::new(self, 10)
    }
    #[doc = "Bit 12 - PPG16 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg16o(&mut self) -> TRG16O_W<TTCR2_SPEC> {
        TRG16O_W::new(self, 12)
    }
    #[doc = "Bit 13 - PPG18 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg18o(&mut self) -> TRG18O_W<TTCR2_SPEC> {
        TRG18O_W::new(self, 13)
    }
    #[doc = "Bit 14 - PPG20 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg20o(&mut self) -> TRG20O_W<TTCR2_SPEC> {
        TRG20O_W::new(self, 14)
    }
    #[doc = "Bit 15 - PPG22 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg22o(&mut self) -> TRG22O_W<TTCR2_SPEC> {
        TRG22O_W::new(self, 15)
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
#[doc = "PPG Start Trigger Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTCR2_SPEC;
impl crate::RegisterSpec for TTCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ttcr2::R`](R) reader structure"]
impl crate::Readable for TTCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ttcr2::W`](W) writer structure"]
impl crate::Writable for TTCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TTCR2 to value 0xf000"]
impl crate::Resettable for TTCR2_SPEC {
    const RESET_VALUE: u16 = 0xf000;
}
