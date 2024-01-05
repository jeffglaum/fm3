#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `RGIM` reader - RGMII Interrupt Mask"]
pub type RGIM_R = crate::BitReader;
#[doc = "Field `RGIM` writer - RGMII Interrupt Mask"]
pub type RGIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIM` reader - PMT Interrupt Mask"]
pub type PIM_R = crate::BitReader;
#[doc = "Field `PIM` writer - PMT Interrupt Mask"]
pub type PIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIM` reader - Time Stamp Interrupt Mask"]
pub type TSIM_R = crate::BitReader;
#[doc = "Field `TSIM` writer - Time Stamp Interrupt Mask"]
pub type TSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIIM` reader - LPI Interrupt Mask"]
pub type LPIIM_R = crate::BitReader;
#[doc = "Field `LPIIM` writer - LPI Interrupt Mask"]
pub type LPIIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RGMII Interrupt Mask"]
    #[inline(always)]
    pub fn rgim(&self) -> RGIM_R {
        RGIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pim(&self) -> PIM_R {
        PIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Time Stamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPI Interrupt Mask"]
    #[inline(always)]
    pub fn lpiim(&self) -> LPIIM_R {
        LPIIM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGMII Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rgim(&mut self) -> RGIM_W<IMR_SPEC> {
        RGIM_W::new(self, 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pim(&mut self) -> PIM_W<IMR_SPEC> {
        PIM_W::new(self, 3)
    }
    #[doc = "Bit 5 - Time Stamp Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn tsim(&mut self) -> TSIM_W<IMR_SPEC> {
        TSIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPI Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpiim(&mut self) -> LPIIM_W<IMR_SPEC> {
        LPIIM_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
