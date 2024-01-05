#[doc = "Register `CRCIN` reader"]
pub type R = crate::R<CRCIN_SPEC>;
#[doc = "Register `CRCIN` writer"]
pub type W = crate::W<CRCIN_SPEC>;
#[doc = "Field `D` reader - Input data"]
pub type D_R = crate::FieldReader<u32>;
#[doc = "Field `D` writer - Input data"]
pub type D_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input data"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input data"]
    #[inline(always)]
    #[must_use]
    pub fn d(&mut self) -> D_W<CRCIN_SPEC> {
        D_W::new(self, 0)
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
#[doc = "Input Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCIN_SPEC;
impl crate::RegisterSpec for CRCIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcin::R`](R) reader structure"]
impl crate::Readable for CRCIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcin::W`](W) writer structure"]
impl crate::Writable for CRCIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCIN to value 0"]
impl crate::Resettable for CRCIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
