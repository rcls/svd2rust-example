#[doc = "Register `EMCCAL` reader"]
pub struct R(crate::R<EMCCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCCAL` writer"]
pub struct W(crate::W<EMCCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCCAL_SPEC>;
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
impl From<crate::W<EMCCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALVALUE` reader - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the IRC oscillator. This represents the composite effect of processing variation, internal regulator supply voltage, and ambient temperature."]
pub struct CALVALUE_R(crate::FieldReader<u8, u8>);
impl CALVALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALVALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALVALUE` writer - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the IRC oscillator. This represents the composite effect of processing variation, internal regulator supply voltage, and ambient temperature."]
pub struct CALVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `START` reader - Start control bit for the EMC calibration counter. Writing a 1 to this bit begins the measurement process. This bit is cleared automatically when the measurement is complete."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start control bit for the EMC calibration counter. Writing a 1 to this bit begins the measurement process. This bit is cleared automatically when the measurement is complete."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `DONE` reader - Measurement completion flag. this bit is set when a calibration measurement is completed. This bit is cleared automatically when the START bit is set."]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - Measurement completion flag. this bit is set when a calibration measurement is completed. This bit is cleared automatically when the START bit is set."]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
    #[doc = "Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the IRC oscillator. This represents the composite effect of processing variation, internal regulator supply voltage, and ambient temperature."]
    #[inline(always)]
    pub fn calvalue(&self) -> CALVALUE_R {
        CALVALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - Start control bit for the EMC calibration counter. Writing a 1 to this bit begins the measurement process. This bit is cleared automatically when the measurement is complete."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Measurement completion flag. this bit is set when a calibration measurement is completed. This bit is cleared automatically when the START bit is set."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Returns the count of the approximately 50 MHz ring oscillator that occur during 32 clocks of the IRC oscillator. This represents the composite effect of processing variation, internal regulator supply voltage, and ambient temperature."]
    #[inline(always)]
    pub fn calvalue(&mut self) -> CALVALUE_W {
        CALVALUE_W { w: self }
    }
    #[doc = "Bit 14 - Start control bit for the EMC calibration counter. Writing a 1 to this bit begins the measurement process. This bit is cleared automatically when the measurement is complete."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 15 - Measurement completion flag. this bit is set when a calibration measurement is completed. This bit is cleared automatically when the START bit is set."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the calibration counter for programmable delays and returns the result value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emccal](index.html) module"]
pub struct EMCCAL_SPEC;
impl crate::RegisterSpec for EMCCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emccal::R](R) reader structure"]
impl crate::Readable for EMCCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emccal::W](W) writer structure"]
impl crate::Writable for EMCCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCCAL to value 0x1f00"]
impl crate::Resettable for EMCCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f00
    }
}
