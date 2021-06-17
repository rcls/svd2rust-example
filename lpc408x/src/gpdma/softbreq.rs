#[doc = "Register `SOFTBREQ` reader"]
pub struct R(crate::R<SOFTBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTBREQ` writer"]
pub struct W(crate::W<SOFTBREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTBREQ_SPEC>;
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
impl From<crate::W<SOFTBREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTBREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTBREQ0` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ0_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ0` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ0_W<'a> {
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
#[doc = "Field `SOFTBREQ1` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ1_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ1` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ1_W<'a> {
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
#[doc = "Field `SOFTBREQ2` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ2_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ2` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ2_W<'a> {
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
#[doc = "Field `SOFTBREQ3` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ3_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ3` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ3_W<'a> {
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
#[doc = "Field `SOFTBREQ4` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ4_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ4` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ4_W<'a> {
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
#[doc = "Field `SOFTBREQ5` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ5_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ5` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ5_W<'a> {
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
#[doc = "Field `SOFTBREQ6` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ6_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ6` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ6_W<'a> {
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
#[doc = "Field `SOFTBREQ7` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ7_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ7` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ7_W<'a> {
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
#[doc = "Field `SOFTBREQ8` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ8_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ8` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ8_W<'a> {
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
#[doc = "Field `SOFTBREQ9` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ9_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ9` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ9_W<'a> {
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
#[doc = "Field `SOFTBREQ10` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ10_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ10` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ10_W<'a> {
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
#[doc = "Field `SOFTBREQ11` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ11_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ11` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ11_W<'a> {
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
#[doc = "Field `SOFTBREQ12` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ12_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ12` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ12_W<'a> {
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
#[doc = "Field `SOFTBREQ13` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ13_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ13` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ13_W<'a> {
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
#[doc = "Field `SOFTBREQ14` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ14_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ14` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ14_W<'a> {
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
#[doc = "Field `SOFTBREQ15` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ15_R(crate::FieldReader<bool, bool>);
impl SOFTBREQ15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTBREQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTBREQ15` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub struct SOFTBREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq0(&self) -> SOFTBREQ0_R {
        SOFTBREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq1(&self) -> SOFTBREQ1_R {
        SOFTBREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq2(&self) -> SOFTBREQ2_R {
        SOFTBREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq3(&self) -> SOFTBREQ3_R {
        SOFTBREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq4(&self) -> SOFTBREQ4_R {
        SOFTBREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq5(&self) -> SOFTBREQ5_R {
        SOFTBREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq6(&self) -> SOFTBREQ6_R {
        SOFTBREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq7(&self) -> SOFTBREQ7_R {
        SOFTBREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq8(&self) -> SOFTBREQ8_R {
        SOFTBREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq9(&self) -> SOFTBREQ9_R {
        SOFTBREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq10(&self) -> SOFTBREQ10_R {
        SOFTBREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq11(&self) -> SOFTBREQ11_R {
        SOFTBREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq12(&self) -> SOFTBREQ12_R {
        SOFTBREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq13(&self) -> SOFTBREQ13_R {
        SOFTBREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq14(&self) -> SOFTBREQ14_R {
        SOFTBREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq15(&self) -> SOFTBREQ15_R {
        SOFTBREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq0(&mut self) -> SOFTBREQ0_W {
        SOFTBREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq1(&mut self) -> SOFTBREQ1_W {
        SOFTBREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq2(&mut self) -> SOFTBREQ2_W {
        SOFTBREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq3(&mut self) -> SOFTBREQ3_W {
        SOFTBREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq4(&mut self) -> SOFTBREQ4_W {
        SOFTBREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq5(&mut self) -> SOFTBREQ5_W {
        SOFTBREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq6(&mut self) -> SOFTBREQ6_W {
        SOFTBREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq7(&mut self) -> SOFTBREQ7_W {
        SOFTBREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq8(&mut self) -> SOFTBREQ8_W {
        SOFTBREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq9(&mut self) -> SOFTBREQ9_W {
        SOFTBREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq10(&mut self) -> SOFTBREQ10_W {
        SOFTBREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq11(&mut self) -> SOFTBREQ11_W {
        SOFTBREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq12(&mut self) -> SOFTBREQ12_W {
        SOFTBREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq13(&mut self) -> SOFTBREQ13_W {
        SOFTBREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq14(&mut self) -> SOFTBREQ14_W {
        SOFTBREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq15(&mut self) -> SOFTBREQ15_W {
        SOFTBREQ15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Software Burst Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softbreq](index.html) module"]
pub struct SOFTBREQ_SPEC;
impl crate::RegisterSpec for SOFTBREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softbreq::R](R) reader structure"]
impl crate::Readable for SOFTBREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softbreq::W](W) writer structure"]
impl crate::Writable for SOFTBREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTBREQ to value 0"]
impl crate::Resettable for SOFTBREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
