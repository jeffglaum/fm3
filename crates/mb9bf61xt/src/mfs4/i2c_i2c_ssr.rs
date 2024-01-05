#[doc = "Register `I2C_SSR` reader"]
pub type R = crate::R<I2C_I2C_SSR_SPEC>;
#[doc = "Register `I2C_SSR` writer"]
pub type W = crate::W<I2C_I2C_SSR_SPEC>;
#[doc = "Field `TBI` reader - Transmit bus idle flag bit (Effective only when DMA mode is enabled)"]
pub type TBI_R = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit data empty flag bit"]
pub type TDRE_R = crate::BitReader;
#[doc = "Field `RDRF` reader - Received data full flag bit"]
pub type RDRF_R = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error flag bit"]
pub type ORE_R = crate::BitReader;
#[doc = "Field `TBIE` reader - Transmit bus idle interrupt enable bit (Effective only when DMA mode is enabled)"]
pub type TBIE_R = crate::BitReader;
#[doc = "Field `TBIE` writer - Transmit bus idle interrupt enable bit (Effective only when DMA mode is enabled)"]
pub type TBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA mode enable bit"]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - DMA mode enable bit"]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSET` reader - Transmit empty flag set bit"]
pub type TSET_R = crate::BitReader;
#[doc = "Field `TSET` writer - Transmit empty flag set bit"]
pub type TSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC` reader - Received error flag clear bit"]
pub type REC_R = crate::BitReader;
#[doc = "Field `REC` writer - Received error flag clear bit"]
pub type REC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit bus idle flag bit (Effective only when DMA mode is enabled)"]
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
    #[doc = "Bit 4 - Transmit bus idle interrupt enable bit (Effective only when DMA mode is enabled)"]
    #[inline(always)]
    pub fn tbie(&self) -> TBIE_R {
        TBIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA mode enable bit"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit empty flag set bit"]
    #[inline(always)]
    pub fn tset(&self) -> TSET_R {
        TSET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received error flag clear bit"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit bus idle interrupt enable bit (Effective only when DMA mode is enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn tbie(&mut self) -> TBIE_W<I2C_I2C_SSR_SPEC> {
        TBIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<I2C_I2C_SSR_SPEC> {
        DMA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit empty flag set bit"]
    #[inline(always)]
    #[must_use]
    pub fn tset(&mut self) -> TSET_W<I2C_I2C_SSR_SPEC> {
        TSET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Received error flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<I2C_I2C_SSR_SPEC> {
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
#[doc = "Serial Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_SSR_SPEC;
impl crate::RegisterSpec for I2C_I2C_SSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_ssr::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_SSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_ssr::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_SSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_SSR to value 0x03"]
impl crate::Resettable for I2C_I2C_SSR_SPEC {
    const RESET_VALUE: u8 = 0x03;
}
