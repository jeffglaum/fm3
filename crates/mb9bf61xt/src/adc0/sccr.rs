#[doc = "Register `SCCR` reader"]
pub type R = crate::R<SCCR_SPEC>;
#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SCCR_SPEC>;
#[doc = "Field `SSTR` reader - Scan conversion start bit"]
pub type SSTR_R = crate::BitReader;
#[doc = "Field `SSTR` writer - Scan conversion start bit"]
pub type SSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHEN` reader - Scan conversion timer start enable bit"]
pub type SHEN_R = crate::BitReader;
#[doc = "Field `SHEN` writer - Scan conversion timer start enable bit"]
pub type SHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPT` reader - Scan conversion repeat bit"]
pub type RPT_R = crate::BitReader;
#[doc = "Field `RPT` writer - Scan conversion repeat bit"]
pub type RPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFCLR` reader - Scan conversion FIFO clear bit"]
pub type SFCLR_R = crate::BitReader;
#[doc = "Field `SFCLR` writer - Scan conversion FIFO clear bit"]
pub type SFCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOVR` reader - Scan conversion overrun flag"]
pub type SOVR_R = crate::BitReader;
#[doc = "Field `SOVR` writer - Scan conversion overrun flag"]
pub type SOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFUL` reader - Scan conversion FIFO full bit"]
pub type SFUL_R = crate::BitReader;
#[doc = "Field `SEMP` reader - Scan conversion FIFO empty bit"]
pub type SEMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Scan conversion start bit"]
    #[inline(always)]
    pub fn sstr(&self) -> SSTR_R {
        SSTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan conversion timer start enable bit"]
    #[inline(always)]
    pub fn shen(&self) -> SHEN_R {
        SHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan conversion repeat bit"]
    #[inline(always)]
    pub fn rpt(&self) -> RPT_R {
        RPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan conversion FIFO clear bit"]
    #[inline(always)]
    pub fn sfclr(&self) -> SFCLR_R {
        SFCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan conversion overrun flag"]
    #[inline(always)]
    pub fn sovr(&self) -> SOVR_R {
        SOVR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan conversion FIFO full bit"]
    #[inline(always)]
    pub fn sful(&self) -> SFUL_R {
        SFUL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan conversion FIFO empty bit"]
    #[inline(always)]
    pub fn semp(&self) -> SEMP_R {
        SEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan conversion start bit"]
    #[inline(always)]
    #[must_use]
    pub fn sstr(&mut self) -> SSTR_W<SCCR_SPEC> {
        SSTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Scan conversion timer start enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn shen(&mut self) -> SHEN_W<SCCR_SPEC> {
        SHEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Scan conversion repeat bit"]
    #[inline(always)]
    #[must_use]
    pub fn rpt(&mut self) -> RPT_W<SCCR_SPEC> {
        RPT_W::new(self, 2)
    }
    #[doc = "Bit 4 - Scan conversion FIFO clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn sfclr(&mut self) -> SFCLR_W<SCCR_SPEC> {
        SFCLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Scan conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn sovr(&mut self) -> SOVR_W<SCCR_SPEC> {
        SOVR_W::new(self, 5)
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
#[doc = "Scan Conversion Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCR_SPEC;
impl crate::RegisterSpec for SCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccr::R`](R) reader structure"]
impl crate::Readable for SCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCCR to value 0x80"]
impl crate::Resettable for SCCR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
