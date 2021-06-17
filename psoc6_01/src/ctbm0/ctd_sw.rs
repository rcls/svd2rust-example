#[doc = "Register `CTD_SW` reader"]
pub struct R(crate::R<CTD_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTD_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTD_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTD_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTD_SW` writer"]
pub struct W(crate::W<CTD_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTD_SW_SPEC>;
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
impl From<crate::W<CTD_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTD_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTDD_CRD` reader - CTDAC Reference opamp output to ctdrefdrive"]
pub struct CTDD_CRD_R(crate::FieldReader<bool, bool>);
impl CTDD_CRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDD_CRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDD_CRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDD_CRD` writer - CTDAC Reference opamp output to ctdrefdrive"]
pub struct CTDD_CRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDD_CRD_W<'a> {
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
#[doc = "Field `CTDS_CRS` reader - ctdrefsense to opamp input"]
pub struct CTDS_CRS_R(crate::FieldReader<bool, bool>);
impl CTDS_CRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDS_CRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDS_CRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDS_CRS` writer - ctdrefsense to opamp input"]
pub struct CTDS_CRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDS_CRS_W<'a> {
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
#[doc = "Field `CTDS_COR` reader - ctdvout to opamp input"]
pub struct CTDS_COR_R(crate::FieldReader<bool, bool>);
impl CTDS_COR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDS_COR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDS_COR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDS_COR` writer - ctdvout to opamp input"]
pub struct CTDS_COR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDS_COR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CTDO_C6H` reader - P6 pin to Hold capacitor"]
pub struct CTDO_C6H_R(crate::FieldReader<bool, bool>);
impl CTDO_C6H_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDO_C6H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDO_C6H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDO_C6H` writer - P6 pin to Hold capacitor"]
pub struct CTDO_C6H_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDO_C6H_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CTDO_COS` reader - ctdvout to Hold capacitor (Sample switch). Note this switch will temporarily be opened for deglitching if CTDAC.DEGLITCH_COS is set"]
pub struct CTDO_COS_R(crate::FieldReader<bool, bool>);
impl CTDO_COS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDO_COS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDO_COS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDO_COS` writer - ctdvout to Hold capacitor (Sample switch). Note this switch will temporarily be opened for deglitching if CTDAC.DEGLITCH_COS is set"]
pub struct CTDO_COS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDO_COS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CTDH_COB` reader - Drive the CTDAC output with CTBM 1x output during hold mode in Sample and Hold operation"]
pub struct CTDH_COB_R(crate::FieldReader<bool, bool>);
impl CTDH_COB_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDH_COB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDH_COB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDH_COB` writer - Drive the CTDAC output with CTBM 1x output during hold mode in Sample and Hold operation"]
pub struct CTDH_COB_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_COB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CTDH_CHD` reader - Hold capacitor connect"]
pub struct CTDH_CHD_R(crate::FieldReader<bool, bool>);
impl CTDH_CHD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDH_CHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDH_CHD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDH_CHD` writer - Hold capacitor connect"]
pub struct CTDH_CHD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_CHD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CTDH_CA0` reader - Hold capacitor to opamp input"]
pub struct CTDH_CA0_R(crate::FieldReader<bool, bool>);
impl CTDH_CA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDH_CA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDH_CA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDH_CA0` writer - Hold capacitor to opamp input"]
pub struct CTDH_CA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_CA0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CTDH_CIS` reader - Hold capacitor isolation (from all the other switches)"]
pub struct CTDH_CIS_R(crate::FieldReader<bool, bool>);
impl CTDH_CIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDH_CIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDH_CIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDH_CIS` writer - Hold capacitor isolation (from all the other switches)"]
pub struct CTDH_CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_CIS_W<'a> {
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
#[doc = "Field `CTDH_ILR` reader - Hold capacitor leakage reduction (drive other side of CIS to capacitor voltage)"]
pub struct CTDH_ILR_R(crate::FieldReader<bool, bool>);
impl CTDH_ILR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDH_ILR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDH_ILR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDH_ILR` writer - Hold capacitor leakage reduction (drive other side of CIS to capacitor voltage)"]
pub struct CTDH_ILR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_ILR_W<'a> {
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
    #[doc = "Bit 1 - CTDAC Reference opamp output to ctdrefdrive"]
    #[inline(always)]
    pub fn ctdd_crd(&self) -> CTDD_CRD_R {
        CTDD_CRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ctdrefsense to opamp input"]
    #[inline(always)]
    pub fn ctds_crs(&self) -> CTDS_CRS_R {
        CTDS_CRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ctdvout to opamp input"]
    #[inline(always)]
    pub fn ctds_cor(&self) -> CTDS_COR_R {
        CTDS_COR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - P6 pin to Hold capacitor"]
    #[inline(always)]
    pub fn ctdo_c6h(&self) -> CTDO_C6H_R {
        CTDO_C6H_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ctdvout to Hold capacitor (Sample switch). Note this switch will temporarily be opened for deglitching if CTDAC.DEGLITCH_COS is set"]
    #[inline(always)]
    pub fn ctdo_cos(&self) -> CTDO_COS_R {
        CTDO_COS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Drive the CTDAC output with CTBM 1x output during hold mode in Sample and Hold operation"]
    #[inline(always)]
    pub fn ctdh_cob(&self) -> CTDH_COB_R {
        CTDH_COB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hold capacitor connect"]
    #[inline(always)]
    pub fn ctdh_chd(&self) -> CTDH_CHD_R {
        CTDH_CHD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Hold capacitor to opamp input"]
    #[inline(always)]
    pub fn ctdh_ca0(&self) -> CTDH_CA0_R {
        CTDH_CA0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Hold capacitor isolation (from all the other switches)"]
    #[inline(always)]
    pub fn ctdh_cis(&self) -> CTDH_CIS_R {
        CTDH_CIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Hold capacitor leakage reduction (drive other side of CIS to capacitor voltage)"]
    #[inline(always)]
    pub fn ctdh_ilr(&self) -> CTDH_ILR_R {
        CTDH_ILR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CTDAC Reference opamp output to ctdrefdrive"]
    #[inline(always)]
    pub fn ctdd_crd(&mut self) -> CTDD_CRD_W {
        CTDD_CRD_W { w: self }
    }
    #[doc = "Bit 4 - ctdrefsense to opamp input"]
    #[inline(always)]
    pub fn ctds_crs(&mut self) -> CTDS_CRS_W {
        CTDS_CRS_W { w: self }
    }
    #[doc = "Bit 5 - ctdvout to opamp input"]
    #[inline(always)]
    pub fn ctds_cor(&mut self) -> CTDS_COR_W {
        CTDS_COR_W { w: self }
    }
    #[doc = "Bit 8 - P6 pin to Hold capacitor"]
    #[inline(always)]
    pub fn ctdo_c6h(&mut self) -> CTDO_C6H_W {
        CTDO_C6H_W { w: self }
    }
    #[doc = "Bit 9 - ctdvout to Hold capacitor (Sample switch). Note this switch will temporarily be opened for deglitching if CTDAC.DEGLITCH_COS is set"]
    #[inline(always)]
    pub fn ctdo_cos(&mut self) -> CTDO_COS_W {
        CTDO_COS_W { w: self }
    }
    #[doc = "Bit 10 - Drive the CTDAC output with CTBM 1x output during hold mode in Sample and Hold operation"]
    #[inline(always)]
    pub fn ctdh_cob(&mut self) -> CTDH_COB_W {
        CTDH_COB_W { w: self }
    }
    #[doc = "Bit 12 - Hold capacitor connect"]
    #[inline(always)]
    pub fn ctdh_chd(&mut self) -> CTDH_CHD_W {
        CTDH_CHD_W { w: self }
    }
    #[doc = "Bit 13 - Hold capacitor to opamp input"]
    #[inline(always)]
    pub fn ctdh_ca0(&mut self) -> CTDH_CA0_W {
        CTDH_CA0_W { w: self }
    }
    #[doc = "Bit 14 - Hold capacitor isolation (from all the other switches)"]
    #[inline(always)]
    pub fn ctdh_cis(&mut self) -> CTDH_CIS_W {
        CTDH_CIS_W { w: self }
    }
    #[doc = "Bit 15 - Hold capacitor leakage reduction (drive other side of CIS to capacitor voltage)"]
    #[inline(always)]
    pub fn ctdh_ilr(&mut self) -> CTDH_ILR_W {
        CTDH_ILR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTDAC connection switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctd_sw](index.html) module"]
pub struct CTD_SW_SPEC;
impl crate::RegisterSpec for CTD_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctd_sw::R](R) reader structure"]
impl crate::Readable for CTD_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctd_sw::W](W) writer structure"]
impl crate::Writable for CTD_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTD_SW to value 0"]
impl crate::Resettable for CTD_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
