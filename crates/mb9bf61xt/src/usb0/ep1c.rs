#[doc = "Register `EP1C` reader"]
pub type R = crate::R<EP1C_SPEC>;
#[doc = "Register `EP1C` writer"]
pub type W = crate::W<EP1C_SPEC>;
#[doc = "Field `PKS` reader - Packet Size Setting bits"]
pub type PKS_R = crate::FieldReader<u16>;
#[doc = "Field `PKS` writer - Packet Size Setting bits"]
pub type PKS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `STAL` reader - Endpoint Stall Setting bit"]
pub type STAL_R = crate::BitReader;
#[doc = "Field `STAL` writer - Endpoint Stall Setting bit"]
pub type STAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NULE` reader - Null Automatic Transfer Enable bit"]
pub type NULE_R = crate::BitReader;
#[doc = "Field `NULE` writer - Null Automatic Transfer Enable bit"]
pub type NULE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` reader - DMA Automatic Transfer Enable bit"]
pub type DMAE_R = crate::BitReader;
#[doc = "Field `DMAE` writer - DMA Automatic Transfer Enable bit"]
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Endpoint Transfer Direction Select bit"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Endpoint Transfer Direction Select bit"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE` reader - Endpoint Transfer Type Select bits"]
pub type TYPE_R = crate::FieldReader;
#[doc = "Field `TYPE` writer - Endpoint Transfer Type Select bits"]
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPEN` reader - Endpoint Enable bit"]
pub type EPEN_R = crate::BitReader;
#[doc = "Field `EPEN` writer - Endpoint Enable bit"]
pub type EPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Packet Size Setting bits"]
    #[inline(always)]
    pub fn pks(&self) -> PKS_R {
        PKS_R::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 9 - Endpoint Stall Setting bit"]
    #[inline(always)]
    pub fn stal(&self) -> STAL_R {
        STAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Null Automatic Transfer Enable bit"]
    #[inline(always)]
    pub fn nule(&self) -> NULE_R {
        NULE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Automatic Transfer Enable bit"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint Transfer Direction Select bit"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Endpoint Transfer Type Select bits"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Endpoint Enable bit"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Packet Size Setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn pks(&mut self) -> PKS_W<EP1C_SPEC> {
        PKS_W::new(self, 0)
    }
    #[doc = "Bit 9 - Endpoint Stall Setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn stal(&mut self) -> STAL_W<EP1C_SPEC> {
        STAL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Null Automatic Transfer Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn nule(&mut self) -> NULE_W<EP1C_SPEC> {
        NULE_W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Automatic Transfer Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<EP1C_SPEC> {
        DMAE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint Transfer Direction Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<EP1C_SPEC> {
        DIR_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Endpoint Transfer Type Select bits"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<EP1C_SPEC> {
        TYPE_W::new(self, 13)
    }
    #[doc = "Bit 15 - Endpoint Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<EP1C_SPEC> {
        EPEN_W::new(self, 15)
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
#[doc = "EP1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP1C_SPEC;
impl crate::RegisterSpec for EP1C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep1c::R`](R) reader structure"]
impl crate::Readable for EP1C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep1c::W`](W) writer structure"]
impl crate::Writable for EP1C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP1C to value 0x6100"]
impl crate::Resettable for EP1C_SPEC {
    const RESET_VALUE: u16 = 0x6100;
}
