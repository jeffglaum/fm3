#[doc = "Register `EP0C` reader"]
pub type R = crate::R<EP0C_SPEC>;
#[doc = "Register `EP0C` writer"]
pub type W = crate::W<EP0C_SPEC>;
#[doc = "Field `PKS0` reader - Packet Size Endpoint 0 Setting bits"]
pub type PKS0_R = crate::FieldReader;
#[doc = "Field `PKS0` writer - Packet Size Endpoint 0 Setting bits"]
pub type PKS0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `STAL` reader - Endpoint 0 Stall Setting bit"]
pub type STAL_R = crate::BitReader;
#[doc = "Field `STAL` writer - Endpoint 0 Stall Setting bit"]
pub type STAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Packet Size Endpoint 0 Setting bits"]
    #[inline(always)]
    pub fn pks0(&self) -> PKS0_R {
        PKS0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Endpoint 0 Stall Setting bit"]
    #[inline(always)]
    pub fn stal(&self) -> STAL_R {
        STAL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Packet Size Endpoint 0 Setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn pks0(&mut self) -> PKS0_W<EP0C_SPEC> {
        PKS0_W::new(self, 0)
    }
    #[doc = "Bit 9 - Endpoint 0 Stall Setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn stal(&mut self) -> STAL_W<EP0C_SPEC> {
        STAL_W::new(self, 9)
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
#[doc = "EP0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP0C_SPEC;
impl crate::RegisterSpec for EP0C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0c::R`](R) reader structure"]
impl crate::Readable for EP0C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep0c::W`](W) writer structure"]
impl crate::Writable for EP0C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP0C to value 0x40"]
impl crate::Resettable for EP0C_SPEC {
    const RESET_VALUE: u16 = 0x40;
}
