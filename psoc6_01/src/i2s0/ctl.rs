#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ENABLED` reader - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub struct TX_ENABLED_R(crate::FieldReader<bool, bool>);
impl TX_ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ENABLED` writer - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub struct TX_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `RX_ENABLED` reader - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub struct RX_ENABLED_R(crate::FieldReader<bool, bool>);
impl RX_ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ENABLED` writer - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub struct RX_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&self) -> TX_ENABLED_R {
        TX_ENABLED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&self) -> RX_ENABLED_R {
        RX_ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&mut self) -> TX_ENABLED_W {
        TX_ENABLED_W { w: self }
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&mut self) -> RX_ENABLED_W {
        RX_ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
