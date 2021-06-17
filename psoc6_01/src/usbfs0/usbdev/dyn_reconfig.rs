#[doc = "Register `DYN_RECONFIG` reader"]
pub struct R(crate::R<DYN_RECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYN_RECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYN_RECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYN_RECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYN_RECONFIG` writer"]
pub struct W(crate::W<DYN_RECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYN_RECONFIG_SPEC>;
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
impl From<crate::W<DYN_RECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYN_RECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DYN_CONFIG_EN` reader - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
pub struct DYN_CONFIG_EN_R(crate::FieldReader<bool, bool>);
impl DYN_CONFIG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DYN_CONFIG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DYN_CONFIG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DYN_CONFIG_EN` writer - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
pub struct DYN_CONFIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_CONFIG_EN_W<'a> {
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
#[doc = "Field `DYN_RECONFIG_EPNO` reader - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
pub struct DYN_RECONFIG_EPNO_R(crate::FieldReader<u8, u8>);
impl DYN_RECONFIG_EPNO_R {
    pub(crate) fn new(bits: u8) -> Self {
        DYN_RECONFIG_EPNO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DYN_RECONFIG_EPNO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DYN_RECONFIG_EPNO` writer - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
pub struct DYN_RECONFIG_EPNO_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_RECONFIG_EPNO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `DYN_RECONFIG_RDY_STS` reader - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
pub struct DYN_RECONFIG_RDY_STS_R(crate::FieldReader<bool, bool>);
impl DYN_RECONFIG_RDY_STS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DYN_RECONFIG_RDY_STS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DYN_RECONFIG_RDY_STS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn dyn_config_en(&self) -> DYN_CONFIG_EN_R {
        DYN_CONFIG_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn dyn_reconfig_epno(&self) -> DYN_RECONFIG_EPNO_R {
        DYN_RECONFIG_EPNO_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
    #[inline(always)]
    pub fn dyn_reconfig_rdy_sts(&self) -> DYN_RECONFIG_RDY_STS_R {
        DYN_RECONFIG_RDY_STS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn dyn_config_en(&mut self) -> DYN_CONFIG_EN_W {
        DYN_CONFIG_EN_W { w: self }
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn dyn_reconfig_epno(&mut self) -> DYN_RECONFIG_EPNO_W {
        DYN_RECONFIG_EPNO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Dynamic reconfiguration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dyn_reconfig](index.html) module"]
pub struct DYN_RECONFIG_SPEC;
impl crate::RegisterSpec for DYN_RECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dyn_reconfig::R](R) reader structure"]
impl crate::Readable for DYN_RECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dyn_reconfig::W](W) writer structure"]
impl crate::Writable for DYN_RECONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYN_RECONFIG to value 0"]
impl crate::Resettable for DYN_RECONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
