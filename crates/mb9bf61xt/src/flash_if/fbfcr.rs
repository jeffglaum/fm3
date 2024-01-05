#[doc = "Register `FBFCR` reader"]
pub type R = crate::R<FBFCR_SPEC>;
#[doc = "Register `FBFCR` writer"]
pub type W = crate::W<FBFCR_SPEC>;
#[doc = "Field `BE` reader - Buffer Enable"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - Buffer Enable"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS` reader - Buffer Status"]
pub type BS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Buffer Enable"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer Status"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<FBFCR_SPEC> {
        BE_W::new(self, 0)
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
#[doc = "Flash Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBFCR_SPEC;
impl crate::RegisterSpec for FBFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbfcr::R`](R) reader structure"]
impl crate::Readable for FBFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fbfcr::W`](W) writer structure"]
impl crate::Writable for FBFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBFCR to value 0"]
impl crate::Resettable for FBFCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
