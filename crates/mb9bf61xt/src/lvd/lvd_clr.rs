#[doc = "Register `LVD_CLR` reader"]
pub type R = crate::R<LVD_CLR_SPEC>;
#[doc = "Register `LVD_CLR` writer"]
pub type W = crate::W<LVD_CLR_SPEC>;
#[doc = "Field `LVDCL` reader - Low-voltage detection interrupt clear bit"]
pub type LVDCL_R = crate::BitReader;
#[doc = "Field `LVDCL` writer - Low-voltage detection interrupt clear bit"]
pub type LVDCL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Low-voltage detection interrupt clear bit"]
    #[inline(always)]
    pub fn lvdcl(&self) -> LVDCL_R {
        LVDCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Low-voltage detection interrupt clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcl(&mut self) -> LVDCL_W<LVD_CLR_SPEC> {
        LVDCL_W::new(self, 7)
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
#[doc = "Low-voltage Detection Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvd_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVD_CLR_SPEC;
impl crate::RegisterSpec for LVD_CLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvd_clr::R`](R) reader structure"]
impl crate::Readable for LVD_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvd_clr::W`](W) writer structure"]
impl crate::Writable for LVD_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVD_CLR to value 0x80"]
impl crate::Resettable for LVD_CLR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
