#[doc = "Register `RXFILTERWOLSTATUS` reader"]
pub struct R(crate::R<RXFILTERWOLSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFILTERWOLSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFILTERWOLSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFILTERWOLSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AUW` reader - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
pub struct AUW_R(crate::FieldReader<bool, bool>);
impl AUW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABW` reader - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
pub struct ABW_R(crate::FieldReader<bool, bool>);
impl ABW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMW` reader - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
pub struct AMW_R(crate::FieldReader<bool, bool>);
impl AMW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUHW` reader - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
pub struct AUHW_R(crate::FieldReader<bool, bool>);
impl AUHW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUHW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUHW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMHW` reader - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
pub struct AMHW_R(crate::FieldReader<bool, bool>);
impl AMHW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMHW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMHW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APW` reader - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
pub struct APW_R(crate::FieldReader<bool, bool>);
impl APW_R {
    pub(crate) fn new(bits: bool) -> Self {
        APW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFW` reader - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
pub struct RFW_R(crate::FieldReader<bool, bool>);
impl RFW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPW` reader - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
pub struct MPW_R(crate::FieldReader<bool, bool>);
impl MPW_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
    #[inline(always)]
    pub fn auw(&self) -> AUW_R {
        AUW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
    #[inline(always)]
    pub fn abw(&self) -> ABW_R {
        ABW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
    #[inline(always)]
    pub fn amw(&self) -> AMW_R {
        AMW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn auhw(&self) -> AUHW_R {
        AUHW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn amhw(&self) -> AMHW_R {
        AMHW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
    #[inline(always)]
    pub fn apw(&self) -> APW_R {
        APW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
    #[inline(always)]
    pub fn rfw(&self) -> RFW_R {
        RFW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
    #[inline(always)]
    pub fn mpw(&self) -> MPW_R {
        MPW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Receive filter WoL status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfilterwolstatus](index.html) module"]
pub struct RXFILTERWOLSTATUS_SPEC;
impl crate::RegisterSpec for RXFILTERWOLSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfilterwolstatus::R](R) reader structure"]
impl crate::Readable for RXFILTERWOLSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFILTERWOLSTATUS to value 0"]
impl crate::Resettable for RXFILTERWOLSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
