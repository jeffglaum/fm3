#[doc = "Register `IRQ46MON` reader"]
pub type R = crate::R<IRQ46MON_SPEC>;
#[doc = "Field `BTINT0` reader - IRQ0 interrupt request of base timer ch.8"]
pub type BTINT0_R = crate::BitReader;
#[doc = "Field `BTINT1` reader - IRQ1 interrupt request of base timer ch.8"]
pub type BTINT1_R = crate::BitReader;
#[doc = "Field `BTINT2` reader - IRQ0 interrupt request of base timer ch.9"]
pub type BTINT2_R = crate::BitReader;
#[doc = "Field `BTINT3` reader - IRQ1 interrupt request of base timer ch.9"]
pub type BTINT3_R = crate::BitReader;
#[doc = "Field `BTINT4` reader - IRQ0 interrupt request of base timer ch.10"]
pub type BTINT4_R = crate::BitReader;
#[doc = "Field `BTINT5` reader - IRQ1 interrupt request of base timer ch.10"]
pub type BTINT5_R = crate::BitReader;
#[doc = "Field `BTINT6` reader - IRQ0 interrupt request of base timer ch.11"]
pub type BTINT6_R = crate::BitReader;
#[doc = "Field `BTINT7` reader - IRQ1 interrupt request of base timer ch.11"]
pub type BTINT7_R = crate::BitReader;
#[doc = "Field `BTINT8` reader - IRQ0 interrupt request of base timer ch.12"]
pub type BTINT8_R = crate::BitReader;
#[doc = "Field `BTINT9` reader - IRQ1 interrupt request of base timer ch.12"]
pub type BTINT9_R = crate::BitReader;
#[doc = "Field `BTINT10` reader - IRQ0 interrupt request of base timer ch.13"]
pub type BTINT10_R = crate::BitReader;
#[doc = "Field `BTINT11` reader - IRQ1 interrupt request of base timer ch.13"]
pub type BTINT11_R = crate::BitReader;
#[doc = "Field `BTINT12` reader - IRQ0 interrupt request of base timer ch.14"]
pub type BTINT12_R = crate::BitReader;
#[doc = "Field `BTINT13` reader - IRQ1 interrupt request of base timer ch.14"]
pub type BTINT13_R = crate::BitReader;
#[doc = "Field `BTINT14` reader - IRQ0 interrupt request of base timer ch.15"]
pub type BTINT14_R = crate::BitReader;
#[doc = "Field `BTINT15` reader - IRQ1 interrupt request of base timer ch.15"]
pub type BTINT15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IRQ0 interrupt request of base timer ch.8"]
    #[inline(always)]
    pub fn btint0(&self) -> BTINT0_R {
        BTINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ1 interrupt request of base timer ch.8"]
    #[inline(always)]
    pub fn btint1(&self) -> BTINT1_R {
        BTINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ0 interrupt request of base timer ch.9"]
    #[inline(always)]
    pub fn btint2(&self) -> BTINT2_R {
        BTINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ1 interrupt request of base timer ch.9"]
    #[inline(always)]
    pub fn btint3(&self) -> BTINT3_R {
        BTINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ0 interrupt request of base timer ch.10"]
    #[inline(always)]
    pub fn btint4(&self) -> BTINT4_R {
        BTINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ1 interrupt request of base timer ch.10"]
    #[inline(always)]
    pub fn btint5(&self) -> BTINT5_R {
        BTINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ0 interrupt request of base timer ch.11"]
    #[inline(always)]
    pub fn btint6(&self) -> BTINT6_R {
        BTINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ1 interrupt request of base timer ch.11"]
    #[inline(always)]
    pub fn btint7(&self) -> BTINT7_R {
        BTINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRQ0 interrupt request of base timer ch.12"]
    #[inline(always)]
    pub fn btint8(&self) -> BTINT8_R {
        BTINT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IRQ1 interrupt request of base timer ch.12"]
    #[inline(always)]
    pub fn btint9(&self) -> BTINT9_R {
        BTINT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRQ0 interrupt request of base timer ch.13"]
    #[inline(always)]
    pub fn btint10(&self) -> BTINT10_R {
        BTINT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IRQ1 interrupt request of base timer ch.13"]
    #[inline(always)]
    pub fn btint11(&self) -> BTINT11_R {
        BTINT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IRQ0 interrupt request of base timer ch.14"]
    #[inline(always)]
    pub fn btint12(&self) -> BTINT12_R {
        BTINT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IRQ1 interrupt request of base timer ch.14"]
    #[inline(always)]
    pub fn btint13(&self) -> BTINT13_R {
        BTINT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IRQ0 interrupt request of base timer ch.15"]
    #[inline(always)]
    pub fn btint14(&self) -> BTINT14_R {
        BTINT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ1 interrupt request of base timer ch.15"]
    #[inline(always)]
    pub fn btint15(&self) -> BTINT15_R {
        BTINT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "IRQ46 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq46mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ46MON_SPEC;
impl crate::RegisterSpec for IRQ46MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq46mon::R`](R) reader structure"]
impl crate::Readable for IRQ46MON_SPEC {}
#[doc = "`reset()` method sets IRQ46MON to value 0"]
impl crate::Resettable for IRQ46MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
