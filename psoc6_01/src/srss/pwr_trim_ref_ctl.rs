#[doc = "Register `PWR_TRIM_REF_CTL` reader"]
pub struct R(crate::R<PWR_TRIM_REF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_TRIM_REF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_TRIM_REF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_TRIM_REF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_TRIM_REF_CTL` writer"]
pub struct W(crate::W<PWR_TRIM_REF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_TRIM_REF_CTL_SPEC>;
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
impl From<crate::W<PWR_TRIM_REF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_TRIM_REF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_REF_TCTRIM` reader - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct ACT_REF_TCTRIM_R(crate::FieldReader<u8, u8>);
impl ACT_REF_TCTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_REF_TCTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_TCTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REF_TCTRIM` writer - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct ACT_REF_TCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_TCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ACT_REF_ITRIM` reader - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct ACT_REF_ITRIM_R(crate::FieldReader<u8, u8>);
impl ACT_REF_ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_REF_ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REF_ITRIM` writer - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct ACT_REF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ACT_REF_ABSTRIM` reader - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct ACT_REF_ABSTRIM_R(crate::FieldReader<u8, u8>);
impl ACT_REF_ABSTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_REF_ABSTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_ABSTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REF_ABSTRIM` writer - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct ACT_REF_ABSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_ABSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `ACT_REF_IBOOST` reader - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub struct ACT_REF_IBOOST_R(crate::FieldReader<bool, bool>);
impl ACT_REF_IBOOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACT_REF_IBOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_IBOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REF_IBOOST` writer - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub struct ACT_REF_IBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_IBOOST_W<'a> {
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
#[doc = "Field `DPSLP_REF_TCTRIM` reader - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct DPSLP_REF_TCTRIM_R(crate::FieldReader<u8, u8>);
impl DPSLP_REF_TCTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPSLP_REF_TCTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REF_TCTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REF_TCTRIM` writer - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub struct DPSLP_REF_TCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_TCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DPSLP_REF_ABSTRIM` reader - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub struct DPSLP_REF_ABSTRIM_R(crate::FieldReader<u8, u8>);
impl DPSLP_REF_ABSTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPSLP_REF_ABSTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REF_ABSTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REF_ABSTRIM` writer - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub struct DPSLP_REF_ABSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_ABSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `DPSLP_REF_ITRIM` reader - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub struct DPSLP_REF_ITRIM_R(crate::FieldReader<u8, u8>);
impl DPSLP_REF_ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPSLP_REF_ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_REF_ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP_REF_ITRIM` writer - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub struct DPSLP_REF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&self) -> ACT_REF_TCTRIM_R {
        ACT_REF_TCTRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&self) -> ACT_REF_ITRIM_R {
        ACT_REF_ITRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&self) -> ACT_REF_ABSTRIM_R {
        ACT_REF_ABSTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&self) -> ACT_REF_IBOOST_R {
        ACT_REF_IBOOST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&self) -> DPSLP_REF_TCTRIM_R {
        DPSLP_REF_TCTRIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&self) -> DPSLP_REF_ABSTRIM_R {
        DPSLP_REF_ABSTRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&self) -> DPSLP_REF_ITRIM_R {
        DPSLP_REF_ITRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&mut self) -> ACT_REF_TCTRIM_W {
        ACT_REF_TCTRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&mut self) -> ACT_REF_ITRIM_W {
        ACT_REF_ITRIM_W { w: self }
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&mut self) -> ACT_REF_ABSTRIM_W {
        ACT_REF_ABSTRIM_W { w: self }
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&mut self) -> ACT_REF_IBOOST_W {
        ACT_REF_IBOOST_W { w: self }
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&mut self) -> DPSLP_REF_TCTRIM_W {
        DPSLP_REF_TCTRIM_W { w: self }
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&mut self) -> DPSLP_REF_ABSTRIM_W {
        DPSLP_REF_ABSTRIM_W { w: self }
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&mut self) -> DPSLP_REF_ITRIM_W {
        DPSLP_REF_ITRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_ref_ctl](index.html) module"]
pub struct PWR_TRIM_REF_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_REF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_trim_ref_ctl::R](R) reader structure"]
impl crate::Readable for PWR_TRIM_REF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_trim_ref_ctl::W](W) writer structure"]
impl crate::Writable for PWR_TRIM_REF_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_TRIM_REF_CTL to value 0x70f0_0000"]
impl crate::Resettable for PWR_TRIM_REF_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x70f0_0000
    }
}
