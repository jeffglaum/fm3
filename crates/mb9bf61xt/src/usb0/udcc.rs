#[doc = "Register `UDCC` reader"]
pub type R = crate::R<UDCC_SPEC>;
#[doc = "Register `UDCC` writer"]
pub type W = crate::W<UDCC_SPEC>;
#[doc = "Field `PWC` reader - Power Control bit"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - Power Control bit"]
pub type PWC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFBK` reader - Data Toggle Mode Select bit"]
pub type RFBK_R = crate::BitReader;
#[doc = "Field `RFBK` writer - Data Toggle Mode Select bit"]
pub type RFBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALCLREN` reader - Endpoint 1 to 5 STAL bit Clear Select bit"]
pub type STALCLREN_R = crate::BitReader;
#[doc = "Field `STALCLREN` writer - Endpoint 1 to 5 STAL bit Clear Select bit"]
pub type STALCLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USTP` reader - USB Operating Clock Stop bit"]
pub type USTP_R = crate::BitReader;
#[doc = "Field `USTP` writer - USB Operating Clock Stop bit"]
pub type USTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCONX` reader - Host Connection bit"]
pub type HCONX_R = crate::BitReader;
#[doc = "Field `HCONX` writer - Host Connection bit"]
pub type HCONX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUM` reader - Resume Setting bit"]
pub type RESUM_R = crate::BitReader;
#[doc = "Field `RESUM` writer - Resume Setting bit"]
pub type RESUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Function Reset bit"]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Function Reset bit"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Control bit"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle Mode Select bit"]
    #[inline(always)]
    pub fn rfbk(&self) -> RFBK_R {
        RFBK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 1 to 5 STAL bit Clear Select bit"]
    #[inline(always)]
    pub fn stalclren(&self) -> STALCLREN_R {
        STALCLREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB Operating Clock Stop bit"]
    #[inline(always)]
    pub fn ustp(&self) -> USTP_R {
        USTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Connection bit"]
    #[inline(always)]
    pub fn hconx(&self) -> HCONX_R {
        HCONX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resume Setting bit"]
    #[inline(always)]
    pub fn resum(&self) -> RESUM_R {
        RESUM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Function Reset bit"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Control bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<UDCC_SPEC> {
        PWC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data Toggle Mode Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn rfbk(&mut self) -> RFBK_W<UDCC_SPEC> {
        RFBK_W::new(self, 1)
    }
    #[doc = "Bit 3 - Endpoint 1 to 5 STAL bit Clear Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn stalclren(&mut self) -> STALCLREN_W<UDCC_SPEC> {
        STALCLREN_W::new(self, 3)
    }
    #[doc = "Bit 4 - USB Operating Clock Stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn ustp(&mut self) -> USTP_W<UDCC_SPEC> {
        USTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Host Connection bit"]
    #[inline(always)]
    #[must_use]
    pub fn hconx(&mut self) -> HCONX_W<UDCC_SPEC> {
        HCONX_W::new(self, 5)
    }
    #[doc = "Bit 6 - Resume Setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn resum(&mut self) -> RESUM_W<UDCC_SPEC> {
        RESUM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Function Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<UDCC_SPEC> {
        RST_W::new(self, 7)
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
#[doc = "UDC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDCC_SPEC;
impl crate::RegisterSpec for UDCC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`udcc::R`](R) reader structure"]
impl crate::Readable for UDCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udcc::W`](W) writer structure"]
impl crate::Writable for UDCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UDCC to value 0xa0"]
impl crate::Resettable for UDCC_SPEC {
    const RESET_VALUE: u16 = 0xa0;
}
