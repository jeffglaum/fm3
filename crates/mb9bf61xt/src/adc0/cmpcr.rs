#[doc = "Register `CMPCR` reader"]
pub type R = crate::R<CMPCR_SPEC>;
#[doc = "Register `CMPCR` writer"]
pub type W = crate::W<CMPCR_SPEC>;
#[doc = "Field `CCH` reader - Comparison target analog input channel"]
pub type CCH_R = crate::FieldReader;
#[doc = "Field `CCH` writer - Comparison target analog input channel"]
pub type CCH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CMD0` reader - Comparison mode 0"]
pub type CMD0_R = crate::BitReader;
#[doc = "Field `CMD0` writer - Comparison mode 0"]
pub type CMD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD1` reader - Comparison mode 1"]
pub type CMD1_R = crate::BitReader;
#[doc = "Field `CMD1` writer - Comparison mode 1"]
pub type CMD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN` reader - Conversion result comparison function operation enable bit"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - Conversion result comparison function operation enable bit"]
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Comparison target analog input channel"]
    #[inline(always)]
    pub fn cch(&self) -> CCH_R {
        CCH_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Comparison mode 0"]
    #[inline(always)]
    pub fn cmd0(&self) -> CMD0_R {
        CMD0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison mode 1"]
    #[inline(always)]
    pub fn cmd1(&self) -> CMD1_R {
        CMD1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion result comparison function operation enable bit"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Comparison target analog input channel"]
    #[inline(always)]
    #[must_use]
    pub fn cch(&mut self) -> CCH_W<CMPCR_SPEC> {
        CCH_W::new(self, 0)
    }
    #[doc = "Bit 5 - Comparison mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmd0(&mut self) -> CMD0_W<CMPCR_SPEC> {
        CMD0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparison mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1(&mut self) -> CMD1_W<CMPCR_SPEC> {
        CMD1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Conversion result comparison function operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<CMPCR_SPEC> {
        CMPEN_W::new(self, 7)
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
#[doc = "A/D Comparison Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPCR_SPEC;
impl crate::RegisterSpec for CMPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cmpcr::R`](R) reader structure"]
impl crate::Readable for CMPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpcr::W`](W) writer structure"]
impl crate::Writable for CMPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CMPCR to value 0"]
impl crate::Resettable for CMPCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
