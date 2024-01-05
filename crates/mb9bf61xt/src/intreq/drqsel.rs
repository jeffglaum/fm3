#[doc = "Register `DRQSEL` reader"]
pub type R = crate::R<DRQSEL_SPEC>;
#[doc = "Register `DRQSEL` writer"]
pub type W = crate::W<DRQSEL_SPEC>;
#[doc = "Field `USBEP1` reader - The EP1 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP1_R = crate::BitReader;
#[doc = "Field `USBEP1` writer - The EP1 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEP2` reader - The EP2 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP2_R = crate::BitReader;
#[doc = "Field `USBEP2` writer - The EP2 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEP3` reader - The EP3 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP3_R = crate::BitReader;
#[doc = "Field `USBEP3` writer - The EP3 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEP4` reader - The EP4 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP4_R = crate::BitReader;
#[doc = "Field `USBEP4` writer - The EP4 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEP5` reader - The EP5 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP5_R = crate::BitReader;
#[doc = "Field `USBEP5` writer - The EP5 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
pub type USBEP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSCAN0` reader - The scan conversion interrupt signal of the A/D converter unit 0 is output as a transfer request to the DMAC."]
pub type ADCSCAN0_R = crate::BitReader;
#[doc = "Field `ADCSCAN0` writer - The scan conversion interrupt signal of the A/D converter unit 0 is output as a transfer request to the DMAC."]
pub type ADCSCAN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSCAN1` reader - The scan conversion interrupt signal of the A/D converter unit 1 is output as a transfer request to the DMAC."]
pub type ADCSCAN1_R = crate::BitReader;
#[doc = "Field `ADCSCAN1` writer - The scan conversion interrupt signal of the A/D converter unit 1 is output as a transfer request to the DMAC."]
pub type ADCSCAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSCAN2` reader - The scan conversion interrupt signal of the A/D converter unit 2 is output as a transfer request to the DMAC."]
pub type ADCSCAN2_R = crate::BitReader;
#[doc = "Field `ADCSCAN2` writer - The scan conversion interrupt signal of the A/D converter unit 2 is output as a transfer request to the DMAC."]
pub type ADCSCAN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ0BT0` reader - The IRQ0 interrupt signal of the base timer ch.0 is output as a transfer request to the DMAC."]
pub type IRQ0BT0_R = crate::BitReader;
#[doc = "Field `IRQ0BT0` writer - The IRQ0 interrupt signal of the base timer ch.0 is output as a transfer request to the DMAC."]
pub type IRQ0BT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ0BT2` reader - The IRQ0 interrupt signal of the base timer ch.3 is output as a transfer request to the DMAC."]
pub type IRQ0BT2_R = crate::BitReader;
#[doc = "Field `IRQ0BT2` writer - The IRQ0 interrupt signal of the base timer ch.3 is output as a transfer request to the DMAC."]
pub type IRQ0BT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ0BT4` reader - The IRQ0 interrupt signal of the base timer ch.4 is output as a transfer request to the DMAC."]
pub type IRQ0BT4_R = crate::BitReader;
#[doc = "Field `IRQ0BT4` writer - The IRQ0 interrupt signal of the base timer ch.4 is output as a transfer request to the DMAC."]
pub type IRQ0BT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ0BT6` reader - The IRQ0 interrupt signal of the base timer ch.6 is output as a transfer request to the DMAC."]
pub type IRQ0BT6_R = crate::BitReader;
#[doc = "Field `IRQ0BT6` writer - The IRQ0 interrupt signal of the base timer ch.6 is output as a transfer request to the DMAC."]
pub type IRQ0BT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS0RX` reader - The reception interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
pub type MFS0RX_R = crate::BitReader;
#[doc = "Field `MFS0RX` writer - The reception interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
pub type MFS0RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS0TX` reader - The transmission interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
pub type MFS0TX_R = crate::BitReader;
#[doc = "Field `MFS0TX` writer - The transmission interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
pub type MFS0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS1RX` reader - The reception interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
pub type MFS1RX_R = crate::BitReader;
#[doc = "Field `MFS1RX` writer - The reception interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
pub type MFS1RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS1TX` reader - The transmission interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
pub type MFS1TX_R = crate::BitReader;
#[doc = "Field `MFS1TX` writer - The transmission interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
pub type MFS1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS2RX` reader - The reception interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
pub type MFS2RX_R = crate::BitReader;
#[doc = "Field `MFS2RX` writer - The reception interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
pub type MFS2RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS2TX` reader - The transmission interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
pub type MFS2TX_R = crate::BitReader;
#[doc = "Field `MFS2TX` writer - The transmission interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
pub type MFS2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS3RX` reader - The reception interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
pub type MFS3RX_R = crate::BitReader;
#[doc = "Field `MFS3RX` writer - The reception interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
pub type MFS3RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS3TX` reader - The transmission interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
pub type MFS3TX_R = crate::BitReader;
#[doc = "Field `MFS3TX` writer - The transmission interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
pub type MFS3TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS4RX` reader - The reception interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
pub type MFS4RX_R = crate::BitReader;
#[doc = "Field `MFS4RX` writer - The reception interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
pub type MFS4RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS4TX` reader - The transmission interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
pub type MFS4TX_R = crate::BitReader;
#[doc = "Field `MFS4TX` writer - The transmission interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
pub type MFS4TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS5RX` reader - The reception interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
pub type MFS5RX_R = crate::BitReader;
#[doc = "Field `MFS5RX` writer - The reception interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
pub type MFS5RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS5TX` reader - The transmission interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
pub type MFS5TX_R = crate::BitReader;
#[doc = "Field `MFS5TX` writer - The transmission interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
pub type MFS5TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS6RX` reader - The reception interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
pub type MFS6RX_R = crate::BitReader;
#[doc = "Field `MFS6RX` writer - The reception interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
pub type MFS6RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS6TX` reader - The transmission interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
pub type MFS6TX_R = crate::BitReader;
#[doc = "Field `MFS6TX` writer - The transmission interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
pub type MFS6TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS7RX` reader - The reception interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
pub type MFS7RX_R = crate::BitReader;
#[doc = "Field `MFS7RX` writer - The reception interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
pub type MFS7RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFS7TX` reader - The transmission interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
pub type MFS7TX_R = crate::BitReader;
#[doc = "Field `MFS7TX` writer - The transmission interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
pub type MFS7TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT0` reader - The interrupt signal of the external interrupt ch.0 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT0_R = crate::BitReader;
#[doc = "Field `EXINT0` writer - The interrupt signal of the external interrupt ch.0 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT1` reader - The interrupt signal of the external interrupt ch.1 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT1_R = crate::BitReader;
#[doc = "Field `EXINT1` writer - The interrupt signal of the external interrupt ch.1 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT2` reader - The interrupt signal of the external interrupt ch.2 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT2_R = crate::BitReader;
#[doc = "Field `EXINT2` writer - The interrupt signal of the external interrupt ch.2 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT3` reader - The interrupt signal of the external interrupt ch.3 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT3_R = crate::BitReader;
#[doc = "Field `EXINT3` writer - The interrupt signal of the external interrupt ch.3 is output as a transfer request to the DMAC (including extension)."]
pub type EXINT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The EP1 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn usbep1(&self) -> USBEP1_R {
        USBEP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The EP2 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn usbep2(&self) -> USBEP2_R {
        USBEP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The EP3 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn usbep3(&self) -> USBEP3_R {
        USBEP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The EP4 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn usbep4(&self) -> USBEP4_R {
        USBEP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The EP5 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn usbep5(&self) -> USBEP5_R {
        USBEP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The scan conversion interrupt signal of the A/D converter unit 0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn adcscan0(&self) -> ADCSCAN0_R {
        ADCSCAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The scan conversion interrupt signal of the A/D converter unit 1 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn adcscan1(&self) -> ADCSCAN1_R {
        ADCSCAN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The scan conversion interrupt signal of the A/D converter unit 2 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn adcscan2(&self) -> ADCSCAN2_R {
        ADCSCAN2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The IRQ0 interrupt signal of the base timer ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn irq0bt0(&self) -> IRQ0BT0_R {
        IRQ0BT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The IRQ0 interrupt signal of the base timer ch.3 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn irq0bt2(&self) -> IRQ0BT2_R {
        IRQ0BT2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The IRQ0 interrupt signal of the base timer ch.4 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn irq0bt4(&self) -> IRQ0BT4_R {
        IRQ0BT4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The IRQ0 interrupt signal of the base timer ch.6 is output as a transfer request to the DMAC."]
    #[inline(always)]
    pub fn irq0bt6(&self) -> IRQ0BT6_R {
        IRQ0BT6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The reception interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs0rx(&self) -> MFS0RX_R {
        MFS0RX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The transmission interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs0tx(&self) -> MFS0TX_R {
        MFS0TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The reception interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs1rx(&self) -> MFS1RX_R {
        MFS1RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The transmission interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs1tx(&self) -> MFS1TX_R {
        MFS1TX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The reception interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs2rx(&self) -> MFS2RX_R {
        MFS2RX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The transmission interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs2tx(&self) -> MFS2TX_R {
        MFS2TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The reception interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs3rx(&self) -> MFS3RX_R {
        MFS3RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The transmission interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs3tx(&self) -> MFS3TX_R {
        MFS3TX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The reception interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs4rx(&self) -> MFS4RX_R {
        MFS4RX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The transmission interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs4tx(&self) -> MFS4TX_R {
        MFS4TX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The reception interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs5rx(&self) -> MFS5RX_R {
        MFS5RX_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The transmission interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs5tx(&self) -> MFS5TX_R {
        MFS5TX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The reception interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs6rx(&self) -> MFS6RX_R {
        MFS6RX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The transmission interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs6tx(&self) -> MFS6TX_R {
        MFS6TX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The reception interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs7rx(&self) -> MFS7RX_R {
        MFS7RX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The transmission interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn mfs7tx(&self) -> MFS7TX_R {
        MFS7TX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt signal of the external interrupt ch.0 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn exint0(&self) -> EXINT0_R {
        EXINT0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt signal of the external interrupt ch.1 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn exint1(&self) -> EXINT1_R {
        EXINT1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The interrupt signal of the external interrupt ch.2 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn exint2(&self) -> EXINT2_R {
        EXINT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The interrupt signal of the external interrupt ch.3 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    pub fn exint3(&self) -> EXINT3_R {
        EXINT3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The EP1 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn usbep1(&mut self) -> USBEP1_W<DRQSEL_SPEC> {
        USBEP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - The EP2 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn usbep2(&mut self) -> USBEP2_W<DRQSEL_SPEC> {
        USBEP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - The EP3 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn usbep3(&mut self) -> USBEP3_W<DRQSEL_SPEC> {
        USBEP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - The EP4 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn usbep4(&mut self) -> USBEP4_W<DRQSEL_SPEC> {
        USBEP4_W::new(self, 3)
    }
    #[doc = "Bit 4 - The EP5 DRQ interrupt signal of the USB ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn usbep5(&mut self) -> USBEP5_W<DRQSEL_SPEC> {
        USBEP5_W::new(self, 4)
    }
    #[doc = "Bit 5 - The scan conversion interrupt signal of the A/D converter unit 0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn adcscan0(&mut self) -> ADCSCAN0_W<DRQSEL_SPEC> {
        ADCSCAN0_W::new(self, 5)
    }
    #[doc = "Bit 6 - The scan conversion interrupt signal of the A/D converter unit 1 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn adcscan1(&mut self) -> ADCSCAN1_W<DRQSEL_SPEC> {
        ADCSCAN1_W::new(self, 6)
    }
    #[doc = "Bit 7 - The scan conversion interrupt signal of the A/D converter unit 2 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn adcscan2(&mut self) -> ADCSCAN2_W<DRQSEL_SPEC> {
        ADCSCAN2_W::new(self, 7)
    }
    #[doc = "Bit 8 - The IRQ0 interrupt signal of the base timer ch.0 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn irq0bt0(&mut self) -> IRQ0BT0_W<DRQSEL_SPEC> {
        IRQ0BT0_W::new(self, 8)
    }
    #[doc = "Bit 9 - The IRQ0 interrupt signal of the base timer ch.3 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn irq0bt2(&mut self) -> IRQ0BT2_W<DRQSEL_SPEC> {
        IRQ0BT2_W::new(self, 9)
    }
    #[doc = "Bit 10 - The IRQ0 interrupt signal of the base timer ch.4 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn irq0bt4(&mut self) -> IRQ0BT4_W<DRQSEL_SPEC> {
        IRQ0BT4_W::new(self, 10)
    }
    #[doc = "Bit 11 - The IRQ0 interrupt signal of the base timer ch.6 is output as a transfer request to the DMAC."]
    #[inline(always)]
    #[must_use]
    pub fn irq0bt6(&mut self) -> IRQ0BT6_W<DRQSEL_SPEC> {
        IRQ0BT6_W::new(self, 11)
    }
    #[doc = "Bit 12 - The reception interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs0rx(&mut self) -> MFS0RX_W<DRQSEL_SPEC> {
        MFS0RX_W::new(self, 12)
    }
    #[doc = "Bit 13 - The transmission interrupt signal of the MFS ch.0 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs0tx(&mut self) -> MFS0TX_W<DRQSEL_SPEC> {
        MFS0TX_W::new(self, 13)
    }
    #[doc = "Bit 14 - The reception interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs1rx(&mut self) -> MFS1RX_W<DRQSEL_SPEC> {
        MFS1RX_W::new(self, 14)
    }
    #[doc = "Bit 15 - The transmission interrupt signal of the MFS ch.1 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs1tx(&mut self) -> MFS1TX_W<DRQSEL_SPEC> {
        MFS1TX_W::new(self, 15)
    }
    #[doc = "Bit 16 - The reception interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs2rx(&mut self) -> MFS2RX_W<DRQSEL_SPEC> {
        MFS2RX_W::new(self, 16)
    }
    #[doc = "Bit 17 - The transmission interrupt signal of the MFS ch.2 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs2tx(&mut self) -> MFS2TX_W<DRQSEL_SPEC> {
        MFS2TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - The reception interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs3rx(&mut self) -> MFS3RX_W<DRQSEL_SPEC> {
        MFS3RX_W::new(self, 18)
    }
    #[doc = "Bit 19 - The transmission interrupt signal of the MFS ch.3 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs3tx(&mut self) -> MFS3TX_W<DRQSEL_SPEC> {
        MFS3TX_W::new(self, 19)
    }
    #[doc = "Bit 20 - The reception interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs4rx(&mut self) -> MFS4RX_W<DRQSEL_SPEC> {
        MFS4RX_W::new(self, 20)
    }
    #[doc = "Bit 21 - The transmission interrupt signal of the MFS ch.4 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs4tx(&mut self) -> MFS4TX_W<DRQSEL_SPEC> {
        MFS4TX_W::new(self, 21)
    }
    #[doc = "Bit 22 - The reception interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs5rx(&mut self) -> MFS5RX_W<DRQSEL_SPEC> {
        MFS5RX_W::new(self, 22)
    }
    #[doc = "Bit 23 - The transmission interrupt signal of the MFS ch.5 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs5tx(&mut self) -> MFS5TX_W<DRQSEL_SPEC> {
        MFS5TX_W::new(self, 23)
    }
    #[doc = "Bit 24 - The reception interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs6rx(&mut self) -> MFS6RX_W<DRQSEL_SPEC> {
        MFS6RX_W::new(self, 24)
    }
    #[doc = "Bit 25 - The transmission interrupt signal of the MFS ch.6 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs6tx(&mut self) -> MFS6TX_W<DRQSEL_SPEC> {
        MFS6TX_W::new(self, 25)
    }
    #[doc = "Bit 26 - The reception interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs7rx(&mut self) -> MFS7RX_W<DRQSEL_SPEC> {
        MFS7RX_W::new(self, 26)
    }
    #[doc = "Bit 27 - The transmission interrupt signal of the MFS ch.7 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn mfs7tx(&mut self) -> MFS7TX_W<DRQSEL_SPEC> {
        MFS7TX_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt signal of the external interrupt ch.0 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn exint0(&mut self) -> EXINT0_W<DRQSEL_SPEC> {
        EXINT0_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt signal of the external interrupt ch.1 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn exint1(&mut self) -> EXINT1_W<DRQSEL_SPEC> {
        EXINT1_W::new(self, 29)
    }
    #[doc = "Bit 30 - The interrupt signal of the external interrupt ch.2 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn exint2(&mut self) -> EXINT2_W<DRQSEL_SPEC> {
        EXINT2_W::new(self, 30)
    }
    #[doc = "Bit 31 - The interrupt signal of the external interrupt ch.3 is output as a transfer request to the DMAC (including extension)."]
    #[inline(always)]
    #[must_use]
    pub fn exint3(&mut self) -> EXINT3_W<DRQSEL_SPEC> {
        EXINT3_W::new(self, 31)
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
#[doc = "DMA Request Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drqsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drqsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRQSEL_SPEC;
impl crate::RegisterSpec for DRQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drqsel::R`](R) reader structure"]
impl crate::Readable for DRQSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drqsel::W`](W) writer structure"]
impl crate::Writable for DRQSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRQSEL to value 0"]
impl crate::Resettable for DRQSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
