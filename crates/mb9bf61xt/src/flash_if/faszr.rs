#[doc = "Register `FASZR` reader"]
pub type R = crate::R<FASZR_SPEC>;
#[doc = "Register `FASZR` writer"]
pub type W = crate::W<FASZR_SPEC>;
#[doc = "Field `ASZ` reader - Flash Access Size"]
pub type ASZ_R = crate::FieldReader;
#[doc = "Field `ASZ` writer - Flash Access Size"]
pub type ASZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Flash Access Size"]
    #[inline(always)]
    pub fn asz(&self) -> ASZ_R {
        ASZ_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash Access Size"]
    #[inline(always)]
    #[must_use]
    pub fn asz(&mut self) -> ASZ_W<FASZR_SPEC> {
        ASZ_W::new(self, 0)
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
#[doc = "Flash Access Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faszr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faszr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FASZR_SPEC;
impl crate::RegisterSpec for FASZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faszr::R`](R) reader structure"]
impl crate::Readable for FASZR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`faszr::W`](W) writer structure"]
impl crate::Writable for FASZR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FASZR to value 0x02"]
impl crate::Resettable for FASZR_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
