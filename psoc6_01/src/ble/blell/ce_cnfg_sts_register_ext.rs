#[doc = "Register `CE_CNFG_STS_REGISTER_EXT` reader"]
pub struct R(crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_CNFG_STS_REGISTER_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_CNFG_STS_REGISTER_EXT` writer"]
pub struct W(crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>;
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
impl From<crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_CNFG_STS_REGISTER_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_2M` reader - transmittion on 2M"]
pub struct TX_2M_R(crate::FieldReader<bool, bool>);
impl TX_2M_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_2M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_2M` writer - transmittion on 2M"]
pub struct TX_2M_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_2M_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RX_2M` reader - receiving on 2M"]
pub struct RX_2M_R(crate::FieldReader<bool, bool>);
impl RX_2M_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_2M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_2M` writer - receiving on 2M"]
pub struct RX_2M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_2M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SN` reader - Sequence number for next scheduled connection index"]
pub struct SN_R(crate::FieldReader<bool, bool>);
impl SN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SN` writer - Sequence number for next scheduled connection index"]
pub struct SN_W<'a> {
    w: &'a mut W,
}
impl<'a> SN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `NESN` reader - Next Sequence number for next scheduled connection index"]
pub struct NESN_R(crate::FieldReader<bool, bool>);
impl NESN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NESN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NESN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NESN` writer - Next Sequence number for next scheduled connection index"]
pub struct NESN_W<'a> {
    w: &'a mut W,
}
impl<'a> NESN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `LAST_UNMAPPED_CHANNEL` reader - Last unmapped channel for next scheduled connection index"]
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
#[doc = "Field `LAST_UNMAPPED_CHANNEL` writer - Last unmapped channel for next scheduled connection index"]
pub struct LAST_UNMAPPED_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_UNMAPPED_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - transmittion on 2M"]
    #[inline(always)]
    pub fn tx_2m(&self) -> TX_2M_R {
        TX_2M_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - receiving on 2M"]
    #[inline(always)]
    pub fn rx_2m(&self) -> RX_2M_R {
        RX_2M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn sn(&self) -> SN_R {
        SN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Next Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn nesn(&self) -> NESN_R {
        NESN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Last unmapped channel for next scheduled connection index"]
    #[inline(always)]
    pub fn last_unmapped_channel(&self) -> LAST_UNMAPPED_CHANNEL_R {
        LAST_UNMAPPED_CHANNEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - transmittion on 2M"]
    #[inline(always)]
    pub fn tx_2m(&mut self) -> TX_2M_W {
        TX_2M_W { w: self }
    }
    #[doc = "Bit 1 - receiving on 2M"]
    #[inline(always)]
    pub fn rx_2m(&mut self) -> RX_2M_W {
        RX_2M_W { w: self }
    }
    #[doc = "Bit 2 - Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn sn(&mut self) -> SN_W {
        SN_W { w: self }
    }
    #[doc = "Bit 3 - Next Sequence number for next scheduled connection index"]
    #[inline(always)]
    pub fn nesn(&mut self) -> NESN_W {
        NESN_W { w: self }
    }
    #[doc = "Bits 8:13 - Last unmapped channel for next scheduled connection index"]
    #[inline(always)]
    pub fn last_unmapped_channel(&mut self) -> LAST_UNMAPPED_CHANNEL_W {
        LAST_UNMAPPED_CHANNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "connection configuration & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_cnfg_sts_register_ext](index.html) module"]
pub struct CE_CNFG_STS_REGISTER_EXT_SPEC;
impl crate::RegisterSpec for CE_CNFG_STS_REGISTER_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_cnfg_sts_register_ext::R](R) reader structure"]
impl crate::Readable for CE_CNFG_STS_REGISTER_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_cnfg_sts_register_ext::W](W) writer structure"]
impl crate::Writable for CE_CNFG_STS_REGISTER_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_CNFG_STS_REGISTER_EXT to value 0"]
impl crate::Resettable for CE_CNFG_STS_REGISTER_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
