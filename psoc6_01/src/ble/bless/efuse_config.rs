#[doc = "Register `EFUSE_CONFIG` reader"]
pub struct R(crate::R<EFUSE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE_CONFIG` writer"]
pub struct W(crate::W<EFUSE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_CONFIG_SPEC>;
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
impl From<crate::W<EFUSE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSE_MODE` reader - This register enables the efuse mode in m0s8bless_ver3"]
pub struct EFUSE_MODE_R(crate::FieldReader<bool, bool>);
impl EFUSE_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_MODE` writer - This register enables the efuse mode in m0s8bless_ver3"]
pub struct EFUSE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_MODE_W<'a> {
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
#[doc = "Field `EFUSE_READ` reader - This bit when set by firmware enables the read from EFUSE macro. It is cleared when the efuse read is completed"]
pub struct EFUSE_READ_R(crate::FieldReader<bool, bool>);
impl EFUSE_READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_READ` writer - This bit when set by firmware enables the read from EFUSE macro. It is cleared when the efuse read is completed"]
pub struct EFUSE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_READ_W<'a> {
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
#[doc = "Field `EFUSE_WRITE` reader - This bit when set by firmware enables the write to EFUSE macro. It is cleared when the efuse write is completed"]
pub struct EFUSE_WRITE_R(crate::FieldReader<bool, bool>);
impl EFUSE_WRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_WRITE` writer - This bit when set by firmware enables the write to EFUSE macro. It is cleared when the efuse write is completed"]
pub struct EFUSE_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_WRITE_W<'a> {
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
    #[doc = "Bit 0 - This register enables the efuse mode in m0s8bless_ver3"]
    #[inline(always)]
    pub fn efuse_mode(&self) -> EFUSE_MODE_R {
        EFUSE_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit when set by firmware enables the read from EFUSE macro. It is cleared when the efuse read is completed"]
    #[inline(always)]
    pub fn efuse_read(&self) -> EFUSE_READ_R {
        EFUSE_READ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit when set by firmware enables the write to EFUSE macro. It is cleared when the efuse write is completed"]
    #[inline(always)]
    pub fn efuse_write(&self) -> EFUSE_WRITE_R {
        EFUSE_WRITE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register enables the efuse mode in m0s8bless_ver3"]
    #[inline(always)]
    pub fn efuse_mode(&mut self) -> EFUSE_MODE_W {
        EFUSE_MODE_W { w: self }
    }
    #[doc = "Bit 1 - This bit when set by firmware enables the read from EFUSE macro. It is cleared when the efuse read is completed"]
    #[inline(always)]
    pub fn efuse_read(&mut self) -> EFUSE_READ_W {
        EFUSE_READ_W { w: self }
    }
    #[doc = "Bit 2 - This bit when set by firmware enables the write to EFUSE macro. It is cleared when the efuse write is completed"]
    #[inline(always)]
    pub fn efuse_write(&mut self) -> EFUSE_WRITE_W {
        EFUSE_WRITE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_config](index.html) module"]
pub struct EFUSE_CONFIG_SPEC;
impl crate::RegisterSpec for EFUSE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_config::R](R) reader structure"]
impl crate::Readable for EFUSE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse_config::W](W) writer structure"]
impl crate::Writable for EFUSE_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSE_CONFIG to value 0"]
impl crate::Resettable for EFUSE_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
