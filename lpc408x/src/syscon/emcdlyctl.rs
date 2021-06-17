#[doc = "Register `EMCDLYCTL` reader"]
pub struct R(crate::R<EMCDLYCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMCDLYCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMCDLYCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMCDLYCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMCDLYCTL` writer"]
pub struct W(crate::W<EMCDLYCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMCDLYCTL_SPEC>;
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
impl From<crate::W<EMCDLYCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMCDLYCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDDLY` reader - Programmable delay value for EMC outputs in command delayed mode. See Section 9.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
pub struct CMDDLY_R(crate::FieldReader<u8, u8>);
impl CMDDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDDLY` writer - Programmable delay value for EMC outputs in command delayed mode. See Section 9.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
pub struct CMDDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `FBCLKDLY` reader - Programmable delay value for the feedback clock that controls input data sampling. See Section 9.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
pub struct FBCLKDLY_R(crate::FieldReader<u8, u8>);
impl FBCLKDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        FBCLKDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBCLKDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBCLKDLY` writer - Programmable delay value for the feedback clock that controls input data sampling. See Section 9.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
pub struct FBCLKDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCLKDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `CLKOUT0DLY` reader - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
pub struct CLKOUT0DLY_R(crate::FieldReader<u8, u8>);
impl CLKOUT0DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUT0DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKOUT0DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUT0DLY` writer - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
pub struct CLKOUT0DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT0DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CLKOUT1DLY` reader - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
pub struct CLKOUT1DLY_R(crate::FieldReader<u8, u8>);
impl CLKOUT1DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUT1DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKOUT1DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUT1DLY` writer - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
pub struct CLKOUT1DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT1DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode. See Section 9.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn cmddly(&self) -> CMDDLY_R {
        CMDDLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling. See Section 9.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn fbclkdly(&self) -> FBCLKDLY_R {
        FBCLKDLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn clkout0dly(&self) -> CLKOUT0DLY_R {
        CLKOUT0DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn clkout1dly(&self) -> CLKOUT1DLY_R {
        CLKOUT1DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode. See Section 9.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn cmddly(&mut self) -> CMDDLY_W {
        CMDDLY_W { w: self }
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling. See Section 9.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn fbclkdly(&mut self) -> FBCLKDLY_W {
        FBCLKDLY_W { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
    #[inline(always)]
    pub fn clkout0dly(&mut self) -> CLKOUT0DLY_W {
        CLKOUT0DLY_W { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 9.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
    #[inline(always)]
    pub fn clkout1dly(&mut self) -> CLKOUT1DLY_W {
        CLKOUT1DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Values for the 4 programmable delays associated with SDRAM operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcdlyctl](index.html) module"]
pub struct EMCDLYCTL_SPEC;
impl crate::RegisterSpec for EMCDLYCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emcdlyctl::R](R) reader structure"]
impl crate::Readable for EMCDLYCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emcdlyctl::W](W) writer structure"]
impl crate::Writable for EMCDLYCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMCDLYCTL to value 0x0210"]
impl crate::Resettable for EMCDLYCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210
    }
}
