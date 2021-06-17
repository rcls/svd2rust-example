#[doc = "Register `CLK_PILO_CONFIG` reader"]
pub struct R(crate::R<CLK_PILO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PILO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PILO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PILO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PILO_CONFIG` writer"]
pub struct W(crate::W<CLK_PILO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PILO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_PILO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PILO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PILO_FFREQ` reader - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub struct PILO_FFREQ_R(crate::FieldReader<u16, u16>);
impl PILO_FFREQ_R {
    pub(crate) fn new(bits: u16) -> Self {
        PILO_FFREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_FFREQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_FFREQ` writer - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub struct PILO_FFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_FFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `PILO_CLK_EN` reader - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub struct PILO_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PILO_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PILO_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_CLK_EN` writer - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub struct PILO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PILO_RESET_N` reader - Reset the PILO. See PILO_EN field for required sequencing."]
pub struct PILO_RESET_N_R(crate::FieldReader<bool, bool>);
impl PILO_RESET_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        PILO_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_RESET_N` writer - Reset the PILO. See PILO_EN field for required sequencing."]
pub struct PILO_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_RESET_N_W<'a> {
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
#[doc = "Field `PILO_EN` reader - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub struct PILO_EN_R(crate::FieldReader<bool, bool>);
impl PILO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PILO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PILO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PILO_EN` writer - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub struct PILO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_EN_W<'a> {
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
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&self) -> PILO_FFREQ_R {
        PILO_FFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&self) -> PILO_CLK_EN_R {
        PILO_CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&self) -> PILO_RESET_N_R {
        PILO_RESET_N_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&self) -> PILO_EN_R {
        PILO_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&mut self) -> PILO_FFREQ_W {
        PILO_FFREQ_W { w: self }
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&mut self) -> PILO_CLK_EN_W {
        PILO_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&mut self) -> PILO_RESET_N_W {
        PILO_RESET_N_W { w: self }
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&mut self) -> PILO_EN_W {
        PILO_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Precision ILO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pilo_config](index.html) module"]
pub struct CLK_PILO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_PILO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pilo_config::R](R) reader structure"]
impl crate::Readable for CLK_PILO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pilo_config::W](W) writer structure"]
impl crate::Writable for CLK_PILO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PILO_CONFIG to value 0x80"]
impl crate::Resettable for CLK_PILO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
