#[doc = "Register `AREA4` reader"]
pub type R = crate::R<AREA4_SPEC>;
#[doc = "Register `AREA4` writer"]
pub type W = crate::W<AREA4_SPEC>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASK` reader - address mask"]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - address mask"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - address mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<AREA4_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - address mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<AREA4_SPEC> {
        MASK_W::new(self, 16)
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
#[doc = "Area Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`area4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`area4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AREA4_SPEC;
impl crate::RegisterSpec for AREA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`area4::R`](R) reader structure"]
impl crate::Readable for AREA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`area4::W`](W) writer structure"]
impl crate::Writable for AREA4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AREA4 to value 0x000f_0040"]
impl crate::Resettable for AREA4_SPEC {
    const RESET_VALUE: u32 = 0x000f_0040;
}
