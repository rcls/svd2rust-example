#[doc = "Register `MMMS_CONN_STATUS` reader"]
pub struct R(crate::R<MMMS_CONN_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_CONN_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_CONN_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_CONN_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURR_CONN_INDEX` reader - Connection Index that was serviced. Legal values are 0,1,2,3."]
pub struct CURR_CONN_INDEX_R(crate::FieldReader<u8, u8>);
impl CURR_CONN_INDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CURR_CONN_INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURR_CONN_INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURR_CONN_TYPE` reader - Connection type 1 - Master Connection 0 - Slave Connection"]
pub struct CURR_CONN_TYPE_R(crate::FieldReader<bool, bool>);
impl CURR_CONN_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CURR_CONN_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURR_CONN_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SN_CURR` reader - Sequence Number of Packets exchanged"]
pub struct SN_CURR_R(crate::FieldReader<bool, bool>);
impl SN_CURR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SN_CURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SN_CURR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NESN_CURR` reader - Next Sequence Number"]
pub struct NESN_CURR_R(crate::FieldReader<bool, bool>);
impl NESN_CURR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NESN_CURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NESN_CURR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_UNMAPPED_CHANNEL` reader - Last Unmapped Channel"]
pub struct LAST_UNMAPPED_CHANNEL_R(crate::FieldReader<u8, u8>);
impl LAST_UNMAPPED_CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LAST_UNMAPPED_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_UNMAPPED_CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKT_MISS` reader - 1 - Packet Missed 0 - Connection exchanged packets"]
pub struct PKT_MISS_R(crate::FieldReader<bool, bool>);
impl PKT_MISS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKT_MISS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKT_MISS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANCHOR_PT_STATE` reader - Anchor Point State 0 - Anchor point missed 1 - Anchor point established"]
pub struct ANCHOR_PT_STATE_R(crate::FieldReader<bool, bool>);
impl ANCHOR_PT_STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANCHOR_PT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANCHOR_PT_STATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Connection Index that was serviced. Legal values are 0,1,2,3."]
    #[inline(always)]
    pub fn curr_conn_index(&self) -> CURR_CONN_INDEX_R {
        CURR_CONN_INDEX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn curr_conn_type(&self) -> CURR_CONN_TYPE_R {
        CURR_CONN_TYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sequence Number of Packets exchanged"]
    #[inline(always)]
    pub fn sn_curr(&self) -> SN_CURR_R {
        SN_CURR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Next Sequence Number"]
    #[inline(always)]
    pub fn nesn_curr(&self) -> NESN_CURR_R {
        NESN_CURR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Last Unmapped Channel"]
    #[inline(always)]
    pub fn last_unmapped_channel(&self) -> LAST_UNMAPPED_CHANNEL_R {
        LAST_UNMAPPED_CHANNEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 1 - Packet Missed 0 - Connection exchanged packets"]
    #[inline(always)]
    pub fn pkt_miss(&self) -> PKT_MISS_R {
        PKT_MISS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Anchor Point State 0 - Anchor point missed 1 - Anchor point established"]
    #[inline(always)]
    pub fn anchor_pt_state(&self) -> ANCHOR_PT_STATE_R {
        ANCHOR_PT_STATE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Connection Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_conn_status](index.html) module"]
pub struct MMMS_CONN_STATUS_SPEC;
impl crate::RegisterSpec for MMMS_CONN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_conn_status::R](R) reader structure"]
impl crate::Readable for MMMS_CONN_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMMS_CONN_STATUS to value 0"]
impl crate::Resettable for MMMS_CONN_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
