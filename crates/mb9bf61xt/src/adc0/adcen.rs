#[doc = "Register `ADCEN` reader"]
pub type R = crate::R<ADCEN_SPEC>;
#[doc = "Register `ADCEN` writer"]
pub type W = crate::W<ADCEN_SPEC>;
#[doc = "Field `ENBL` reader - A/D operation enable bit"]
pub type ENBL_R = crate::BitReader;
#[doc = "Field `ENBL` writer - A/D operation enable bit"]
pub type ENBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - A/D operation enable state bit"]
pub type READY_R = crate::BitReader;
#[doc = "Field `CYCLSL` reader - Basic cycle selection bit"]
pub type CYCLSL_R = crate::FieldReader;
#[doc = "Field `CYCLSL` writer - Basic cycle selection bit"]
pub type CYCLSL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - A/D operation enable bit"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A/D operation enable state bit"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Basic cycle selection bit"]
    #[inline(always)]
    pub fn cyclsl(&self) -> CYCLSL_R {
        CYCLSL_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - A/D operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> ENBL_W<ADCEN_SPEC> {
        ENBL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Basic cycle selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn cyclsl(&mut self) -> CYCLSL_W<ADCEN_SPEC> {
        CYCLSL_W::new(self, 4)
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
#[doc = "A/D Operation Enable Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCEN_SPEC;
impl crate::RegisterSpec for ADCEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcen::R`](R) reader structure"]
impl crate::Readable for ADCEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcen::W`](W) writer structure"]
impl crate::Writable for ADCEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADCEN to value 0"]
impl crate::Resettable for ADCEN_SPEC {
    const RESET_VALUE: u8 = 0;
}
