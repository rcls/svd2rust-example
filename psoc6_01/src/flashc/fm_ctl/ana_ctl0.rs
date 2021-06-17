#[doc = "Register `ANA_CTL0` reader"]
pub struct R(crate::R<ANA_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CTL0` writer"]
pub struct W(crate::W<ANA_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CTL0_SPEC>;
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
impl From<crate::W<ANA_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSLDAC` reader - Trimming of common source line DAC."]
pub struct CSLDAC_R(crate::FieldReader<u8, u8>);
impl CSLDAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSLDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSLDAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSLDAC` writer - Trimming of common source line DAC."]
pub struct CSLDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSLDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `VCC_SEL` reader - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
pub struct VCC_SEL_R(crate::FieldReader<bool, bool>);
impl VCC_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCC_SEL` writer - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
pub struct VCC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `FLIP_AMUXBUS_AB` reader - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
pub struct FLIP_AMUXBUS_AB_R(crate::FieldReader<bool, bool>);
impl FLIP_AMUXBUS_AB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIP_AMUXBUS_AB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLIP_AMUXBUS_AB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIP_AMUXBUS_AB` writer - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
pub struct FLIP_AMUXBUS_AB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIP_AMUXBUS_AB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&self) -> CSLDAC_R {
        CSLDAC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn vcc_sel(&self) -> VCC_SEL_R {
        VCC_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&self) -> FLIP_AMUXBUS_AB_R {
        FLIP_AMUXBUS_AB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&mut self) -> CSLDAC_W {
        CSLDAC_W { w: self }
    }
    #[doc = "Bit 24 - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn vcc_sel(&mut self) -> VCC_SEL_W {
        VCC_SEL_W { w: self }
    }
    #[doc = "Bit 27 - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&mut self) -> FLIP_AMUXBUS_AB_W {
        FLIP_AMUXBUS_AB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl0](index.html) module"]
pub struct ANA_CTL0_SPEC;
impl crate::RegisterSpec for ANA_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_ctl0::R](R) reader structure"]
impl crate::Readable for ANA_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_ctl0::W](W) writer structure"]
impl crate::Writable for ANA_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_CTL0 to value 0x0400"]
impl crate::Resettable for ANA_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
