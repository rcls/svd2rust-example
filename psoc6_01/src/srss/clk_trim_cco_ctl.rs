#[doc = "Register `CLK_TRIM_CCO_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_CCO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_CCO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_CCO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_CCO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_CCO_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_CCO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_CCO_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_CCO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_CCO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCO_RCSTRIM` reader - CCO reference current source trim."]
pub struct CCO_RCSTRIM_R(crate::FieldReader<u8, u8>);
impl CCO_RCSTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_RCSTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_RCSTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_RCSTRIM` writer - CCO reference current source trim."]
pub struct CCO_RCSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_RCSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CCO_STABLE_CNT` reader - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub struct CCO_STABLE_CNT_R(crate::FieldReader<u8, u8>);
impl CCO_STABLE_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCO_STABLE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_STABLE_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO_STABLE_CNT` writer - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub struct CCO_STABLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_STABLE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `ENABLE_CNT` reader - Enables the automatic stabilization counter."]
pub struct ENABLE_CNT_R(crate::FieldReader<bool, bool>);
impl ENABLE_CNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_CNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_CNT` writer - Enables the automatic stabilization counter."]
pub struct ENABLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_CNT_W<'a> {
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
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&self) -> CCO_RCSTRIM_R {
        CCO_RCSTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&self) -> CCO_STABLE_CNT_R {
        CCO_STABLE_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&self) -> ENABLE_CNT_R {
        ENABLE_CNT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&mut self) -> CCO_RCSTRIM_W {
        CCO_RCSTRIM_W { w: self }
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&mut self) -> CCO_STABLE_CNT_W {
        CCO_STABLE_CNT_W { w: self }
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&mut self) -> ENABLE_CNT_W {
        ENABLE_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_cco_ctl](index.html) module"]
pub struct CLK_TRIM_CCO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_CCO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_cco_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_cco_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL to value 0xa700_0020"]
impl crate::Resettable for CLK_TRIM_CCO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa700_0020
    }
}
