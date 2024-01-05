#[doc = "Register `DQESEL` reader"]
pub type R = crate::R<DQESEL_SPEC>;
#[doc = "Register `DQESEL` writer"]
pub type W = crate::W<DQESEL_SPEC>;
#[doc = "Field `ESEL10` reader - Connect USB-ch1 to IDREQ \\[10\\]"]
pub type ESEL10_R = crate::FieldReader;
#[doc = "Field `ESEL10` writer - Connect USB-ch1 to IDREQ \\[10\\]"]
pub type ESEL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL11` reader - Connect USB-ch1 to IDREQ \\[11\\]"]
pub type ESEL11_R = crate::FieldReader;
#[doc = "Field `ESEL11` writer - Connect USB-ch1 to IDREQ \\[11\\]"]
pub type ESEL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL24` reader - Connect USB-ch1 to IDREQ \\[24\\]"]
pub type ESEL24_R = crate::FieldReader;
#[doc = "Field `ESEL24` writer - Connect USB-ch1 to IDREQ \\[24\\]"]
pub type ESEL24_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL25` reader - Connect USB-ch1 to IDREQ \\[25\\]"]
pub type ESEL25_R = crate::FieldReader;
#[doc = "Field `ESEL25` writer - Connect USB-ch1 to IDREQ \\[25\\]"]
pub type ESEL25_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL26` reader - Connect USB-ch1 to IDREQ \\[26\\]"]
pub type ESEL26_R = crate::FieldReader;
#[doc = "Field `ESEL26` writer - Connect USB-ch1 to IDREQ \\[26\\]"]
pub type ESEL26_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL27` reader - Connect USB-ch1 to IDREQ \\[27\\]"]
pub type ESEL27_R = crate::FieldReader;
#[doc = "Field `ESEL27` writer - Connect USB-ch1 to IDREQ \\[27\\]"]
pub type ESEL27_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL30` reader - Connect USB-ch1 to IDREQ \\[30\\]"]
pub type ESEL30_R = crate::FieldReader;
#[doc = "Field `ESEL30` writer - Connect USB-ch1 to IDREQ \\[30\\]"]
pub type ESEL30_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESEL31` reader - Connect USB-ch1 to IDREQ \\[31\\]"]
pub type ESEL31_R = crate::FieldReader;
#[doc = "Field `ESEL31` writer - Connect USB-ch1 to IDREQ \\[31\\]"]
pub type ESEL31_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Connect USB-ch1 to IDREQ \\[10\\]"]
    #[inline(always)]
    pub fn esel10(&self) -> ESEL10_R {
        ESEL10_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Connect USB-ch1 to IDREQ \\[11\\]"]
    #[inline(always)]
    pub fn esel11(&self) -> ESEL11_R {
        ESEL11_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Connect USB-ch1 to IDREQ \\[24\\]"]
    #[inline(always)]
    pub fn esel24(&self) -> ESEL24_R {
        ESEL24_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Connect USB-ch1 to IDREQ \\[25\\]"]
    #[inline(always)]
    pub fn esel25(&self) -> ESEL25_R {
        ESEL25_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Connect USB-ch1 to IDREQ \\[26\\]"]
    #[inline(always)]
    pub fn esel26(&self) -> ESEL26_R {
        ESEL26_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Connect USB-ch1 to IDREQ \\[27\\]"]
    #[inline(always)]
    pub fn esel27(&self) -> ESEL27_R {
        ESEL27_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Connect USB-ch1 to IDREQ \\[30\\]"]
    #[inline(always)]
    pub fn esel30(&self) -> ESEL30_R {
        ESEL30_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Connect USB-ch1 to IDREQ \\[31\\]"]
    #[inline(always)]
    pub fn esel31(&self) -> ESEL31_R {
        ESEL31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Connect USB-ch1 to IDREQ \\[10\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel10(&mut self) -> ESEL10_W<DQESEL_SPEC> {
        ESEL10_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Connect USB-ch1 to IDREQ \\[11\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel11(&mut self) -> ESEL11_W<DQESEL_SPEC> {
        ESEL11_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Connect USB-ch1 to IDREQ \\[24\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel24(&mut self) -> ESEL24_W<DQESEL_SPEC> {
        ESEL24_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Connect USB-ch1 to IDREQ \\[25\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel25(&mut self) -> ESEL25_W<DQESEL_SPEC> {
        ESEL25_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Connect USB-ch1 to IDREQ \\[26\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel26(&mut self) -> ESEL26_W<DQESEL_SPEC> {
        ESEL26_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Connect USB-ch1 to IDREQ \\[27\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel27(&mut self) -> ESEL27_W<DQESEL_SPEC> {
        ESEL27_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Connect USB-ch1 to IDREQ \\[30\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel30(&mut self) -> ESEL30_W<DQESEL_SPEC> {
        ESEL30_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Connect USB-ch1 to IDREQ \\[31\\]"]
    #[inline(always)]
    #[must_use]
    pub fn esel31(&mut self) -> ESEL31_W<DQESEL_SPEC> {
        ESEL31_W::new(self, 28)
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
#[doc = "DMA Request Extended Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dqesel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dqesel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DQESEL_SPEC;
impl crate::RegisterSpec for DQESEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dqesel::R`](R) reader structure"]
impl crate::Readable for DQESEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dqesel::W`](W) writer structure"]
impl crate::Writable for DQESEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DQESEL to value 0"]
impl crate::Resettable for DQESEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
