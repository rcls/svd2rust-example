#[doc = "Register `IPGR` reader"]
pub struct R(crate::R<IPGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPGR` writer"]
pub struct W(crate::W<IPGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IPGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBTOBINTEGAP2` reader - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub struct NBTOBINTEGAP2_R(crate::FieldReader<u8, u8>);
impl NBTOBINTEGAP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBTOBINTEGAP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBTOBINTEGAP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBTOBINTEGAP2` writer - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub struct NBTOBINTEGAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTOBINTEGAP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `NBTOBINTEGAP1` reader - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
pub struct NBTOBINTEGAP1_R(crate::FieldReader<u8, u8>);
impl NBTOBINTEGAP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBTOBINTEGAP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBTOBINTEGAP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBTOBINTEGAP1` writer - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
pub struct NBTOBINTEGAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTOBINTEGAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&self) -> NBTOBINTEGAP2_R {
        NBTOBINTEGAP2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&self) -> NBTOBINTEGAP1_R {
        NBTOBINTEGAP1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&mut self) -> NBTOBINTEGAP2_W {
        NBTOBINTEGAP2_W { w: self }
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&mut self) -> NBTOBINTEGAP1_W {
        NBTOBINTEGAP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non Back-to-Back Inter-Packet-Gap register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgr](index.html) module"]
pub struct IPGR_SPEC;
impl crate::RegisterSpec for IPGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipgr::R](R) reader structure"]
impl crate::Readable for IPGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipgr::W](W) writer structure"]
impl crate::Writable for IPGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPGR to value 0"]
impl crate::Resettable for IPGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
