#[doc = "Register `RT_TMCR` reader"]
pub type R = crate::R<RT_RT_TMCR_SPEC>;
#[doc = "Register `RT_TMCR` writer"]
pub type W = crate::W<RT_RT_TMCR_SPEC>;
#[doc = "Field `STRG` reader - Software trigger bit"]
pub type STRG_R = crate::BitReader;
#[doc = "Field `STRG` writer - Software trigger bit"]
pub type STRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEN` reader - Timer enable bit"]
pub type CTEN_R = crate::BitReader;
#[doc = "Field `CTEN` writer - Timer enable bit"]
pub type CTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDSE` reader - Mode selection bit"]
pub type MDSE_R = crate::BitReader;
#[doc = "Field `MDSE` writer - Mode selection bit"]
pub type MDSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output polarity specification bit"]
pub type OSEL_R = crate::BitReader;
#[doc = "Field `OSEL` writer - Output polarity specification bit"]
pub type OSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMD` reader - Timer function selection bits"]
pub type FMD_R = crate::FieldReader;
#[doc = "Field `FMD` writer - Timer function selection bits"]
pub type FMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `T32` reader - 32-bit timer selection bit"]
pub type T32_R = crate::BitReader;
#[doc = "Field `T32` writer - 32-bit timer selection bit"]
pub type T32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGS` reader - Trigger input edge selection bits"]
pub type EGS_R = crate::FieldReader;
#[doc = "Field `EGS` writer - Trigger input edge selection bits"]
pub type EGS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKS2_0` reader - Count clock selection bit"]
pub type CKS2_0_R = crate::FieldReader;
#[doc = "Field `CKS2_0` writer - Count clock selection bit"]
pub type CKS2_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Software trigger bit"]
    #[inline(always)]
    pub fn strg(&self) -> STRG_R {
        STRG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer enable bit"]
    #[inline(always)]
    pub fn cten(&self) -> CTEN_R {
        CTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode selection bit"]
    #[inline(always)]
    pub fn mdse(&self) -> MDSE_R {
        MDSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output polarity specification bit"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Timer function selection bits"]
    #[inline(always)]
    pub fn fmd(&self) -> FMD_R {
        FMD_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 32-bit timer selection bit"]
    #[inline(always)]
    pub fn t32(&self) -> T32_R {
        T32_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trigger input edge selection bits"]
    #[inline(always)]
    pub fn egs(&self) -> EGS_R {
        EGS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Count clock selection bit"]
    #[inline(always)]
    pub fn cks2_0(&self) -> CKS2_0_R {
        CKS2_0_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger bit"]
    #[inline(always)]
    #[must_use]
    pub fn strg(&mut self) -> STRG_W<RT_RT_TMCR_SPEC> {
        STRG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cten(&mut self) -> CTEN_W<RT_RT_TMCR_SPEC> {
        CTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn mdse(&mut self) -> MDSE_W<RT_RT_TMCR_SPEC> {
        MDSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output polarity specification bit"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<RT_RT_TMCR_SPEC> {
        OSEL_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Timer function selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn fmd(&mut self) -> FMD_W<RT_RT_TMCR_SPEC> {
        FMD_W::new(self, 4)
    }
    #[doc = "Bit 7 - 32-bit timer selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn t32(&mut self) -> T32_W<RT_RT_TMCR_SPEC> {
        T32_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Trigger input edge selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn egs(&mut self) -> EGS_W<RT_RT_TMCR_SPEC> {
        EGS_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Count clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn cks2_0(&mut self) -> CKS2_0_W<RT_RT_TMCR_SPEC> {
        CKS2_0_W::new(self, 12)
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
#[doc = "Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_tmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_tmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RT_RT_TMCR_SPEC;
impl crate::RegisterSpec for RT_RT_TMCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rt_rt_tmcr::R`](R) reader structure"]
impl crate::Readable for RT_RT_TMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rt_rt_tmcr::W`](W) writer structure"]
impl crate::Writable for RT_RT_TMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RT_TMCR to value 0"]
impl crate::Resettable for RT_RT_TMCR_SPEC {
    const RESET_VALUE: u16 = 0;
}
