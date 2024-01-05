#[doc = "Register `PFNS` reader"]
pub type R = crate::R<PFNS_SPEC>;
#[doc = "Register `PFNS` writer"]
pub type W = crate::W<PFNS_SPEC>;
#[doc = "Field `PFS` reader - Priority conversion FIFO stage count setting bits"]
pub type PFS_R = crate::FieldReader;
#[doc = "Field `PFS` writer - Priority conversion FIFO stage count setting bits"]
pub type PFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TEST` reader - Test bits"]
pub type TEST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Priority conversion FIFO stage count setting bits"]
    #[inline(always)]
    pub fn pfs(&self) -> PFS_R {
        PFS_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Test bits"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Priority conversion FIFO stage count setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn pfs(&mut self) -> PFS_W<PFNS_SPEC> {
        PFS_W::new(self, 0)
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
#[doc = "Priority Conversion FIFO Stage Count Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfns::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfns::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFNS_SPEC;
impl crate::RegisterSpec for PFNS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pfns::R`](R) reader structure"]
impl crate::Readable for PFNS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfns::W`](W) writer structure"]
impl crate::Writable for PFNS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PFNS to value 0"]
impl crate::Resettable for PFNS_SPEC {
    const RESET_VALUE: u8 = 0;
}
