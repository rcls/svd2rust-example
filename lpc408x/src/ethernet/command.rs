#[doc = "Register `COMMAND` reader"]
pub struct R(crate::R<COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMAND` writer"]
pub struct W(crate::W<COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMAND_SPEC>;
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
impl From<crate::W<COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXENABLE` reader - Enable receive."]
pub struct RXENABLE_R(crate::FieldReader<bool, bool>);
impl RXENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXENABLE` writer - Enable receive."]
pub struct RXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENABLE_W<'a> {
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
#[doc = "Field `TXENABLE` reader - Enable transmit."]
pub struct TXENABLE_R(crate::FieldReader<bool, bool>);
impl TXENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENABLE` writer - Enable transmit."]
pub struct TXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENABLE_W<'a> {
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
#[doc = "Field `REGRESET` reader - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
pub struct REGRESET_R(crate::FieldReader<bool, bool>);
impl REGRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGRESET` writer - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
pub struct REGRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> REGRESET_W<'a> {
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
#[doc = "Field `TXRESET` reader - When a 1 is written, the transmit datapath is reset."]
pub struct TXRESET_R(crate::FieldReader<bool, bool>);
impl TXRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRESET` writer - When a 1 is written, the transmit datapath is reset."]
pub struct TXRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RXRESET` reader - When a 1 is written, the receive datapath is reset."]
pub struct RXRESET_R(crate::FieldReader<bool, bool>);
impl RXRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRESET` writer - When a 1 is written, the receive datapath is reset."]
pub struct RXRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PASSRUNTFRAME` reader - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
pub struct PASSRUNTFRAME_R(crate::FieldReader<bool, bool>);
impl PASSRUNTFRAME_R {
    pub(crate) fn new(bits: bool) -> Self {
        PASSRUNTFRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASSRUNTFRAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASSRUNTFRAME` writer - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
pub struct PASSRUNTFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSRUNTFRAME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PASSRXFILTER` reader - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
pub struct PASSRXFILTER_R(crate::FieldReader<bool, bool>);
impl PASSRXFILTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PASSRXFILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PASSRXFILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PASSRXFILTER` writer - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
pub struct PASSRXFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> PASSRXFILTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TXFLOWCONTROL` reader - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
pub struct TXFLOWCONTROL_R(crate::FieldReader<bool, bool>);
impl TXFLOWCONTROL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFLOWCONTROL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFLOWCONTROL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFLOWCONTROL` writer - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
pub struct TXFLOWCONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLOWCONTROL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RMII` reader - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
pub struct RMII_R(crate::FieldReader<bool, bool>);
impl RMII_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII` writer - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
pub struct RMII_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FULLDUPLEX` reader - When set to 1 , indicates full duplex operation."]
pub struct FULLDUPLEX_R(crate::FieldReader<bool, bool>);
impl FULLDUPLEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULLDUPLEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULLDUPLEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULLDUPLEX` writer - When set to 1 , indicates full duplex operation."]
pub struct FULLDUPLEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUPLEX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&self) -> TXENABLE_R {
        TXENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&self) -> REGRESET_R {
        REGRESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&self) -> TXRESET_R {
        TXRESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&self) -> RXRESET_R {
        RXRESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&self) -> PASSRUNTFRAME_R {
        PASSRUNTFRAME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&self) -> PASSRXFILTER_R {
        PASSRXFILTER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&self) -> TXFLOWCONTROL_R {
        TXFLOWCONTROL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RXENABLE_W {
        RXENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&mut self) -> TXENABLE_W {
        TXENABLE_W { w: self }
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&mut self) -> REGRESET_W {
        REGRESET_W { w: self }
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&mut self) -> TXRESET_W {
        TXRESET_W { w: self }
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&mut self) -> RXRESET_W {
        RXRESET_W { w: self }
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&mut self) -> PASSRUNTFRAME_W {
        PASSRUNTFRAME_W { w: self }
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&mut self) -> PASSRXFILTER_W {
        PASSRXFILTER_W { w: self }
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&mut self) -> TXFLOWCONTROL_W {
        TXFLOWCONTROL_W { w: self }
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&mut self) -> RMII_W {
        RMII_W { w: self }
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W {
        FULLDUPLEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](index.html) module"]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [command::R](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [command::W](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
