#[doc = "Register `EXT_PA_LNA_DLY_CNFG` reader"]
pub struct R(crate::R<EXT_PA_LNA_DLY_CNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_PA_LNA_DLY_CNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_PA_LNA_DLY_CNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_PA_LNA_DLY_CNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_PA_LNA_DLY_CNFG` writer"]
pub struct W(crate::W<EXT_PA_LNA_DLY_CNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_PA_LNA_DLY_CNFG_SPEC>;
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
impl From<crate::W<EXT_PA_LNA_DLY_CNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_PA_LNA_DLY_CNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNA_CTL_DELAY` reader - The delay used to assert LNA_CTL, LNA_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the external Low Noise Amplifier. The value to be programmed to the lna_ctl_delay \\[7:0\\]
= rx_on_delay - LNA_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) LNA_tRamp = External Low Noise Amplifier startup time"]
pub struct LNA_CTL_DELAY_R(crate::FieldReader<u8, u8>);
impl LNA_CTL_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_CTL_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_CTL_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_CTL_DELAY` writer - The delay used to assert LNA_CTL, LNA_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the external Low Noise Amplifier. The value to be programmed to the lna_ctl_delay \\[7:0\\]
= rx_on_delay - LNA_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) LNA_tRamp = External Low Noise Amplifier startup time"]
pub struct LNA_CTL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CTL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PA_CTL_DELAY` reader - The delay used to assert PA_CTL exactly PA_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the external power amplifier. The value to be programmed to the pa_ctl_delay \\[7:0\\]
= tx_on_delay - PA_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) PA_tRamp = External Power Amplifier ramp time"]
pub struct PA_CTL_DELAY_R(crate::FieldReader<u8, u8>);
impl PA_CTL_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_CTL_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_CTL_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_CTL_DELAY` writer - The delay used to assert PA_CTL exactly PA_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the external power amplifier. The value to be programmed to the pa_ctl_delay \\[7:0\\]
= tx_on_delay - PA_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) PA_tRamp = External Power Amplifier ramp time"]
pub struct PA_CTL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CTL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The delay used to assert LNA_CTL, LNA_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the external Low Noise Amplifier. The value to be programmed to the lna_ctl_delay \\[7:0\\]
= rx_on_delay - LNA_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) LNA_tRamp = External Low Noise Amplifier startup time"]
    #[inline(always)]
    pub fn lna_ctl_delay(&self) -> LNA_CTL_DELAY_R {
        LNA_CTL_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The delay used to assert PA_CTL exactly PA_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the external power amplifier. The value to be programmed to the pa_ctl_delay \\[7:0\\]
= tx_on_delay - PA_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) PA_tRamp = External Power Amplifier ramp time"]
    #[inline(always)]
    pub fn pa_ctl_delay(&self) -> PA_CTL_DELAY_R {
        PA_CTL_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The delay used to assert LNA_CTL, LNA_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the external Low Noise Amplifier. The value to be programmed to the lna_ctl_delay \\[7:0\\]
= rx_on_delay - LNA_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) LNA_tRamp = External Low Noise Amplifier startup time"]
    #[inline(always)]
    pub fn lna_ctl_delay(&mut self) -> LNA_CTL_DELAY_W {
        LNA_CTL_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - The delay used to assert PA_CTL exactly PA_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the external power amplifier. The value to be programmed to the pa_ctl_delay \\[7:0\\]
= tx_on_delay - PA_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) PA_tRamp = External Power Amplifier ramp time"]
    #[inline(always)]
    pub fn pa_ctl_delay(&mut self) -> PA_CTL_DELAY_W {
        PA_CTL_DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External TX PA and RX LNA delay configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_pa_lna_dly_cnfg](index.html) module"]
pub struct EXT_PA_LNA_DLY_CNFG_SPEC;
impl crate::RegisterSpec for EXT_PA_LNA_DLY_CNFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_pa_lna_dly_cnfg::R](R) reader structure"]
impl crate::Readable for EXT_PA_LNA_DLY_CNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_pa_lna_dly_cnfg::W](W) writer structure"]
impl crate::Writable for EXT_PA_LNA_DLY_CNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_PA_LNA_DLY_CNFG to value 0"]
impl crate::Resettable for EXT_PA_LNA_DLY_CNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
