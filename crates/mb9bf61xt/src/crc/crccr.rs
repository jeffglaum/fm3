#[doc = "Register `CRCCR` reader"]
pub type R = crate::R<CRCCR_SPEC>;
#[doc = "Register `CRCCR` writer"]
pub type W = crate::W<CRCCR_SPEC>;
#[doc = "Field `INIT` reader - CRC mode selection bit"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - CRC mode selection bit"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC32` reader - Byte-order setting bit"]
pub type CRC32_R = crate::BitReader;
#[doc = "Field `CRC32` writer - Byte-order setting bit"]
pub type CRC32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTLEND` reader - Bit-order setting bit"]
pub type LTLEND_R = crate::BitReader;
#[doc = "Field `LTLEND` writer - Bit-order setting bit"]
pub type LTLEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFST` reader - CRC result byte-order setting bit"]
pub type LSBFST_R = crate::BitReader;
#[doc = "Field `LSBFST` writer - CRC result byte-order setting bit"]
pub type LSBFST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLTE` reader - CRC result bit-order setting bit"]
pub type CRCLTE_R = crate::BitReader;
#[doc = "Field `CRCLTE` writer - CRC result bit-order setting bit"]
pub type CRCLTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLSF` reader - Final XOR control bit"]
pub type CRCLSF_R = crate::BitReader;
#[doc = "Field `CRCLSF` writer - Final XOR control bit"]
pub type CRCLSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FXOR` reader - Initialization bit"]
pub type FXOR_R = crate::BitReader;
#[doc = "Field `FXOR` writer - Initialization bit"]
pub type FXOR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC mode selection bit"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Byte-order setting bit"]
    #[inline(always)]
    pub fn crc32(&self) -> CRC32_R {
        CRC32_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit-order setting bit"]
    #[inline(always)]
    pub fn ltlend(&self) -> LTLEND_R {
        LTLEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CRC result byte-order setting bit"]
    #[inline(always)]
    pub fn lsbfst(&self) -> LSBFST_R {
        LSBFST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC result bit-order setting bit"]
    #[inline(always)]
    pub fn crclte(&self) -> CRCLTE_R {
        CRCLTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Final XOR control bit"]
    #[inline(always)]
    pub fn crclsf(&self) -> CRCLSF_R {
        CRCLSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization bit"]
    #[inline(always)]
    pub fn fxor(&self) -> FXOR_R {
        FXOR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<CRCCR_SPEC> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Byte-order setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn crc32(&mut self) -> CRC32_W<CRCCR_SPEC> {
        CRC32_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit-order setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn ltlend(&mut self) -> LTLEND_W<CRCCR_SPEC> {
        LTLEND_W::new(self, 2)
    }
    #[doc = "Bit 3 - CRC result byte-order setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfst(&mut self) -> LSBFST_W<CRCCR_SPEC> {
        LSBFST_W::new(self, 3)
    }
    #[doc = "Bit 4 - CRC result bit-order setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn crclte(&mut self) -> CRCLTE_W<CRCCR_SPEC> {
        CRCLTE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Final XOR control bit"]
    #[inline(always)]
    #[must_use]
    pub fn crclsf(&mut self) -> CRCLSF_W<CRCCR_SPEC> {
        CRCLSF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Initialization bit"]
    #[inline(always)]
    #[must_use]
    pub fn fxor(&mut self) -> FXOR_W<CRCCR_SPEC> {
        FXOR_W::new(self, 6)
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
#[doc = "CRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCCR_SPEC;
impl crate::RegisterSpec for CRCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crccr::R`](R) reader structure"]
impl crate::Readable for CRCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crccr::W`](W) writer structure"]
impl crate::Writable for CRCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CRCCR to value 0"]
impl crate::Resettable for CRCCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
