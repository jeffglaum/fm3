#[doc = "Register `QICRL` reader"]
pub type R = crate::R<QICRL_SPEC>;
#[doc = "Register `QICRL` writer"]
pub type W = crate::W<QICRL_SPEC>;
#[doc = "Field `QPCMIE` reader - PC match interrupt enable bit"]
pub type QPCMIE_R = crate::BitReader;
#[doc = "Field `QPCMIE` writer - PC match interrupt enable bit"]
pub type QPCMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPCMF` reader - PC match interrupt request flag bit"]
pub type QPCMF_R = crate::BitReader;
#[doc = "Field `QPCMF` writer - PC match interrupt request flag bit"]
pub type QPCMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPRCMIE` reader - PC and RC match interrupt enable bit"]
pub type QPRCMIE_R = crate::BitReader;
#[doc = "Field `QPRCMIE` writer - PC and RC match interrupt enable bit"]
pub type QPRCMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPRCMF` reader - PC and RC match interrupt request flag bit"]
pub type QPRCMF_R = crate::BitReader;
#[doc = "Field `QPRCMF` writer - PC and RC match interrupt request flag bit"]
pub type QPRCMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUZIE` reader - \"Overflow, underflow, or zero index interrupt enable bit\""]
pub type OUZIE_R = crate::BitReader;
#[doc = "Field `OUZIE` writer - \"Overflow, underflow, or zero index interrupt enable bit\""]
pub type OUZIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UFDF` reader - Underflow interrupt request flag bit"]
pub type UFDF_R = crate::BitReader;
#[doc = "Field `UFDF` writer - Underflow interrupt request flag bit"]
pub type UFDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFDF` reader - Overflow interrupt request flag bit"]
pub type OFDF_R = crate::BitReader;
#[doc = "Field `OFDF` writer - Overflow interrupt request flag bit"]
pub type OFDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZIIF` reader - Zero index interrupt request flag bit"]
pub type ZIIF_R = crate::BitReader;
#[doc = "Field `ZIIF` writer - Zero index interrupt request flag bit"]
pub type ZIIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PC match interrupt enable bit"]
    #[inline(always)]
    pub fn qpcmie(&self) -> QPCMIE_R {
        QPCMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PC match interrupt request flag bit"]
    #[inline(always)]
    pub fn qpcmf(&self) -> QPCMF_R {
        QPCMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PC and RC match interrupt enable bit"]
    #[inline(always)]
    pub fn qprcmie(&self) -> QPRCMIE_R {
        QPRCMIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PC and RC match interrupt request flag bit"]
    #[inline(always)]
    pub fn qprcmf(&self) -> QPRCMF_R {
        QPRCMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - \"Overflow, underflow, or zero index interrupt enable bit\""]
    #[inline(always)]
    pub fn ouzie(&self) -> OUZIE_R {
        OUZIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow interrupt request flag bit"]
    #[inline(always)]
    pub fn ufdf(&self) -> UFDF_R {
        UFDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overflow interrupt request flag bit"]
    #[inline(always)]
    pub fn ofdf(&self) -> OFDF_R {
        OFDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Zero index interrupt request flag bit"]
    #[inline(always)]
    pub fn ziif(&self) -> ZIIF_R {
        ZIIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PC match interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn qpcmie(&mut self) -> QPCMIE_W<QICRL_SPEC> {
        QPCMIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - PC match interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn qpcmf(&mut self) -> QPCMF_W<QICRL_SPEC> {
        QPCMF_W::new(self, 1)
    }
    #[doc = "Bit 2 - PC and RC match interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn qprcmie(&mut self) -> QPRCMIE_W<QICRL_SPEC> {
        QPRCMIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - PC and RC match interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn qprcmf(&mut self) -> QPRCMF_W<QICRL_SPEC> {
        QPRCMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - \"Overflow, underflow, or zero index interrupt enable bit\""]
    #[inline(always)]
    #[must_use]
    pub fn ouzie(&mut self) -> OUZIE_W<QICRL_SPEC> {
        OUZIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn ufdf(&mut self) -> UFDF_W<QICRL_SPEC> {
        UFDF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Overflow interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn ofdf(&mut self) -> OFDF_W<QICRL_SPEC> {
        OFDF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Zero index interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn ziif(&mut self) -> ZIIF_W<QICRL_SPEC> {
        ZIIF_W::new(self, 7)
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
#[doc = "Low-Order Bytes of QPRC Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qicrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qicrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QICRL_SPEC;
impl crate::RegisterSpec for QICRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qicrl::R`](R) reader structure"]
impl crate::Readable for QICRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qicrl::W`](W) writer structure"]
impl crate::Writable for QICRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets QICRL to value 0"]
impl crate::Resettable for QICRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
