#[doc = "Register `PCCR` reader"]
pub type R = crate::R<PCCR_SPEC>;
#[doc = "Register `PCCR` writer"]
pub type W = crate::W<PCCR_SPEC>;
#[doc = "Field `PSTR` reader - Priority conversion start bit"]
pub type PSTR_R = crate::BitReader;
#[doc = "Field `PSTR` writer - Priority conversion start bit"]
pub type PSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHEN` reader - Priority conversion timer start enable bit"]
pub type PHEN_R = crate::BitReader;
#[doc = "Field `PHEN` writer - Priority conversion timer start enable bit"]
pub type PHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEEN` reader - Priority conversion external start enable bit"]
pub type PEEN_R = crate::BitReader;
#[doc = "Field `PEEN` writer - Priority conversion external start enable bit"]
pub type PEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCE` reader - External trigger analog input selection bit"]
pub type ESCE_R = crate::BitReader;
#[doc = "Field `ESCE` writer - External trigger analog input selection bit"]
pub type ESCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCLR` reader - Priority conversion FIFO clear bit"]
pub type PFCLR_R = crate::BitReader;
#[doc = "Field `PFCLR` writer - Priority conversion FIFO clear bit"]
pub type PFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POVR` reader - Priority conversion overrun flag"]
pub type POVR_R = crate::BitReader;
#[doc = "Field `POVR` writer - Priority conversion overrun flag"]
pub type POVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFUL` reader - Priority conversion FIFO full bit"]
pub type PFUL_R = crate::BitReader;
#[doc = "Field `PEMP` reader - Priority conversion FIFO empty bit"]
pub type PEMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Priority conversion start bit"]
    #[inline(always)]
    pub fn pstr(&self) -> PSTR_R {
        PSTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Priority conversion timer start enable bit"]
    #[inline(always)]
    pub fn phen(&self) -> PHEN_R {
        PHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Priority conversion external start enable bit"]
    #[inline(always)]
    pub fn peen(&self) -> PEEN_R {
        PEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External trigger analog input selection bit"]
    #[inline(always)]
    pub fn esce(&self) -> ESCE_R {
        ESCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Priority conversion FIFO clear bit"]
    #[inline(always)]
    pub fn pfclr(&self) -> PFCLR_R {
        PFCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Priority conversion overrun flag"]
    #[inline(always)]
    pub fn povr(&self) -> POVR_R {
        POVR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Priority conversion FIFO full bit"]
    #[inline(always)]
    pub fn pful(&self) -> PFUL_R {
        PFUL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Priority conversion FIFO empty bit"]
    #[inline(always)]
    pub fn pemp(&self) -> PEMP_R {
        PEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Priority conversion start bit"]
    #[inline(always)]
    #[must_use]
    pub fn pstr(&mut self) -> PSTR_W<PCCR_SPEC> {
        PSTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Priority conversion timer start enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn phen(&mut self) -> PHEN_W<PCCR_SPEC> {
        PHEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Priority conversion external start enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn peen(&mut self) -> PEEN_W<PCCR_SPEC> {
        PEEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - External trigger analog input selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn esce(&mut self) -> ESCE_W<PCCR_SPEC> {
        ESCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Priority conversion FIFO clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn pfclr(&mut self) -> PFCLR_W<PCCR_SPEC> {
        PFCLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Priority conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn povr(&mut self) -> POVR_W<PCCR_SPEC> {
        POVR_W::new(self, 5)
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
#[doc = "Priority Conversion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCCR_SPEC;
impl crate::RegisterSpec for PCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pccr::R`](R) reader structure"]
impl crate::Readable for PCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pccr::W`](W) writer structure"]
impl crate::Writable for PCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PCCR to value 0x80"]
impl crate::Resettable for PCCR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
