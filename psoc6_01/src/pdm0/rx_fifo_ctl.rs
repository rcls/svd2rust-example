#[doc = "Register `RX_FIFO_CTL` reader"]
pub struct R(crate::R<RX_FIFO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_FIFO_CTL` writer"]
pub struct W(crate::W<RX_FIFO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_FIFO_CTL_SPEC>;
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
impl From<crate::W<RX_FIFO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_FIFO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
pub struct TRIGGER_LEVEL_R(crate::FieldReader<u8, u8>);
impl TRIGGER_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGGER_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
pub struct TRIGGER_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CLEAR` reader - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub struct CLEAR_R(crate::FieldReader<bool, bool>);
impl CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAR` writer - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FREEZE` reader - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
pub struct FREEZE_R(crate::FieldReader<bool, bool>);
impl FREEZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREEZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREEZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREEZE` writer - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
pub struct FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREEZE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 254 in Mono channel enabled (MODE_CTL.PCM_CH_SET = '1' or '2'), up to 253 in Stereo channel enabled (MODE_CTL.PCM_CH_SET = '3')."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W {
        TRIGGER_LEVEL_W { w: self }
    }
    #[doc = "Bit 16 - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 17 - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W {
        FREEZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_ctl](index.html) module"]
pub struct RX_FIFO_CTL_SPEC;
impl crate::RegisterSpec for RX_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_ctl::R](R) reader structure"]
impl crate::Readable for RX_FIFO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_fifo_ctl::W](W) writer structure"]
impl crate::Writable for RX_FIFO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_FIFO_CTL to value 0"]
impl crate::Resettable for RX_FIFO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
