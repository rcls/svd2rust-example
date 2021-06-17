#[doc = "Register `SOFTLBREQ` reader"]
pub struct R(crate::R<SOFTLBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTLBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTLBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTLBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTLBREQ` writer"]
pub struct W(crate::W<SOFTLBREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTLBREQ_SPEC>;
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
impl From<crate::W<SOFTLBREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTLBREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTLBREQ0` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ0_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ0` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ0_W<'a> {
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
#[doc = "Field `SOFTLBREQ1` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ1_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ1` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ1_W<'a> {
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
#[doc = "Field `SOFTLBREQ2` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ2_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ2` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ2_W<'a> {
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
#[doc = "Field `SOFTLBREQ3` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ3_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ3` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ3_W<'a> {
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
#[doc = "Field `SOFTLBREQ4` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ4_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ4` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ4_W<'a> {
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
#[doc = "Field `SOFTLBREQ5` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ5_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ5` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ5_W<'a> {
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
#[doc = "Field `SOFTLBREQ6` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ6_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ6` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ6_W<'a> {
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
#[doc = "Field `SOFTLBREQ7` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ7_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ7` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ7_W<'a> {
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
#[doc = "Field `SOFTLBREQ8` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ8_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ8` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ8_W<'a> {
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
#[doc = "Field `SOFTLBREQ9` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ9_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ9` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ9_W<'a> {
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
#[doc = "Field `SOFTLBREQ10` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ10_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ10` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ10_W<'a> {
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
#[doc = "Field `SOFTLBREQ11` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ11_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ11` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ11_W<'a> {
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
#[doc = "Field `SOFTLBREQ12` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ12_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ12` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ12_W<'a> {
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
#[doc = "Field `SOFTLBREQ13` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ13_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ13` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ13_W<'a> {
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
#[doc = "Field `SOFTLBREQ14` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ14_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ14` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ14_W<'a> {
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
#[doc = "Field `SOFTLBREQ15` reader - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ15_R(crate::FieldReader<bool, bool>);
impl SOFTLBREQ15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLBREQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLBREQ15` writer - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
pub struct SOFTLBREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq0(&self) -> SOFTLBREQ0_R {
        SOFTLBREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq1(&self) -> SOFTLBREQ1_R {
        SOFTLBREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq2(&self) -> SOFTLBREQ2_R {
        SOFTLBREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq3(&self) -> SOFTLBREQ3_R {
        SOFTLBREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq4(&self) -> SOFTLBREQ4_R {
        SOFTLBREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq5(&self) -> SOFTLBREQ5_R {
        SOFTLBREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq6(&self) -> SOFTLBREQ6_R {
        SOFTLBREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq7(&self) -> SOFTLBREQ7_R {
        SOFTLBREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq8(&self) -> SOFTLBREQ8_R {
        SOFTLBREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq9(&self) -> SOFTLBREQ9_R {
        SOFTLBREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq10(&self) -> SOFTLBREQ10_R {
        SOFTLBREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq11(&self) -> SOFTLBREQ11_R {
        SOFTLBREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq12(&self) -> SOFTLBREQ12_R {
        SOFTLBREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq13(&self) -> SOFTLBREQ13_R {
        SOFTLBREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq14(&self) -> SOFTLBREQ14_R {
        SOFTLBREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq15(&self) -> SOFTLBREQ15_R {
        SOFTLBREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq0(&mut self) -> SOFTLBREQ0_W {
        SOFTLBREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq1(&mut self) -> SOFTLBREQ1_W {
        SOFTLBREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq2(&mut self) -> SOFTLBREQ2_W {
        SOFTLBREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq3(&mut self) -> SOFTLBREQ3_W {
        SOFTLBREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq4(&mut self) -> SOFTLBREQ4_W {
        SOFTLBREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq5(&mut self) -> SOFTLBREQ5_W {
        SOFTLBREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq6(&mut self) -> SOFTLBREQ6_W {
        SOFTLBREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq7(&mut self) -> SOFTLBREQ7_W {
        SOFTLBREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq8(&mut self) -> SOFTLBREQ8_W {
        SOFTLBREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq9(&mut self) -> SOFTLBREQ9_W {
        SOFTLBREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq10(&mut self) -> SOFTLBREQ10_W {
        SOFTLBREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq11(&mut self) -> SOFTLBREQ11_W {
        SOFTLBREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq12(&mut self) -> SOFTLBREQ12_W {
        SOFTLBREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq13(&mut self) -> SOFTLBREQ13_W {
        SOFTLBREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq14(&mut self) -> SOFTLBREQ14_W {
        SOFTLBREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq15(&mut self) -> SOFTLBREQ15_W {
        SOFTLBREQ15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Software Last Burst Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softlbreq](index.html) module"]
pub struct SOFTLBREQ_SPEC;
impl crate::RegisterSpec for SOFTLBREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softlbreq::R](R) reader structure"]
impl crate::Readable for SOFTLBREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softlbreq::W](W) writer structure"]
impl crate::Writable for SOFTLBREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTLBREQ to value 0"]
impl crate::Resettable for SOFTLBREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
