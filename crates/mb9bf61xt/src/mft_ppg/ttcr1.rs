#[doc = "Register `TTCR1` reader"]
pub type R = crate::R<TTCR1_SPEC>;
#[doc = "Register `TTCR1` writer"]
pub type W = crate::W<TTCR1_SPEC>;
#[doc = "Field `STR1` reader - 8-bit UP counter operation enable bit for comparison"]
pub type STR1_R = crate::BitReader;
#[doc = "Field `STR1` writer - 8-bit UP counter operation enable bit for comparison"]
pub type STR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONI1` reader - 8-bit UP counter operation state monitor bit for comparison"]
pub type MONI1_R = crate::BitReader;
#[doc = "Field `CS1` reader - 8-bit UP counter clock select bits for comparison"]
pub type CS1_R = crate::FieldReader;
#[doc = "Field `CS1` writer - 8-bit UP counter clock select bits for comparison"]
pub type CS1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRG1O` reader - PPG1 trigger stop bit"]
pub type TRG1O_R = crate::BitReader;
#[doc = "Field `TRG1O` writer - PPG1 trigger stop bit"]
pub type TRG1O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG3O` reader - PPG3 trigger stop bit"]
pub type TRG3O_R = crate::BitReader;
#[doc = "Field `TRG3O` writer - PPG3 trigger stop bit"]
pub type TRG3O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG5O` reader - PPG5 trigger stop bit"]
pub type TRG5O_R = crate::BitReader;
#[doc = "Field `TRG5O` writer - PPG5 trigger stop bit"]
pub type TRG5O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG7O` reader - PPG7 trigger stop bit"]
pub type TRG7O_R = crate::BitReader;
#[doc = "Field `TRG7O` writer - PPG7 trigger stop bit"]
pub type TRG7O_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8-bit UP counter operation enable bit for comparison"]
    #[inline(always)]
    pub fn str1(&self) -> STR1_R {
        STR1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 8-bit UP counter operation state monitor bit for comparison"]
    #[inline(always)]
    pub fn moni1(&self) -> MONI1_R {
        MONI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 8-bit UP counter clock select bits for comparison"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - PPG1 trigger stop bit"]
    #[inline(always)]
    pub fn trg1o(&self) -> TRG1O_R {
        TRG1O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PPG3 trigger stop bit"]
    #[inline(always)]
    pub fn trg3o(&self) -> TRG3O_R {
        TRG3O_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPG5 trigger stop bit"]
    #[inline(always)]
    pub fn trg5o(&self) -> TRG5O_R {
        TRG5O_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PPG7 trigger stop bit"]
    #[inline(always)]
    pub fn trg7o(&self) -> TRG7O_R {
        TRG7O_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8-bit UP counter operation enable bit for comparison"]
    #[inline(always)]
    #[must_use]
    pub fn str1(&mut self) -> STR1_W<TTCR1_SPEC> {
        STR1_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 8-bit UP counter clock select bits for comparison"]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<TTCR1_SPEC> {
        CS1_W::new(self, 10)
    }
    #[doc = "Bit 12 - PPG1 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg1o(&mut self) -> TRG1O_W<TTCR1_SPEC> {
        TRG1O_W::new(self, 12)
    }
    #[doc = "Bit 13 - PPG3 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg3o(&mut self) -> TRG3O_W<TTCR1_SPEC> {
        TRG3O_W::new(self, 13)
    }
    #[doc = "Bit 14 - PPG5 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg5o(&mut self) -> TRG5O_W<TTCR1_SPEC> {
        TRG5O_W::new(self, 14)
    }
    #[doc = "Bit 15 - PPG7 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg7o(&mut self) -> TRG7O_W<TTCR1_SPEC> {
        TRG7O_W::new(self, 15)
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
#[doc = "PPG Start Trigger Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTCR1_SPEC;
impl crate::RegisterSpec for TTCR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ttcr1::R`](R) reader structure"]
impl crate::Readable for TTCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ttcr1::W`](W) writer structure"]
impl crate::Writable for TTCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TTCR1 to value 0xf000"]
impl crate::Resettable for TTCR1_SPEC {
    const RESET_VALUE: u16 = 0xf000;
}
