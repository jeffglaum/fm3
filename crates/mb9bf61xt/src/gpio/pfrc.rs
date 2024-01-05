#[doc = "Register `PFRC` reader"]
pub type R = crate::R<PFRC_SPEC>;
#[doc = "Register `PFRC` writer"]
pub type W = crate::W<PFRC_SPEC>;
#[doc = "Field `PC0` reader - Bit0 of PFRC"]
pub type PC0_R = crate::BitReader;
#[doc = "Field `PC0` writer - Bit0 of PFRC"]
pub type PC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC1` reader - Bit1 of PFRC"]
pub type PC1_R = crate::BitReader;
#[doc = "Field `PC1` writer - Bit1 of PFRC"]
pub type PC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2` reader - Bit2 of PFRC"]
pub type PC2_R = crate::BitReader;
#[doc = "Field `PC2` writer - Bit2 of PFRC"]
pub type PC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3` reader - Bit3 of PFRC"]
pub type PC3_R = crate::BitReader;
#[doc = "Field `PC3` writer - Bit3 of PFRC"]
pub type PC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4` reader - Bit4 of PFRC"]
pub type PC4_R = crate::BitReader;
#[doc = "Field `PC4` writer - Bit4 of PFRC"]
pub type PC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5` reader - Bit5 of PFRC"]
pub type PC5_R = crate::BitReader;
#[doc = "Field `PC5` writer - Bit5 of PFRC"]
pub type PC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6` reader - Bit6 of PFRC"]
pub type PC6_R = crate::BitReader;
#[doc = "Field `PC6` writer - Bit6 of PFRC"]
pub type PC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7` reader - Bit7 of PFRC"]
pub type PC7_R = crate::BitReader;
#[doc = "Field `PC7` writer - Bit7 of PFRC"]
pub type PC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC8` reader - Bit8 of PFRC"]
pub type PC8_R = crate::BitReader;
#[doc = "Field `PC8` writer - Bit8 of PFRC"]
pub type PC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC9` reader - Bit9 of PFRC"]
pub type PC9_R = crate::BitReader;
#[doc = "Field `PC9` writer - Bit9 of PFRC"]
pub type PC9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCA` reader - Bit10 of PFRC"]
pub type PCA_R = crate::BitReader;
#[doc = "Field `PCA` writer - Bit10 of PFRC"]
pub type PCA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCB` reader - Bit11 of PFRC"]
pub type PCB_R = crate::BitReader;
#[doc = "Field `PCB` writer - Bit11 of PFRC"]
pub type PCB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCC` reader - Bit12 of PFRC"]
pub type PCC_R = crate::BitReader;
#[doc = "Field `PCC` writer - Bit12 of PFRC"]
pub type PCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCD` reader - Bit13 of PFRC"]
pub type PCD_R = crate::BitReader;
#[doc = "Field `PCD` writer - Bit13 of PFRC"]
pub type PCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Bit14 of PFRC"]
pub type PCE_R = crate::BitReader;
#[doc = "Field `PCE` writer - Bit14 of PFRC"]
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Bit15 of PFRC"]
pub type PCF_R = crate::BitReader;
#[doc = "Field `PCF` writer - Bit15 of PFRC"]
pub type PCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFRC"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRC"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRC"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFRC"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFRC"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFRC"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFRC"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFRC"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFRC"]
    #[inline(always)]
    pub fn pc8(&self) -> PC8_R {
        PC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFRC"]
    #[inline(always)]
    pub fn pc9(&self) -> PC9_R {
        PC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of PFRC"]
    #[inline(always)]
    pub fn pca(&self) -> PCA_R {
        PCA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of PFRC"]
    #[inline(always)]
    pub fn pcb(&self) -> PCB_R {
        PCB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of PFRC"]
    #[inline(always)]
    pub fn pcc(&self) -> PCC_R {
        PCC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of PFRC"]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit14 of PFRC"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit15 of PFRC"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> PC0_W<PFRC_SPEC> {
        PC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> PC1_W<PFRC_SPEC> {
        PC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> PC2_W<PFRC_SPEC> {
        PC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> PC3_W<PFRC_SPEC> {
        PC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc4(&mut self) -> PC4_W<PFRC_SPEC> {
        PC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> PC5_W<PFRC_SPEC> {
        PC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<PFRC_SPEC> {
        PC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<PFRC_SPEC> {
        PC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc8(&mut self) -> PC8_W<PFRC_SPEC> {
        PC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pc9(&mut self) -> PC9_W<PFRC_SPEC> {
        PC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pca(&mut self) -> PCA_W<PFRC_SPEC> {
        PCA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pcb(&mut self) -> PCB_W<PFRC_SPEC> {
        PCB_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pcc(&mut self) -> PCC_W<PFRC_SPEC> {
        PCC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pcd(&mut self) -> PCD_W<PFRC_SPEC> {
        PCD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit14 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<PFRC_SPEC> {
        PCE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bit15 of PFRC"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<PFRC_SPEC> {
        PCF_W::new(self, 15)
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
#[doc = "Port function setting register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRC_SPEC;
impl crate::RegisterSpec for PFRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfrc::R`](R) reader structure"]
impl crate::Readable for PFRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfrc::W`](W) writer structure"]
impl crate::Writable for PFRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRC to value 0"]
impl crate::Resettable for PFRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
