#[doc = "Register `LLH_FEATURE_CONFIG` reader"]
pub struct R(crate::R<LLH_FEATURE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LLH_FEATURE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LLH_FEATURE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LLH_FEATURE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LLH_FEATURE_CONFIG` writer"]
pub struct W(crate::W<LLH_FEATURE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LLH_FEATURE_CONFIG_SPEC>;
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
impl From<crate::W<LLH_FEATURE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LLH_FEATURE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUICK_TRANSMIT` reader - Quick transmit feature in slave latency is enabled by setting this bit. When slave latency is enabled, this feature enables the slave to transmit in the immediate connection interval, in case required, instead of waiting till the end of slave latency"]
pub struct QUICK_TRANSMIT_R(crate::FieldReader<bool, bool>);
impl QUICK_TRANSMIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        QUICK_TRANSMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUICK_TRANSMIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUICK_TRANSMIT` writer - Quick transmit feature in slave latency is enabled by setting this bit. When slave latency is enabled, this feature enables the slave to transmit in the immediate connection interval, in case required, instead of waiting till the end of slave latency"]
pub struct QUICK_TRANSMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> QUICK_TRANSMIT_W<'a> {
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
#[doc = "Field `SL_DSM_EN` reader - Enable/Disable Slave Latency Period DSM."]
pub struct SL_DSM_EN_R(crate::FieldReader<bool, bool>);
impl SL_DSM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SL_DSM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SL_DSM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SL_DSM_EN` writer - Enable/Disable Slave Latency Period DSM."]
pub struct SL_DSM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_DSM_EN_W<'a> {
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
#[doc = "Field `US_COUNTER_OFFSET_ADJ` reader - Enable/Disable the connection US counter offset adjust. For non-MMMS mode, this bit must be tied to 1."]
pub struct US_COUNTER_OFFSET_ADJ_R(crate::FieldReader<bool, bool>);
impl US_COUNTER_OFFSET_ADJ_R {
    pub(crate) fn new(bits: bool) -> Self {
        US_COUNTER_OFFSET_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for US_COUNTER_OFFSET_ADJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `US_COUNTER_OFFSET_ADJ` writer - Enable/Disable the connection US counter offset adjust. For non-MMMS mode, this bit must be tied to 1."]
pub struct US_COUNTER_OFFSET_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> US_COUNTER_OFFSET_ADJ_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Quick transmit feature in slave latency is enabled by setting this bit. When slave latency is enabled, this feature enables the slave to transmit in the immediate connection interval, in case required, instead of waiting till the end of slave latency"]
    #[inline(always)]
    pub fn quick_transmit(&self) -> QUICK_TRANSMIT_R {
        QUICK_TRANSMIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable/Disable Slave Latency Period DSM."]
    #[inline(always)]
    pub fn sl_dsm_en(&self) -> SL_DSM_EN_R {
        SL_DSM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable/Disable the connection US counter offset adjust. For non-MMMS mode, this bit must be tied to 1."]
    #[inline(always)]
    pub fn us_counter_offset_adj(&self) -> US_COUNTER_OFFSET_ADJ_R {
        US_COUNTER_OFFSET_ADJ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quick transmit feature in slave latency is enabled by setting this bit. When slave latency is enabled, this feature enables the slave to transmit in the immediate connection interval, in case required, instead of waiting till the end of slave latency"]
    #[inline(always)]
    pub fn quick_transmit(&mut self) -> QUICK_TRANSMIT_W {
        QUICK_TRANSMIT_W { w: self }
    }
    #[doc = "Bit 1 - Enable/Disable Slave Latency Period DSM."]
    #[inline(always)]
    pub fn sl_dsm_en(&mut self) -> SL_DSM_EN_W {
        SL_DSM_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable/Disable the connection US counter offset adjust. For non-MMMS mode, this bit must be tied to 1."]
    #[inline(always)]
    pub fn us_counter_offset_adj(&mut self) -> US_COUNTER_OFFSET_ADJ_W {
        US_COUNTER_OFFSET_ADJ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Feature enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [llh_feature_config](index.html) module"]
pub struct LLH_FEATURE_CONFIG_SPEC;
impl crate::RegisterSpec for LLH_FEATURE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [llh_feature_config::R](R) reader structure"]
impl crate::Readable for LLH_FEATURE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [llh_feature_config::W](W) writer structure"]
impl crate::Writable for LLH_FEATURE_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LLH_FEATURE_CONFIG to value 0x06"]
impl crate::Resettable for LLH_FEATURE_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
