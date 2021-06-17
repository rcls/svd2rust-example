#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB_DONE` reader - RCB Controller transfer done event. On completion of every RCB transaction, this bit is set"]
pub struct RCB_DONE_R(crate::FieldReader<bool, bool>);
impl RCB_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCB_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCB_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCB_DONE` writer - RCB Controller transfer done event. On completion of every RCB transaction, this bit is set"]
pub struct RCB_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB_DONE_W<'a> {
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
#[doc = "Field `TX_FIFO_TRIGGER` reader - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL."]
pub struct TX_FIFO_TRIGGER_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_TRIGGER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_TRIGGER` writer - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL."]
pub struct TX_FIFO_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_TRIGGER_W<'a> {
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
#[doc = "Field `TX_FIFO_NOT_FULL` reader - TX FIFO is not full. entries != TX_ENTRIES"]
pub struct TX_FIFO_NOT_FULL_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_NOT_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_NOT_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_NOT_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_NOT_FULL` writer - TX FIFO is not full. entries != TX_ENTRIES"]
pub struct TX_FIFO_NOT_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_NOT_FULL_W<'a> {
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
#[doc = "Field `TX_FIFO_EMPTY` reader - TX FIFO is empty; i.e. it has 0 entries."]
pub struct TX_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_EMPTY` writer - TX FIFO is empty; i.e. it has 0 entries."]
pub struct TX_FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EMPTY_W<'a> {
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
#[doc = "Field `TX_FIFO_OVERFLOW` reader - Attempt to write to a full TX FIFO."]
pub struct TX_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_OVERFLOW` writer - Attempt to write to a full TX FIFO."]
pub struct TX_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_OVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TX_FIFO_UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'."]
pub struct TX_FIFO_UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_UNDERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'."]
pub struct TX_FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_UNDERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RX_FIFO_TRIGGER` reader - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
pub struct RX_FIFO_TRIGGER_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_TRIGGER_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_TRIGGER` writer - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
pub struct RX_FIFO_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_TRIGGER_W<'a> {
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
#[doc = "Field `RX_FIFO_NOT_EMPTY` reader - RX FIFO is not empty."]
pub struct RX_FIFO_NOT_EMPTY_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_NOT_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_NOT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_NOT_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_NOT_EMPTY` writer - RX FIFO is not empty."]
pub struct RX_FIFO_NOT_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_NOT_EMPTY_W<'a> {
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
#[doc = "Field `RX_FIFO_FULL` reader - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. entries == (16-TX_ENTRIES_"]
pub struct RX_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_FULL` writer - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. entries == (16-TX_ENTRIES_"]
pub struct RX_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RX_FIFO_OVERFLOW` reader - Attempt to write to a full RX FIFO."]
pub struct RX_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_OVERFLOW` writer - Attempt to write to a full RX FIFO."]
pub struct RX_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_OVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RX_FIFO_UNDERFLOW` reader - Attempt to read from an empty RX FIFO."]
pub struct RX_FIFO_UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_UNDERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_UNDERFLOW` writer - Attempt to read from an empty RX FIFO."]
pub struct RX_FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_UNDERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RCB Controller transfer done event. On completion of every RCB transaction, this bit is set"]
    #[inline(always)]
    pub fn rcb_done(&self) -> RCB_DONE_R {
        RCB_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX FIFO is not full. entries != TX_ENTRIES"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TX_FIFO_NOT_FULL_R {
        TX_FIFO_NOT_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'."]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. entries == (16-TX_ENTRIES_"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULL_R {
        RX_FIFO_FULL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RCB Controller transfer done event. On completion of every RCB transaction, this bit is set"]
    #[inline(always)]
    pub fn rcb_done(&mut self) -> RCB_DONE_W {
        RCB_DONE_W { w: self }
    }
    #[doc = "Bit 8 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&mut self) -> TX_FIFO_TRIGGER_W {
        TX_FIFO_TRIGGER_W { w: self }
    }
    #[doc = "Bit 9 - TX FIFO is not full. entries != TX_ENTRIES"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&mut self) -> TX_FIFO_NOT_FULL_W {
        TX_FIFO_NOT_FULL_W { w: self }
    }
    #[doc = "Bit 10 - TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub fn tx_fifo_empty(&mut self) -> TX_FIFO_EMPTY_W {
        TX_FIFO_EMPTY_W { w: self }
    }
    #[doc = "Bit 11 - Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_overflow(&mut self) -> TX_FIFO_OVERFLOW_W {
        TX_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 12 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'."]
    #[inline(always)]
    pub fn tx_fifo_underflow(&mut self) -> TX_FIFO_UNDERFLOW_W {
        TX_FIFO_UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTL."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&mut self) -> RX_FIFO_TRIGGER_W {
        RX_FIFO_TRIGGER_W { w: self }
    }
    #[doc = "Bit 17 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&mut self) -> RX_FIFO_NOT_EMPTY_W {
        RX_FIFO_NOT_EMPTY_W { w: self }
    }
    #[doc = "Bit 18 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. entries == (16-TX_ENTRIES_"]
    #[inline(always)]
    pub fn rx_fifo_full(&mut self) -> RX_FIFO_FULL_W {
        RX_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 19 - Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W {
        RX_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 20 - Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_underflow(&mut self) -> RX_FIFO_UNDERFLOW_W {
        RX_FIFO_UNDERFLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0x0600"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
