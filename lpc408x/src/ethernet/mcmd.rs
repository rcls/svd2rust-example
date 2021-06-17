#[doc = "Register `MCMD` reader"]
pub struct R(crate::R<MCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCMD` writer"]
pub struct W(crate::W<MCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMD_SPEC>;
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
impl From<crate::W<MCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ` reader - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
pub struct READ_R(crate::FieldReader<bool, bool>);
impl READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ` writer - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
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
#[doc = "Field `SCAN` reader - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
pub struct SCAN_R(crate::FieldReader<bool, bool>);
impl SCAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN` writer - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
pub struct SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Mgmt Command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmd](index.html) module"]
pub struct MCMD_SPEC;
impl crate::RegisterSpec for MCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcmd::R](R) reader structure"]
impl crate::Readable for MCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcmd::W](W) writer structure"]
impl crate::Writable for MCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCMD to value 0"]
impl crate::Resettable for MCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
