#[doc = "Register `CSIO_BGR` reader"]
pub type R = crate::R<CSIO_CSIO_BGR_SPEC>;
#[doc = "Register `CSIO_BGR` writer"]
pub type W = crate::W<CSIO_CSIO_BGR_SPEC>;
#[doc = "Field `BGR0` reader - Baud Rate Generator Registers 0"]
pub type BGR0_R = crate::FieldReader;
#[doc = "Field `BGR0` writer - Baud Rate Generator Registers 0"]
pub type BGR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BGR1` reader - Baud Rate Generator Registers 1"]
pub type BGR1_R = crate::FieldReader;
#[doc = "Field `BGR1` writer - Baud Rate Generator Registers 1"]
pub type BGR1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - Baud Rate Generator Registers 0"]
    #[inline(always)]
    pub fn bgr0(&self) -> BGR0_R {
        BGR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Baud Rate Generator Registers 1"]
    #[inline(always)]
    pub fn bgr1(&self) -> BGR1_R {
        BGR1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Generator Registers 0"]
    #[inline(always)]
    #[must_use]
    pub fn bgr0(&mut self) -> BGR0_W<CSIO_CSIO_BGR_SPEC> {
        BGR0_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Baud Rate Generator Registers 1"]
    #[inline(always)]
    #[must_use]
    pub fn bgr1(&mut self) -> BGR1_W<CSIO_CSIO_BGR_SPEC> {
        BGR1_W::new(self, 8)
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
#[doc = "Baud Rate Generator Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csio_csio_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csio_csio_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIO_CSIO_BGR_SPEC;
impl crate::RegisterSpec for CSIO_CSIO_BGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csio_csio_bgr::R`](R) reader structure"]
impl crate::Readable for CSIO_CSIO_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csio_csio_bgr::W`](W) writer structure"]
impl crate::Writable for CSIO_CSIO_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CSIO_BGR to value 0"]
impl crate::Resettable for CSIO_CSIO_BGR_SPEC {
    const RESET_VALUE: u16 = 0;
}
