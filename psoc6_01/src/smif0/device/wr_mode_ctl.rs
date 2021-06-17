#[doc = "Register `WR_MODE_CTL` reader"]
pub struct R(crate::R<WR_MODE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_MODE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_MODE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_MODE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_MODE_CTL` writer"]
pub struct W(crate::W<WR_MODE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_MODE_CTL_SPEC>;
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
impl From<crate::W<WR_MODE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_MODE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Mode byte code."]
pub struct CODE_R(crate::FieldReader<u8, u8>);
impl CODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODE` writer - Mode byte code."]
pub struct CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub struct WIDTH_R(crate::FieldReader<u8, u8>);
impl WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PRESENT` reader - Presence of mode field: '0': not present '1': present"]
pub struct PRESENT_R(crate::FieldReader<bool, bool>);
impl PRESENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESENT` writer - Presence of mode field: '0': not present '1': present"]
pub struct PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESENT_W<'a> {
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
    #[doc = "Bits 0:7 - Mode byte code."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mode byte code."]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W {
        CODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bit 31 - Presence of mode field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&mut self) -> PRESENT_W {
        PRESENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_mode_ctl](index.html) module"]
pub struct WR_MODE_CTL_SPEC;
impl crate::RegisterSpec for WR_MODE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_mode_ctl::R](R) reader structure"]
impl crate::Readable for WR_MODE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_mode_ctl::W](W) writer structure"]
impl crate::Writable for WR_MODE_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR_MODE_CTL to value 0"]
impl crate::Resettable for WR_MODE_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
