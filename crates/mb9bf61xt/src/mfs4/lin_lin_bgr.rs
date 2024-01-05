#[doc = "Register `LIN_BGR` reader"]
pub type R = crate::R<LIN_LIN_BGR_SPEC>;
#[doc = "Register `LIN_BGR` writer"]
pub type W = crate::W<LIN_LIN_BGR_SPEC>;
#[doc = "Field `BGR0` reader - Baud Rate Generator Registers 0"]
pub type BGR0_R = crate::FieldReader;
#[doc = "Field `BGR0` writer - Baud Rate Generator Registers 0"]
pub type BGR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BGR1` reader - Baud Rate Generator Registers 1"]
pub type BGR1_R = crate::FieldReader;
#[doc = "Field `BGR1` writer - Baud Rate Generator Registers 1"]
pub type BGR1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EXT` reader - External clock select bit"]
pub type EXT_R = crate::BitReader;
#[doc = "Field `EXT` writer - External clock select bit"]
pub type EXT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 15 - External clock select bit"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Rate Generator Registers 0"]
    #[inline(always)]
    #[must_use]
    pub fn bgr0(&mut self) -> BGR0_W<LIN_LIN_BGR_SPEC> {
        BGR0_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Baud Rate Generator Registers 1"]
    #[inline(always)]
    #[must_use]
    pub fn bgr1(&mut self) -> BGR1_W<LIN_LIN_BGR_SPEC> {
        BGR1_W::new(self, 8)
    }
    #[doc = "Bit 15 - External clock select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> EXT_W<LIN_LIN_BGR_SPEC> {
        EXT_W::new(self, 15)
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
#[doc = "Baud Rate Generator Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIN_LIN_BGR_SPEC;
impl crate::RegisterSpec for LIN_LIN_BGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lin_lin_bgr::R`](R) reader structure"]
impl crate::Readable for LIN_LIN_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lin_lin_bgr::W`](W) writer structure"]
impl crate::Writable for LIN_LIN_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LIN_BGR to value 0"]
impl crate::Resettable for LIN_LIN_BGR_SPEC {
    const RESET_VALUE: u16 = 0;
}
