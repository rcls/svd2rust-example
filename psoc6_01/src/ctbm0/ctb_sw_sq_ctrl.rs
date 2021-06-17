#[doc = "Register `CTB_SW_SQ_CTRL` reader"]
pub struct R(crate::R<CTB_SW_SQ_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTB_SW_SQ_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTB_SW_SQ_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTB_SW_SQ_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTB_SW_SQ_CTRL` writer"]
pub struct W(crate::W<CTB_SW_SQ_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTB_SW_SQ_CTRL_SPEC>;
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
impl From<crate::W<CTB_SW_SQ_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTB_SW_SQ_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2_SQ_CTRL23` reader - for D51"]
pub struct P2_SQ_CTRL23_R(crate::FieldReader<bool, bool>);
impl P2_SQ_CTRL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_SQ_CTRL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_SQ_CTRL23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_SQ_CTRL23` writer - for D51"]
pub struct P2_SQ_CTRL23_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_SQ_CTRL23_W<'a> {
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
#[doc = "Field `P3_SQ_CTRL23` reader - for D52, D62"]
pub struct P3_SQ_CTRL23_R(crate::FieldReader<bool, bool>);
impl P3_SQ_CTRL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3_SQ_CTRL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3_SQ_CTRL23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3_SQ_CTRL23` writer - for D52, D62"]
pub struct P3_SQ_CTRL23_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_SQ_CTRL23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - for D51"]
    #[inline(always)]
    pub fn p2_sq_ctrl23(&self) -> P2_SQ_CTRL23_R {
        P2_SQ_CTRL23_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - for D52, D62"]
    #[inline(always)]
    pub fn p3_sq_ctrl23(&self) -> P3_SQ_CTRL23_R {
        P3_SQ_CTRL23_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - for D51"]
    #[inline(always)]
    pub fn p2_sq_ctrl23(&mut self) -> P2_SQ_CTRL23_W {
        P2_SQ_CTRL23_W { w: self }
    }
    #[doc = "Bit 11 - for D52, D62"]
    #[inline(always)]
    pub fn p3_sq_ctrl23(&mut self) -> P3_SQ_CTRL23_W {
        P3_SQ_CTRL23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTB bus switch Sar Sequencer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_sw_sq_ctrl](index.html) module"]
pub struct CTB_SW_SQ_CTRL_SPEC;
impl crate::RegisterSpec for CTB_SW_SQ_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctb_sw_sq_ctrl::R](R) reader structure"]
impl crate::Readable for CTB_SW_SQ_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctb_sw_sq_ctrl::W](W) writer structure"]
impl crate::Writable for CTB_SW_SQ_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTB_SW_SQ_CTRL to value 0"]
impl crate::Resettable for CTB_SW_SQ_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
