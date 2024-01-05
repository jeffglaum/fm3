#[doc = "Register `TSCR` reader"]
pub type R = crate::R<TSCR_SPEC>;
#[doc = "Register `TSCR` writer"]
pub type W = crate::W<TSCR_SPEC>;
#[doc = "Field `TSE` reader - Time Stamp Enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - Time Stamp Enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCU` reader - Time Stamp Fine or Coarse Update"]
pub type TFCU_R = crate::BitReader;
#[doc = "Field `TFCU` writer - Time Stamp Fine or Coarse Update"]
pub type TFCU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSI` reader - Time Stamp Initialize"]
pub type TSI_R = crate::BitReader;
#[doc = "Field `TSI` writer - Time Stamp Initialize"]
pub type TSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSU` reader - Time Stamp Update"]
pub type TSU_R = crate::BitReader;
#[doc = "Field `TSU` writer - Time Stamp Update"]
pub type TSU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TITE` reader - Time Stamp Interrupt Trigger Enable"]
pub type TITE_R = crate::BitReader;
#[doc = "Field `TITE` writer - Time Stamp Interrupt Trigger Enable"]
pub type TITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARU` reader - Addend Register Update"]
pub type TARU_R = crate::BitReader;
#[doc = "Field `TARU` writer - Addend Register Update"]
pub type TARU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEA` reader - Enable Time Stamp for All Frames"]
pub type TSEA_R = crate::BitReader;
#[doc = "Field `TSEA` writer - Enable Time Stamp for All Frames"]
pub type TSEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDB` reader - Time Stamp Digital or Binary rollover control"]
pub type TSDB_R = crate::BitReader;
#[doc = "Field `TSDB` writer - Time Stamp Digital or Binary rollover control"]
pub type TSDB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSV2E` reader - Enable PTP packet snooping for version 2 format"]
pub type TSV2E_R = crate::BitReader;
#[doc = "Field `TSV2E` writer - Enable PTP packet snooping for version 2 format"]
pub type TSV2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TETSP` reader - Enable Time Stamp Snapshot for PTP over Ethernet frames"]
pub type TETSP_R = crate::BitReader;
#[doc = "Field `TETSP` writer - Enable Time Stamp Snapshot for PTP over Ethernet frames"]
pub type TETSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIP6E` reader - Enable Time Stamp Snapshot for IPv6 frames"]
pub type TSIP6E_R = crate::BitReader;
#[doc = "Field `TSIP6E` writer - Enable Time Stamp Snapshot for IPv6 frames"]
pub type TSIP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIP4E` reader - Enable Time Stamp Snapshot for IPv4 frames"]
pub type TSIP4E_R = crate::BitReader;
#[doc = "Field `TSIP4E` writer - Enable Time Stamp Snapshot for IPv4 frames"]
pub type TSIP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TETSEM` reader - Enable Time Stamp Snapshot for Event Messages"]
pub type TETSEM_R = crate::BitReader;
#[doc = "Field `TETSEM` writer - Enable Time Stamp Snapshot for Event Messages"]
pub type TETSEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMRM` reader - Enable Snapshot for Messages Relevant to Master"]
pub type TSMRM_R = crate::BitReader;
#[doc = "Field `TSMRM` writer - Enable Snapshot for Messages Relevant to Master"]
pub type TSMRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSPS` reader - SelectPTP packets for taking snapshots"]
pub type TSPS_R = crate::FieldReader;
#[doc = "Field `TSPS` writer - SelectPTP packets for taking snapshots"]
pub type TSPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMF` reader - Enable MAC address for PTP frame filtering"]
pub type TSENMF_R = crate::BitReader;
#[doc = "Field `TSENMF` writer - Enable MAC address for PTP frame filtering"]
pub type TSENMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear"]
pub type ATSFC_R = crate::BitReader;
#[doc = "Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear"]
pub type ATSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Time Stamp Enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Stamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tfcu(&self) -> TFCU_R {
        TFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Stamp Initialize"]
    #[inline(always)]
    pub fn tsi(&self) -> TSI_R {
        TSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Stamp Update"]
    #[inline(always)]
    pub fn tsu(&self) -> TSU_R {
        TSU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Stamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tite(&self) -> TITE_R {
        TITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn taru(&self) -> TARU_R {
        TARU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Time Stamp for All Frames"]
    #[inline(always)]
    pub fn tsea(&self) -> TSEA_R {
        TSEA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Time Stamp Digital or Binary rollover control"]
    #[inline(always)]
    pub fn tsdb(&self) -> TSDB_R {
        TSDB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet snooping for version 2 format"]
    #[inline(always)]
    pub fn tsv2e(&self) -> TSV2E_R {
        TSV2E_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Time Stamp Snapshot for PTP over Ethernet frames"]
    #[inline(always)]
    pub fn tetsp(&self) -> TETSP_R {
        TETSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Time Stamp Snapshot for IPv6 frames"]
    #[inline(always)]
    pub fn tsip6e(&self) -> TSIP6E_R {
        TSIP6E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Time Stamp Snapshot for IPv4 frames"]
    #[inline(always)]
    pub fn tsip4e(&self) -> TSIP4E_R {
        TSIP4E_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Time Stamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tetsem(&self) -> TETSEM_R {
        TETSEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmrm(&self) -> TSMRM_R {
        TSMRM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - SelectPTP packets for taking snapshots"]
    #[inline(always)]
    pub fn tsps(&self) -> TSPS_R {
        TSPS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering"]
    #[inline(always)]
    pub fn tsenmf(&self) -> TSENMF_R {
        TSENMF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Stamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<TSCR_SPEC> {
        TSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Time Stamp Fine or Coarse Update"]
    #[inline(always)]
    #[must_use]
    pub fn tfcu(&mut self) -> TFCU_W<TSCR_SPEC> {
        TFCU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Time Stamp Initialize"]
    #[inline(always)]
    #[must_use]
    pub fn tsi(&mut self) -> TSI_W<TSCR_SPEC> {
        TSI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Time Stamp Update"]
    #[inline(always)]
    #[must_use]
    pub fn tsu(&mut self) -> TSU_W<TSCR_SPEC> {
        TSU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Time Stamp Interrupt Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tite(&mut self) -> TITE_W<TSCR_SPEC> {
        TITE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    #[must_use]
    pub fn taru(&mut self) -> TARU_W<TSCR_SPEC> {
        TARU_W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Time Stamp for All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsea(&mut self) -> TSEA_W<TSCR_SPEC> {
        TSEA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Time Stamp Digital or Binary rollover control"]
    #[inline(always)]
    #[must_use]
    pub fn tsdb(&mut self) -> TSDB_W<TSCR_SPEC> {
        TSDB_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP packet snooping for version 2 format"]
    #[inline(always)]
    #[must_use]
    pub fn tsv2e(&mut self) -> TSV2E_W<TSCR_SPEC> {
        TSV2E_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Time Stamp Snapshot for PTP over Ethernet frames"]
    #[inline(always)]
    #[must_use]
    pub fn tetsp(&mut self) -> TETSP_W<TSCR_SPEC> {
        TETSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Time Stamp Snapshot for IPv6 frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsip6e(&mut self) -> TSIP6E_W<TSCR_SPEC> {
        TSIP6E_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Time Stamp Snapshot for IPv4 frames"]
    #[inline(always)]
    #[must_use]
    pub fn tsip4e(&mut self) -> TSIP4E_W<TSCR_SPEC> {
        TSIP4E_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Time Stamp Snapshot for Event Messages"]
    #[inline(always)]
    #[must_use]
    pub fn tetsem(&mut self) -> TETSEM_W<TSCR_SPEC> {
        TETSEM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    #[must_use]
    pub fn tsmrm(&mut self) -> TSMRM_W<TSCR_SPEC> {
        TSMRM_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - SelectPTP packets for taking snapshots"]
    #[inline(always)]
    #[must_use]
    pub fn tsps(&mut self) -> TSPS_W<TSCR_SPEC> {
        TSPS_W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering"]
    #[inline(always)]
    #[must_use]
    pub fn tsenmf(&mut self) -> TSENMF_W<TSCR_SPEC> {
        TSENMF_W::new(self, 18)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    #[must_use]
    pub fn atsfc(&mut self) -> ATSFC_W<TSCR_SPEC> {
        ATSFC_W::new(self, 24)
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
#[doc = "Time Stamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCR_SPEC;
impl crate::RegisterSpec for TSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscr::R`](R) reader structure"]
impl crate::Readable for TSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscr::W`](W) writer structure"]
impl crate::Writable for TSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCR to value 0x2000"]
impl crate::Resettable for TSCR_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
