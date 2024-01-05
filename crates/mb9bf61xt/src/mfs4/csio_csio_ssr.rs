#[doc = "Register `CSIO_SSR` reader"]
pub type R = crate::R<CSIO_CSIO_SSR_SPEC>;
#[doc = "Register `CSIO_SSR` writer"]
pub type W = crate::W<CSIO_CSIO_SSR_SPEC>;
#[doc = "Field `TBI` reader - Transmit bus idle flag bit"]
pub type TBI_R = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit data empty flag bit"]
pub type TDRE_R = crate::BitReader;
#[doc = "Field `RDRF` reader - Received data full flag bit"]
pub type RDRF_R = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error flag bit"]
pub type ORE_R = crate::BitReader;
#[doc = "Field `REC` reader - Received error flag clear bit"]
pub type REC_R = crate::BitReader;
#[doc = "Field `REC` writer - Received error flag clear bit"]
pub type REC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit bus idle flag bit"]
    #[inline(always)]
    pub fn tbi(&self) -> TBI_R {
        TBI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data empty flag bit"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received data full flag bit"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error flag bit"]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Received error flag clear bit"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Received error flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<CSIO_CSIO_SSR_SPEC> {
        REC_W::new(self, 7)
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
#[doc = "Serial Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_ssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_ssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIO_CSIO_SSR_SPEC;
impl crate::RegisterSpec for CSIO_CSIO_SSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csio_csio_ssr::R`](R) reader structure"]
impl crate::Readable for CSIO_CSIO_SSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csio_csio_ssr::W`](W) writer structure"]
impl crate::Writable for CSIO_CSIO_SSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSIO_SSR to value 0x03"]
impl crate::Resettable for CSIO_CSIO_SSR_SPEC {
    const RESET_VALUE: u8 = 0x03;
}
