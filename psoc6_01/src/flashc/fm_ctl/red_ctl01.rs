#[doc = "Register `RED_CTL01` reader"]
pub struct R(crate::R<RED_CTL01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL01` writer"]
pub struct W(crate::W<RED_CTL01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL01_SPEC>;
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
impl From<crate::W<RED_CTL01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RED_ADDR_0` reader - Bad Row Pair Address for Sector 0"]
pub struct RED_ADDR_0_R(crate::FieldReader<u8, u8>);
impl RED_ADDR_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RED_ADDR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_ADDR_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED_ADDR_0` writer - Bad Row Pair Address for Sector 0"]
pub struct RED_ADDR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RED_EN_0` reader - '1': Redundancy Enable for Sector 0"]
pub struct RED_EN_0_R(crate::FieldReader<bool, bool>);
impl RED_EN_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RED_EN_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_EN_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED_EN_0` writer - '1': Redundancy Enable for Sector 0"]
pub struct RED_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_0_W<'a> {
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
#[doc = "Field `RED_ADDR_1` reader - Bad Row Pair Address for Sector 1"]
pub struct RED_ADDR_1_R(crate::FieldReader<u8, u8>);
impl RED_ADDR_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RED_ADDR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_ADDR_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED_ADDR_1` writer - Bad Row Pair Address for Sector 1"]
pub struct RED_ADDR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RED_EN_1` reader - '1': Redundancy Enable for Sector 1"]
pub struct RED_EN_1_R(crate::FieldReader<bool, bool>);
impl RED_EN_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RED_EN_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_EN_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED_EN_1` writer - '1': Redundancy Enable for Sector 1"]
pub struct RED_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn red_addr_0(&self) -> RED_ADDR_0_R {
        RED_ADDR_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - '1': Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn red_en_0(&self) -> RED_EN_0_R {
        RED_EN_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn red_addr_1(&self) -> RED_ADDR_1_R {
        RED_ADDR_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - '1': Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn red_en_1(&self) -> RED_EN_1_R {
        RED_EN_1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn red_addr_0(&mut self) -> RED_ADDR_0_W {
        RED_ADDR_0_W { w: self }
    }
    #[doc = "Bit 8 - '1': Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn red_en_0(&mut self) -> RED_EN_0_W {
        RED_EN_0_W { w: self }
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn red_addr_1(&mut self) -> RED_ADDR_1_W {
        RED_ADDR_1_W { w: self }
    }
    #[doc = "Bit 24 - '1': Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn red_en_1(&mut self) -> RED_EN_1_W {
        RED_EN_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Control normal sectors 0,1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl01](index.html) module"]
pub struct RED_CTL01_SPEC;
impl crate::RegisterSpec for RED_CTL01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl01::R](R) reader structure"]
impl crate::Readable for RED_CTL01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl01::W](W) writer structure"]
impl crate::Writable for RED_CTL01_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RED_CTL01 to value 0"]
impl crate::Resettable for RED_CTL01_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
