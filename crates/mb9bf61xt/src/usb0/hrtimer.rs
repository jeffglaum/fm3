#[doc = "Register `HRTIMER` reader"]
pub type R = crate::R<HRTIMER_SPEC>;
#[doc = "Register `HRTIMER` writer"]
pub type W = crate::W<HRTIMER_SPEC>;
#[doc = "Field `RTIMER0` reader - retry timer setting 0"]
pub type RTIMER0_R = crate::FieldReader;
#[doc = "Field `RTIMER0` writer - retry timer setting 0"]
pub type RTIMER0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTIMER1` reader - retry timer setting 1"]
pub type RTIMER1_R = crate::FieldReader;
#[doc = "Field `RTIMER1` writer - retry timer setting 1"]
pub type RTIMER1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - retry timer setting 0"]
    #[inline(always)]
    pub fn rtimer0(&self) -> RTIMER0_R {
        RTIMER0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - retry timer setting 1"]
    #[inline(always)]
    pub fn rtimer1(&self) -> RTIMER1_R {
        RTIMER1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - retry timer setting 0"]
    #[inline(always)]
    #[must_use]
    pub fn rtimer0(&mut self) -> RTIMER0_W<HRTIMER_SPEC> {
        RTIMER0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - retry timer setting 1"]
    #[inline(always)]
    #[must_use]
    pub fn rtimer1(&mut self) -> RTIMER1_W<HRTIMER_SPEC> {
        RTIMER1_W::new(self, 8)
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
#[doc = "Retry Timer Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRTIMER_SPEC;
impl crate::RegisterSpec for HRTIMER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hrtimer::R`](R) reader structure"]
impl crate::Readable for HRTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hrtimer::W`](W) writer structure"]
impl crate::Writable for HRTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HRTIMER to value 0"]
impl crate::Resettable for HRTIMER_SPEC {
    const RESET_VALUE: u16 = 0;
}
