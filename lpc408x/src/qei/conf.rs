#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRINV` reader - Direction invert. When 1, complements the DIR bit."]
pub struct DIRINV_R(crate::FieldReader<bool, bool>);
impl DIRINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRINV` writer - Direction invert. When 1, complements the DIR bit."]
pub struct DIRINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRINV_W<'a> {
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
#[doc = "Field `SIGMODE` reader - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
pub struct SIGMODE_R(crate::FieldReader<bool, bool>);
impl SIGMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIGMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGMODE` writer - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
pub struct SIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGMODE_W<'a> {
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
#[doc = "Field `CAPMODE` reader - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
pub struct CAPMODE_R(crate::FieldReader<bool, bool>);
impl CAPMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPMODE` writer - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
pub struct CAPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPMODE_W<'a> {
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
#[doc = "Field `INVINX` reader - Invert Index. When 1, inverts the sense of the index input."]
pub struct INVINX_R(crate::FieldReader<bool, bool>);
impl INVINX_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVINX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVINX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVINX` writer - Invert Index. When 1, inverts the sense of the index input."]
pub struct INVINX_W<'a> {
    w: &'a mut W,
}
impl<'a> INVINX_W<'a> {
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
#[doc = "Field `CRESPI` reader - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
pub struct CRESPI_R(crate::FieldReader<bool, bool>);
impl CRESPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRESPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRESPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRESPI` writer - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
pub struct CRESPI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRESPI_W<'a> {
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
#[doc = "Field `INXGATE` reader - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
pub struct INXGATE_R(crate::FieldReader<u8, u8>);
impl INXGATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        INXGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INXGATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INXGATE` writer - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
pub struct INXGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INXGATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&self) -> DIRINV_R {
        DIRINV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&self) -> SIGMODE_R {
        SIGMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&self) -> INVINX_R {
        INVINX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&self) -> CRESPI_R {
        CRESPI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&self) -> INXGATE_R {
        INXGATE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&mut self) -> DIRINV_W {
        DIRINV_W { w: self }
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&mut self) -> SIGMODE_W {
        SIGMODE_W { w: self }
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&mut self) -> CAPMODE_W {
        CAPMODE_W { w: self }
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&mut self) -> INVINX_W {
        INVINX_W { w: self }
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&mut self) -> CRESPI_W {
        CRESPI_W { w: self }
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&mut self) -> INXGATE_W {
        INXGATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
