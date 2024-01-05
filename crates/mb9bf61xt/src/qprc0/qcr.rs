#[doc = "Register `QCR` reader"]
pub type R = crate::R<QCR_SPEC>;
#[doc = "Register `QCR` writer"]
pub type W = crate::W<QCR_SPEC>;
#[doc = "Field `PCM` reader - Position counter mode bits"]
pub type PCM_R = crate::FieldReader;
#[doc = "Field `PCM` writer - Position counter mode bits"]
pub type PCM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RCM` reader - Revolution counter mode bits"]
pub type RCM_R = crate::FieldReader;
#[doc = "Field `RCM` writer - Revolution counter mode bits"]
pub type RCM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSTP` reader - Position counter stop bit"]
pub type PSTP_R = crate::BitReader;
#[doc = "Field `PSTP` writer - Position counter stop bit"]
pub type PSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGSC` reader - Count clear or gate selection bit"]
pub type CGSC_R = crate::BitReader;
#[doc = "Field `CGSC` writer - Count clear or gate selection bit"]
pub type CGSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSEL` reader - Register function selection bit"]
pub type RSEL_R = crate::BitReader;
#[doc = "Field `RSEL` writer - Register function selection bit"]
pub type RSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - Swap bit"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap bit"]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRM` reader - Position counter reset mask bits"]
pub type PCRM_R = crate::FieldReader;
#[doc = "Field `PCRM` writer - Position counter reset mask bits"]
pub type PCRM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AES` reader - AIN detection edge selection bits"]
pub type AES_R = crate::FieldReader;
#[doc = "Field `AES` writer - AIN detection edge selection bits"]
pub type AES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BES` reader - BIN detection edge selection bits"]
pub type BES_R = crate::FieldReader;
#[doc = "Field `BES` writer - BIN detection edge selection bits"]
pub type BES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CGE` reader - Detection edge selection bits"]
pub type CGE_R = crate::FieldReader;
#[doc = "Field `CGE` writer - Detection edge selection bits"]
pub type CGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Position counter mode bits"]
    #[inline(always)]
    pub fn pcm(&self) -> PCM_R {
        PCM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Revolution counter mode bits"]
    #[inline(always)]
    pub fn rcm(&self) -> RCM_R {
        RCM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Position counter stop bit"]
    #[inline(always)]
    pub fn pstp(&self) -> PSTP_R {
        PSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Count clear or gate selection bit"]
    #[inline(always)]
    pub fn cgsc(&self) -> CGSC_R {
        CGSC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Register function selection bit"]
    #[inline(always)]
    pub fn rsel(&self) -> RSEL_R {
        RSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Swap bit"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Position counter reset mask bits"]
    #[inline(always)]
    pub fn pcrm(&self) -> PCRM_R {
        PCRM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - AIN detection edge selection bits"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BIN detection edge selection bits"]
    #[inline(always)]
    pub fn bes(&self) -> BES_R {
        BES_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Detection edge selection bits"]
    #[inline(always)]
    pub fn cge(&self) -> CGE_R {
        CGE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Position counter mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn pcm(&mut self) -> PCM_W<QCR_SPEC> {
        PCM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Revolution counter mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn rcm(&mut self) -> RCM_W<QCR_SPEC> {
        RCM_W::new(self, 2)
    }
    #[doc = "Bit 4 - Position counter stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn pstp(&mut self) -> PSTP_W<QCR_SPEC> {
        PSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Count clear or gate selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn cgsc(&mut self) -> CGSC_W<QCR_SPEC> {
        CGSC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Register function selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn rsel(&mut self) -> RSEL_W<QCR_SPEC> {
        RSEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Swap bit"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<QCR_SPEC> {
        SWAP_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Position counter reset mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn pcrm(&mut self) -> PCRM_W<QCR_SPEC> {
        PCRM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - AIN detection edge selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<QCR_SPEC> {
        AES_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - BIN detection edge selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn bes(&mut self) -> BES_W<QCR_SPEC> {
        BES_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Detection edge selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn cge(&mut self) -> CGE_W<QCR_SPEC> {
        CGE_W::new(self, 14)
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
#[doc = "QPRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QCR_SPEC;
impl crate::RegisterSpec for QCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qcr::R`](R) reader structure"]
impl crate::Readable for QCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qcr::W`](W) writer structure"]
impl crate::Writable for QCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets QCR to value 0"]
impl crate::Resettable for QCR_SPEC {
    const RESET_VALUE: u16 = 0;
}
