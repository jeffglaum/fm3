#[doc = "Register `EPFR14` reader"]
pub type R = crate::R<EPFR14_SPEC>;
#[doc = "Register `EPFR14` writer"]
pub type W = crate::W<EPFR14_SPEC>;
#[doc = "Field `QAIN2S` reader - QDU-ch.2 AIN Input Pin bits"]
pub type QAIN2S_R = crate::FieldReader;
#[doc = "Field `QAIN2S` writer - QDU-ch.2 AIN Input Pin bits"]
pub type QAIN2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QBIN2S` reader - QDU-ch.2 BIN Input Pin bits"]
pub type QBIN2S_R = crate::FieldReader;
#[doc = "Field `QBIN2S` writer - QDU-ch.2 BIN Input Pin bits"]
pub type QBIN2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QZIN2S` reader - QDU-ch.2 ZIN Input Pin bits"]
pub type QZIN2S_R = crate::FieldReader;
#[doc = "Field `QZIN2S` writer - QDU-ch.2 ZIN Input Pin bits"]
pub type QZIN2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `E_TD0E` reader - \"E_TX00, E_TX01 Output Select bit \""]
pub type E_TD0E_R = crate::BitReader;
#[doc = "Field `E_TD0E` writer - \"E_TX00, E_TX01 Output Select bit \""]
pub type E_TD0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_TD1E` reader - \"E_TX02_TX10, E_TX03_TX11 Output Select bit \""]
pub type E_TD1E_R = crate::BitReader;
#[doc = "Field `E_TD1E` writer - \"E_TX02_TX10, E_TX03_TX11 Output Select bit \""]
pub type E_TD1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_TE0E` reader - E_TXEN0 Output Select bit"]
pub type E_TE0E_R = crate::BitReader;
#[doc = "Field `E_TE0E` writer - E_TXEN0 Output Select bit"]
pub type E_TE0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_TE1E` reader - E_TXER0_TXEN1 Output Select bit"]
pub type E_TE1E_R = crate::BitReader;
#[doc = "Field `E_TE1E` writer - E_TXER0_TXEN1 Output Select bit"]
pub type E_TE1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_MC0E` reader - E_MDC0 Output Select bit"]
pub type E_MC0E_R = crate::BitReader;
#[doc = "Field `E_MC0E` writer - E_MDC0 Output Select bit"]
pub type E_MC0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_MC1B` reader - E_MDC1 I/O Select bit"]
pub type E_MC1B_R = crate::BitReader;
#[doc = "Field `E_MC1B` writer - E_MDC1 I/O Select bit"]
pub type E_MC1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_MD0B` reader - E_MDO0 I/O Select bit"]
pub type E_MD0B_R = crate::BitReader;
#[doc = "Field `E_MD0B` writer - E_MDO0 I/O Select bit"]
pub type E_MD0B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_MD1B` reader - E_MDO1 I/O Select bit"]
pub type E_MD1B_R = crate::BitReader;
#[doc = "Field `E_MD1B` writer - E_MDO1 I/O Select bit"]
pub type E_MD1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_CKE` reader - E_COUT Output Select bit"]
pub type E_CKE_R = crate::BitReader;
#[doc = "Field `E_CKE` writer - E_COUT Output Select bit"]
pub type E_CKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_PSE` reader - PPS0_PPS1 Output Select bit for Ethernet"]
pub type E_PSE_R = crate::BitReader;
#[doc = "Field `E_PSE` writer - PPS0_PPS1 Output Select bit for Ethernet"]
pub type E_PSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E_SPLC` reader - Input cutoff Select bit in Standby of input Pin for Ethernet"]
pub type E_SPLC_R = crate::FieldReader;
#[doc = "Field `E_SPLC` writer - Input cutoff Select bit in Standby of input Pin for Ethernet"]
pub type E_SPLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - QDU-ch.2 AIN Input Pin bits"]
    #[inline(always)]
    pub fn qain2s(&self) -> QAIN2S_R {
        QAIN2S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - QDU-ch.2 BIN Input Pin bits"]
    #[inline(always)]
    pub fn qbin2s(&self) -> QBIN2S_R {
        QBIN2S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - QDU-ch.2 ZIN Input Pin bits"]
    #[inline(always)]
    pub fn qzin2s(&self) -> QZIN2S_R {
        QZIN2S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 18 - \"E_TX00, E_TX01 Output Select bit \""]
    #[inline(always)]
    pub fn e_td0e(&self) -> E_TD0E_R {
        E_TD0E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - \"E_TX02_TX10, E_TX03_TX11 Output Select bit \""]
    #[inline(always)]
    pub fn e_td1e(&self) -> E_TD1E_R {
        E_TD1E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - E_TXEN0 Output Select bit"]
    #[inline(always)]
    pub fn e_te0e(&self) -> E_TE0E_R {
        E_TE0E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - E_TXER0_TXEN1 Output Select bit"]
    #[inline(always)]
    pub fn e_te1e(&self) -> E_TE1E_R {
        E_TE1E_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - E_MDC0 Output Select bit"]
    #[inline(always)]
    pub fn e_mc0e(&self) -> E_MC0E_R {
        E_MC0E_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - E_MDC1 I/O Select bit"]
    #[inline(always)]
    pub fn e_mc1b(&self) -> E_MC1B_R {
        E_MC1B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - E_MDO0 I/O Select bit"]
    #[inline(always)]
    pub fn e_md0b(&self) -> E_MD0B_R {
        E_MD0B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - E_MDO1 I/O Select bit"]
    #[inline(always)]
    pub fn e_md1b(&self) -> E_MD1B_R {
        E_MD1B_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - E_COUT Output Select bit"]
    #[inline(always)]
    pub fn e_cke(&self) -> E_CKE_R {
        E_CKE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PPS0_PPS1 Output Select bit for Ethernet"]
    #[inline(always)]
    pub fn e_pse(&self) -> E_PSE_R {
        E_PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Input cutoff Select bit in Standby of input Pin for Ethernet"]
    #[inline(always)]
    pub fn e_splc(&self) -> E_SPLC_R {
        E_SPLC_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QDU-ch.2 AIN Input Pin bits"]
    #[inline(always)]
    #[must_use]
    pub fn qain2s(&mut self) -> QAIN2S_W<EPFR14_SPEC> {
        QAIN2S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - QDU-ch.2 BIN Input Pin bits"]
    #[inline(always)]
    #[must_use]
    pub fn qbin2s(&mut self) -> QBIN2S_W<EPFR14_SPEC> {
        QBIN2S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - QDU-ch.2 ZIN Input Pin bits"]
    #[inline(always)]
    #[must_use]
    pub fn qzin2s(&mut self) -> QZIN2S_W<EPFR14_SPEC> {
        QZIN2S_W::new(self, 4)
    }
    #[doc = "Bit 18 - \"E_TX00, E_TX01 Output Select bit \""]
    #[inline(always)]
    #[must_use]
    pub fn e_td0e(&mut self) -> E_TD0E_W<EPFR14_SPEC> {
        E_TD0E_W::new(self, 18)
    }
    #[doc = "Bit 19 - \"E_TX02_TX10, E_TX03_TX11 Output Select bit \""]
    #[inline(always)]
    #[must_use]
    pub fn e_td1e(&mut self) -> E_TD1E_W<EPFR14_SPEC> {
        E_TD1E_W::new(self, 19)
    }
    #[doc = "Bit 20 - E_TXEN0 Output Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_te0e(&mut self) -> E_TE0E_W<EPFR14_SPEC> {
        E_TE0E_W::new(self, 20)
    }
    #[doc = "Bit 21 - E_TXER0_TXEN1 Output Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_te1e(&mut self) -> E_TE1E_W<EPFR14_SPEC> {
        E_TE1E_W::new(self, 21)
    }
    #[doc = "Bit 22 - E_MDC0 Output Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_mc0e(&mut self) -> E_MC0E_W<EPFR14_SPEC> {
        E_MC0E_W::new(self, 22)
    }
    #[doc = "Bit 23 - E_MDC1 I/O Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_mc1b(&mut self) -> E_MC1B_W<EPFR14_SPEC> {
        E_MC1B_W::new(self, 23)
    }
    #[doc = "Bit 24 - E_MDO0 I/O Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_md0b(&mut self) -> E_MD0B_W<EPFR14_SPEC> {
        E_MD0B_W::new(self, 24)
    }
    #[doc = "Bit 25 - E_MDO1 I/O Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_md1b(&mut self) -> E_MD1B_W<EPFR14_SPEC> {
        E_MD1B_W::new(self, 25)
    }
    #[doc = "Bit 26 - E_COUT Output Select bit"]
    #[inline(always)]
    #[must_use]
    pub fn e_cke(&mut self) -> E_CKE_W<EPFR14_SPEC> {
        E_CKE_W::new(self, 26)
    }
    #[doc = "Bit 27 - PPS0_PPS1 Output Select bit for Ethernet"]
    #[inline(always)]
    #[must_use]
    pub fn e_pse(&mut self) -> E_PSE_W<EPFR14_SPEC> {
        E_PSE_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Input cutoff Select bit in Standby of input Pin for Ethernet"]
    #[inline(always)]
    #[must_use]
    pub fn e_splc(&mut self) -> E_SPLC_W<EPFR14_SPEC> {
        E_SPLC_W::new(self, 28)
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
#[doc = "Extended pin function setting register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR14_SPEC;
impl crate::RegisterSpec for EPFR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr14::R`](R) reader structure"]
impl crate::Readable for EPFR14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr14::W`](W) writer structure"]
impl crate::Writable for EPFR14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR14 to value 0"]
impl crate::Resettable for EPFR14_SPEC {
    const RESET_VALUE: u32 = 0;
}
