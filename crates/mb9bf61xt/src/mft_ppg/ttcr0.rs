#[doc = "Register `TTCR0` reader"]
pub type R = crate::R<TTCR0_SPEC>;
#[doc = "Register `TTCR0` writer"]
pub type W = crate::W<TTCR0_SPEC>;
#[doc = "Field `STR0` reader - 8-bit UP counter operation enable bit for comparison"]
pub type STR0_R = crate::BitReader;
#[doc = "Field `STR0` writer - 8-bit UP counter operation enable bit for comparison"]
pub type STR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONI0` reader - 8-bit UP counter operation state monitor bit for comparison"]
pub type MONI0_R = crate::BitReader;
#[doc = "Field `CS0` reader - 8-bit UP counter clock select bits for comparison"]
pub type CS0_R = crate::FieldReader;
#[doc = "Field `CS0` writer - 8-bit UP counter clock select bits for comparison"]
pub type CS0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRG0O` reader - PPG0 trigger stop bit"]
pub type TRG0O_R = crate::BitReader;
#[doc = "Field `TRG0O` writer - PPG0 trigger stop bit"]
pub type TRG0O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG2O` reader - PPG2 trigger stop bit"]
pub type TRG2O_R = crate::BitReader;
#[doc = "Field `TRG2O` writer - PPG2 trigger stop bit"]
pub type TRG2O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG4O` reader - PPG4 trigger stop bit"]
pub type TRG4O_R = crate::BitReader;
#[doc = "Field `TRG4O` writer - PPG4 trigger stop bit"]
pub type TRG4O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRG6O` reader - PPG6 trigger stop bit"]
pub type TRG6O_R = crate::BitReader;
#[doc = "Field `TRG6O` writer - PPG6 trigger stop bit"]
pub type TRG6O_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8-bit UP counter operation enable bit for comparison"]
    #[inline(always)]
    pub fn str0(&self) -> STR0_R {
        STR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 8-bit UP counter operation state monitor bit for comparison"]
    #[inline(always)]
    pub fn moni0(&self) -> MONI0_R {
        MONI0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 8-bit UP counter clock select bits for comparison"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - PPG0 trigger stop bit"]
    #[inline(always)]
    pub fn trg0o(&self) -> TRG0O_R {
        TRG0O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PPG2 trigger stop bit"]
    #[inline(always)]
    pub fn trg2o(&self) -> TRG2O_R {
        TRG2O_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPG4 trigger stop bit"]
    #[inline(always)]
    pub fn trg4o(&self) -> TRG4O_R {
        TRG4O_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PPG6 trigger stop bit"]
    #[inline(always)]
    pub fn trg6o(&self) -> TRG6O_R {
        TRG6O_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8-bit UP counter operation enable bit for comparison"]
    #[inline(always)]
    #[must_use]
    pub fn str0(&mut self) -> STR0_W<TTCR0_SPEC> {
        STR0_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 8-bit UP counter clock select bits for comparison"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<TTCR0_SPEC> {
        CS0_W::new(self, 10)
    }
    #[doc = "Bit 12 - PPG0 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg0o(&mut self) -> TRG0O_W<TTCR0_SPEC> {
        TRG0O_W::new(self, 12)
    }
    #[doc = "Bit 13 - PPG2 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg2o(&mut self) -> TRG2O_W<TTCR0_SPEC> {
        TRG2O_W::new(self, 13)
    }
    #[doc = "Bit 14 - PPG4 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg4o(&mut self) -> TRG4O_W<TTCR0_SPEC> {
        TRG4O_W::new(self, 14)
    }
    #[doc = "Bit 15 - PPG6 trigger stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn trg6o(&mut self) -> TRG6O_W<TTCR0_SPEC> {
        TRG6O_W::new(self, 15)
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
#[doc = "PPG Start Trigger Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTCR0_SPEC;
impl crate::RegisterSpec for TTCR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ttcr0::R`](R) reader structure"]
impl crate::Readable for TTCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ttcr0::W`](W) writer structure"]
impl crate::Writable for TTCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TTCR0 to value 0xf000"]
impl crate::Resettable for TTCR0_SPEC {
    const RESET_VALUE: u16 = 0xf000;
}
