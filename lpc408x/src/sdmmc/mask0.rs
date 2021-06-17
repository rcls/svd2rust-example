#[doc = "Register `MASK0` reader"]
pub struct R(crate::R<MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK0` writer"]
pub struct W(crate::W<MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK0_SPEC>;
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
impl From<crate::W<MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK0` reader - Mask CmdCrcFail flag."]
pub struct MASK0_R(crate::FieldReader<bool, bool>);
impl MASK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK0` writer - Mask CmdCrcFail flag."]
pub struct MASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK0_W<'a> {
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
#[doc = "Field `MASK1` reader - Mask DataCrcFail flag."]
pub struct MASK1_R(crate::FieldReader<bool, bool>);
impl MASK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK1` writer - Mask DataCrcFail flag."]
pub struct MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK1_W<'a> {
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
#[doc = "Field `MASK2` reader - Mask CmdTimeOut flag."]
pub struct MASK2_R(crate::FieldReader<bool, bool>);
impl MASK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK2` writer - Mask CmdTimeOut flag."]
pub struct MASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK2_W<'a> {
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
#[doc = "Field `MASK3` reader - Mask DataTimeOut flag."]
pub struct MASK3_R(crate::FieldReader<bool, bool>);
impl MASK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK3` writer - Mask DataTimeOut flag."]
pub struct MASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK3_W<'a> {
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
#[doc = "Field `MASK4` reader - Mask TxUnderrun flag."]
pub struct MASK4_R(crate::FieldReader<bool, bool>);
impl MASK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK4` writer - Mask TxUnderrun flag."]
pub struct MASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK4_W<'a> {
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
#[doc = "Field `MASK5` reader - Mask RxOverrun flag."]
pub struct MASK5_R(crate::FieldReader<bool, bool>);
impl MASK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK5` writer - Mask RxOverrun flag."]
pub struct MASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK5_W<'a> {
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
#[doc = "Field `MASK6` reader - Mask CmdRespEnd flag."]
pub struct MASK6_R(crate::FieldReader<bool, bool>);
impl MASK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK6` writer - Mask CmdRespEnd flag."]
pub struct MASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK6_W<'a> {
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
#[doc = "Field `MASK7` reader - Mask CmdSent flag."]
pub struct MASK7_R(crate::FieldReader<bool, bool>);
impl MASK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK7` writer - Mask CmdSent flag."]
pub struct MASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK7_W<'a> {
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
#[doc = "Field `MASK8` reader - Mask DataEnd flag."]
pub struct MASK8_R(crate::FieldReader<bool, bool>);
impl MASK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK8` writer - Mask DataEnd flag."]
pub struct MASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK8_W<'a> {
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
#[doc = "Field `MASK9` reader - Mask StartBitErr flag."]
pub struct MASK9_R(crate::FieldReader<bool, bool>);
impl MASK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK9` writer - Mask StartBitErr flag."]
pub struct MASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK9_W<'a> {
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
#[doc = "Field `MASK10` reader - Mask DataBlockEnd flag."]
pub struct MASK10_R(crate::FieldReader<bool, bool>);
impl MASK10_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK10` writer - Mask DataBlockEnd flag."]
pub struct MASK10_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK10_W<'a> {
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
#[doc = "Field `MASK11` reader - Mask CmdActive flag."]
pub struct MASK11_R(crate::FieldReader<bool, bool>);
impl MASK11_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK11` writer - Mask CmdActive flag."]
pub struct MASK11_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK11_W<'a> {
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
#[doc = "Field `MASK12` reader - Mask TxActive flag."]
pub struct MASK12_R(crate::FieldReader<bool, bool>);
impl MASK12_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK12` writer - Mask TxActive flag."]
pub struct MASK12_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK12_W<'a> {
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
#[doc = "Field `MASK13` reader - Mask RxActive flag."]
pub struct MASK13_R(crate::FieldReader<bool, bool>);
impl MASK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK13` writer - Mask RxActive flag."]
pub struct MASK13_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `MASK14` reader - Mask TxFifoHalfEmpty flag."]
pub struct MASK14_R(crate::FieldReader<bool, bool>);
impl MASK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK14` writer - Mask TxFifoHalfEmpty flag."]
pub struct MASK14_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `MASK15` reader - Mask RxFifoHalfFull flag."]
pub struct MASK15_R(crate::FieldReader<bool, bool>);
impl MASK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK15` writer - Mask RxFifoHalfFull flag."]
pub struct MASK15_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MASK16` reader - Mask TxFifoFull flag."]
pub struct MASK16_R(crate::FieldReader<bool, bool>);
impl MASK16_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK16` writer - Mask TxFifoFull flag."]
pub struct MASK16_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK16_W<'a> {
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
#[doc = "Field `MASK17` reader - Mask RxFifoFull flag."]
pub struct MASK17_R(crate::FieldReader<bool, bool>);
impl MASK17_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK17` writer - Mask RxFifoFull flag."]
pub struct MASK17_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK17_W<'a> {
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
#[doc = "Field `MASK18` reader - Mask TxFifoEmpty flag."]
pub struct MASK18_R(crate::FieldReader<bool, bool>);
impl MASK18_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK18` writer - Mask TxFifoEmpty flag."]
pub struct MASK18_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK18_W<'a> {
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
#[doc = "Field `MASK19` reader - Mask RxFifoEmpty flag."]
pub struct MASK19_R(crate::FieldReader<bool, bool>);
impl MASK19_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK19` writer - Mask RxFifoEmpty flag."]
pub struct MASK19_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK19_W<'a> {
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
#[doc = "Field `MASK20` reader - Mask TxDataAvlbl flag."]
pub struct MASK20_R(crate::FieldReader<bool, bool>);
impl MASK20_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK20` writer - Mask TxDataAvlbl flag."]
pub struct MASK20_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK20_W<'a> {
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
#[doc = "Field `MASK21` reader - Mask RxDataAvlbl flag."]
pub struct MASK21_R(crate::FieldReader<bool, bool>);
impl MASK21_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK21` writer - Mask RxDataAvlbl flag."]
pub struct MASK21_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mask CmdCrcFail flag."]
    #[inline(always)]
    pub fn mask0(&self) -> MASK0_R {
        MASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask DataCrcFail flag."]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask CmdTimeOut flag."]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask DataTimeOut flag."]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask TxUnderrun flag."]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask RxOverrun flag."]
    #[inline(always)]
    pub fn mask5(&self) -> MASK5_R {
        MASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask CmdRespEnd flag."]
    #[inline(always)]
    pub fn mask6(&self) -> MASK6_R {
        MASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask CmdSent flag."]
    #[inline(always)]
    pub fn mask7(&self) -> MASK7_R {
        MASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask DataEnd flag."]
    #[inline(always)]
    pub fn mask8(&self) -> MASK8_R {
        MASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask StartBitErr flag."]
    #[inline(always)]
    pub fn mask9(&self) -> MASK9_R {
        MASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mask DataBlockEnd flag."]
    #[inline(always)]
    pub fn mask10(&self) -> MASK10_R {
        MASK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask CmdActive flag."]
    #[inline(always)]
    pub fn mask11(&self) -> MASK11_R {
        MASK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mask TxActive flag."]
    #[inline(always)]
    pub fn mask12(&self) -> MASK12_R {
        MASK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mask RxActive flag."]
    #[inline(always)]
    pub fn mask13(&self) -> MASK13_R {
        MASK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mask TxFifoHalfEmpty flag."]
    #[inline(always)]
    pub fn mask14(&self) -> MASK14_R {
        MASK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask RxFifoHalfFull flag."]
    #[inline(always)]
    pub fn mask15(&self) -> MASK15_R {
        MASK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mask TxFifoFull flag."]
    #[inline(always)]
    pub fn mask16(&self) -> MASK16_R {
        MASK16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mask RxFifoFull flag."]
    #[inline(always)]
    pub fn mask17(&self) -> MASK17_R {
        MASK17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mask TxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask18(&self) -> MASK18_R {
        MASK18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mask RxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask19(&self) -> MASK19_R {
        MASK19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Mask TxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask20(&self) -> MASK20_R {
        MASK20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Mask RxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask21(&self) -> MASK21_R {
        MASK21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask CmdCrcFail flag."]
    #[inline(always)]
    pub fn mask0(&mut self) -> MASK0_W {
        MASK0_W { w: self }
    }
    #[doc = "Bit 1 - Mask DataCrcFail flag."]
    #[inline(always)]
    pub fn mask1(&mut self) -> MASK1_W {
        MASK1_W { w: self }
    }
    #[doc = "Bit 2 - Mask CmdTimeOut flag."]
    #[inline(always)]
    pub fn mask2(&mut self) -> MASK2_W {
        MASK2_W { w: self }
    }
    #[doc = "Bit 3 - Mask DataTimeOut flag."]
    #[inline(always)]
    pub fn mask3(&mut self) -> MASK3_W {
        MASK3_W { w: self }
    }
    #[doc = "Bit 4 - Mask TxUnderrun flag."]
    #[inline(always)]
    pub fn mask4(&mut self) -> MASK4_W {
        MASK4_W { w: self }
    }
    #[doc = "Bit 5 - Mask RxOverrun flag."]
    #[inline(always)]
    pub fn mask5(&mut self) -> MASK5_W {
        MASK5_W { w: self }
    }
    #[doc = "Bit 6 - Mask CmdRespEnd flag."]
    #[inline(always)]
    pub fn mask6(&mut self) -> MASK6_W {
        MASK6_W { w: self }
    }
    #[doc = "Bit 7 - Mask CmdSent flag."]
    #[inline(always)]
    pub fn mask7(&mut self) -> MASK7_W {
        MASK7_W { w: self }
    }
    #[doc = "Bit 8 - Mask DataEnd flag."]
    #[inline(always)]
    pub fn mask8(&mut self) -> MASK8_W {
        MASK8_W { w: self }
    }
    #[doc = "Bit 9 - Mask StartBitErr flag."]
    #[inline(always)]
    pub fn mask9(&mut self) -> MASK9_W {
        MASK9_W { w: self }
    }
    #[doc = "Bit 10 - Mask DataBlockEnd flag."]
    #[inline(always)]
    pub fn mask10(&mut self) -> MASK10_W {
        MASK10_W { w: self }
    }
    #[doc = "Bit 11 - Mask CmdActive flag."]
    #[inline(always)]
    pub fn mask11(&mut self) -> MASK11_W {
        MASK11_W { w: self }
    }
    #[doc = "Bit 12 - Mask TxActive flag."]
    #[inline(always)]
    pub fn mask12(&mut self) -> MASK12_W {
        MASK12_W { w: self }
    }
    #[doc = "Bit 13 - Mask RxActive flag."]
    #[inline(always)]
    pub fn mask13(&mut self) -> MASK13_W {
        MASK13_W { w: self }
    }
    #[doc = "Bit 14 - Mask TxFifoHalfEmpty flag."]
    #[inline(always)]
    pub fn mask14(&mut self) -> MASK14_W {
        MASK14_W { w: self }
    }
    #[doc = "Bit 15 - Mask RxFifoHalfFull flag."]
    #[inline(always)]
    pub fn mask15(&mut self) -> MASK15_W {
        MASK15_W { w: self }
    }
    #[doc = "Bit 16 - Mask TxFifoFull flag."]
    #[inline(always)]
    pub fn mask16(&mut self) -> MASK16_W {
        MASK16_W { w: self }
    }
    #[doc = "Bit 17 - Mask RxFifoFull flag."]
    #[inline(always)]
    pub fn mask17(&mut self) -> MASK17_W {
        MASK17_W { w: self }
    }
    #[doc = "Bit 18 - Mask TxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask18(&mut self) -> MASK18_W {
        MASK18_W { w: self }
    }
    #[doc = "Bit 19 - Mask RxFifoEmpty flag."]
    #[inline(always)]
    pub fn mask19(&mut self) -> MASK19_W {
        MASK19_W { w: self }
    }
    #[doc = "Bit 20 - Mask TxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask20(&mut self) -> MASK20_W {
        MASK20_W { w: self }
    }
    #[doc = "Bit 21 - Mask RxDataAvlbl flag."]
    #[inline(always)]
    pub fn mask21(&mut self) -> MASK21_W {
        MASK21_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 0 mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](index.html) module"]
pub struct MASK0_SPEC;
impl crate::RegisterSpec for MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask0::R](R) reader structure"]
impl crate::Readable for MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask0::W](W) writer structure"]
impl crate::Writable for MASK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK0 to value 0"]
impl crate::Resettable for MASK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
