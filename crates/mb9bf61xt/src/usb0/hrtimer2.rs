#[doc = "Register `HRTIMER2` reader"]
pub type R = crate::R<HRTIMER2_SPEC>;
#[doc = "Register `HRTIMER2` writer"]
pub type W = crate::W<HRTIMER2_SPEC>;
#[doc = "Field `RTIMER2` reader - retry timer setting 2"]
pub type RTIMER2_R = crate::FieldReader;
#[doc = "Field `RTIMER2` writer - retry timer setting 2"]
pub type RTIMER2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - retry timer setting 2"]
    #[inline(always)]
    pub fn rtimer2(&self) -> RTIMER2_R {
        RTIMER2_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - retry timer setting 2"]
    #[inline(always)]
    #[must_use]
    pub fn rtimer2(&mut self) -> RTIMER2_W<HRTIMER2_SPEC> {
        RTIMER2_W::new(self, 0)
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
#[doc = "Retry Timer Setup Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrtimer2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrtimer2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRTIMER2_SPEC;
impl crate::RegisterSpec for HRTIMER2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hrtimer2::R`](R) reader structure"]
impl crate::Readable for HRTIMER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrtimer2::W`](W) writer structure"]
impl crate::Writable for HRTIMER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HRTIMER2 to value 0"]
impl crate::Resettable for HRTIMER2_SPEC {
    const RESET_VALUE: u8 = 0;
}
