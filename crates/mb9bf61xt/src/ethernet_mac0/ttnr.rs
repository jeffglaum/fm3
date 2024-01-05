#[doc = "Register `TTNR` reader"]
pub type R = crate::R<TTNR_SPEC>;
#[doc = "Register `TTNR` writer"]
pub type W = crate::W<TTNR_SPEC>;
#[doc = "Field `TSTR` reader - Target Time Stamp Nanoseconds Register"]
pub type TSTR_R = crate::FieldReader<u32>;
#[doc = "Field `TSTR` writer - Target Time Stamp Nanoseconds Register"]
pub type TSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Target Time Stamp Nanoseconds Register"]
    #[inline(always)]
    pub fn tstr(&self) -> TSTR_R {
        TSTR_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Stamp Nanoseconds Register"]
    #[inline(always)]
    #[must_use]
    pub fn tstr(&mut self) -> TSTR_W<TTNR_SPEC> {
        TSTR_W::new(self, 0)
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
#[doc = "Target Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTNR_SPEC;
impl crate::RegisterSpec for TTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttnr::R`](R) reader structure"]
impl crate::Readable for TTNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ttnr::W`](W) writer structure"]
impl crate::Writable for TTNR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTNR to value 0"]
impl crate::Resettable for TTNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
