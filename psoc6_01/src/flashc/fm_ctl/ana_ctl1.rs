#[doc = "Register `ANA_CTL1` reader"]
pub struct R(crate::R<ANA_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CTL1` writer"]
pub struct W(crate::W<ANA_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CTL1_SPEC>;
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
impl From<crate::W<ANA_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDAC` reader - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub struct MDAC_R(crate::FieldReader<u8, u8>);
impl MDAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDAC` writer - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub struct MDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PDAC` reader - Trimming of positive pump output Voltage:"]
pub struct PDAC_R(crate::FieldReader<u8, u8>);
impl PDAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC` writer - Trimming of positive pump output Voltage:"]
pub struct PDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `NDAC` reader - Trimming of negative pump output Voltage:"]
pub struct NDAC_R(crate::FieldReader<u8, u8>);
impl NDAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDAC` writer - Trimming of negative pump output Voltage:"]
pub struct NDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `VPROT_OVERRIDE` reader - '0': vprot = BG.vprot. '1': vprot = vcc"]
pub struct VPROT_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl VPROT_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VPROT_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPROT_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPROT_OVERRIDE` writer - '0': vprot = BG.vprot. '1': vprot = vcc"]
pub struct VPROT_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VPROT_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `R_GRANT_CTL` reader - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
pub struct R_GRANT_CTL_R(crate::FieldReader<bool, bool>);
impl R_GRANT_CTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        R_GRANT_CTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_GRANT_CTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_GRANT_CTL` writer - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
pub struct R_GRANT_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> R_GRANT_CTL_W<'a> {
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
#[doc = "Field `RST_SFT_HVPL` reader - '1': Page Latches Soft Reset"]
pub struct RST_SFT_HVPL_R(crate::FieldReader<bool, bool>);
impl RST_SFT_HVPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RST_SFT_HVPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_SFT_HVPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_SFT_HVPL` writer - '1': Page Latches Soft Reset"]
pub struct RST_SFT_HVPL_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_SFT_HVPL_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&self) -> MDAC_R {
        MDAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac(&self) -> PDAC_R {
        PDAC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn ndac(&self) -> NDAC_R {
        NDAC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - '0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn vprot_override(&self) -> VPROT_OVERRIDE_R {
        VPROT_OVERRIDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn r_grant_ctl(&self) -> R_GRANT_CTL_R {
        R_GRANT_CTL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - '1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn rst_sft_hvpl(&self) -> RST_SFT_HVPL_R {
        RST_SFT_HVPL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&mut self) -> MDAC_W {
        MDAC_W { w: self }
    }
    #[doc = "Bits 16:19 - Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac(&mut self) -> PDAC_W {
        PDAC_W { w: self }
    }
    #[doc = "Bits 24:27 - Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn ndac(&mut self) -> NDAC_W {
        NDAC_W { w: self }
    }
    #[doc = "Bit 28 - '0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn vprot_override(&mut self) -> VPROT_OVERRIDE_W {
        VPROT_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 29 - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn r_grant_ctl(&mut self) -> R_GRANT_CTL_W {
        R_GRANT_CTL_W { w: self }
    }
    #[doc = "Bit 30 - '1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn rst_sft_hvpl(&mut self) -> RST_SFT_HVPL_W {
        RST_SFT_HVPL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl1](index.html) module"]
pub struct ANA_CTL1_SPEC;
impl crate::RegisterSpec for ANA_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_ctl1::R](R) reader structure"]
impl crate::Readable for ANA_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_ctl1::W](W) writer structure"]
impl crate::Writable for ANA_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_CTL1 to value 0x0606_0000"]
impl crate::Resettable for ANA_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0606_0000
    }
}
