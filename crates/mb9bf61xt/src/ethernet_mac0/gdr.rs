#[doc = "Register `GDR` reader"]
pub type R = crate::R<GDR_SPEC>;
#[doc = "Register `GDR` writer"]
pub type W = crate::W<GDR_SPEC>;
#[doc = "Field `GD` reader - GMII/MII Data Register"]
pub type GD_R = crate::FieldReader<u16>;
#[doc = "Field `GD` writer - GMII/MII Data Register"]
pub type GD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GMII/MII Data Register"]
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GMII/MII Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn gd(&mut self) -> GD_W<GDR_SPEC> {
        GD_W::new(self, 0)
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
#[doc = "GMII/MII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDR_SPEC;
impl crate::RegisterSpec for GDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdr::R`](R) reader structure"]
impl crate::Readable for GDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdr::W`](W) writer structure"]
impl crate::Writable for GDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDR to value 0"]
impl crate::Resettable for GDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
