#[doc = "Register `DCLKR` reader"]
pub type R = crate::R<DCLKR_SPEC>;
#[doc = "Register `DCLKR` writer"]
pub type W = crate::W<DCLKR_SPEC>;
#[doc = "Field `MDIV` reader - MCLK Division Ratio Setup"]
pub type MDIV_R = crate::FieldReader;
#[doc = "Field `MDIV` writer - MCLK Division Ratio Setup"]
pub type MDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCLKON` reader - MCLK ON"]
pub type MCLKON_R = crate::BitReader;
#[doc = "Field `MCLKON` writer - MCLK ON"]
pub type MCLKON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - MCLK Division Ratio Setup"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - MCLK ON"]
    #[inline(always)]
    pub fn mclkon(&self) -> MCLKON_R {
        MCLKON_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCLK Division Ratio Setup"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv(&mut self) -> MDIV_W<DCLKR_SPEC> {
        MDIV_W::new(self, 0)
    }
    #[doc = "Bit 4 - MCLK ON"]
    #[inline(always)]
    #[must_use]
    pub fn mclkon(&mut self) -> MCLKON_W<DCLKR_SPEC> {
        MCLKON_W::new(self, 4)
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
#[doc = "Division Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dclkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dclkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCLKR_SPEC;
impl crate::RegisterSpec for DCLKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dclkr::R`](R) reader structure"]
impl crate::Readable for DCLKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dclkr::W`](W) writer structure"]
impl crate::Writable for DCLKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCLKR to value 0x01"]
impl crate::Resettable for DCLKR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
