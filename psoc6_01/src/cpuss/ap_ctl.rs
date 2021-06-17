#[doc = "Register `AP_CTL` reader"]
pub struct R(crate::R<AP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AP_CTL` writer"]
pub struct W(crate::W<AP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AP_CTL_SPEC>;
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
impl From<crate::W<AP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM0_ENABLE` reader - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub struct CM0_ENABLE_R(crate::FieldReader<bool, bool>);
impl CM0_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CM0_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM0_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM0_ENABLE` writer - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub struct CM0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM0_ENABLE_W<'a> {
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
#[doc = "Field `CM4_ENABLE` reader - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub struct CM4_ENABLE_R(crate::FieldReader<bool, bool>);
impl CM4_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CM4_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM4_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM4_ENABLE` writer - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub struct CM4_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM4_ENABLE_W<'a> {
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
#[doc = "Field `SYS_ENABLE` reader - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub struct SYS_ENABLE_R(crate::FieldReader<bool, bool>);
impl SYS_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_ENABLE` writer - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub struct SYS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ENABLE_W<'a> {
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
#[doc = "Field `CM0_DISABLE` reader - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub struct CM0_DISABLE_R(crate::FieldReader<bool, bool>);
impl CM0_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CM0_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM0_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM0_DISABLE` writer - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub struct CM0_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM0_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CM4_DISABLE` reader - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub struct CM4_DISABLE_R(crate::FieldReader<bool, bool>);
impl CM4_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CM4_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM4_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM4_DISABLE` writer - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub struct CM4_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CM4_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SYS_DISABLE` reader - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub struct SYS_DISABLE_R(crate::FieldReader<bool, bool>);
impl SYS_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_DISABLE` writer - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub struct SYS_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&self) -> CM0_ENABLE_R {
        CM0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&self) -> CM4_ENABLE_R {
        CM4_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&self) -> SYS_ENABLE_R {
        SYS_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&self) -> CM0_DISABLE_R {
        CM0_DISABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&self) -> CM4_DISABLE_R {
        CM4_DISABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&self) -> SYS_DISABLE_R {
        SYS_DISABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&mut self) -> CM0_ENABLE_W {
        CM0_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&mut self) -> CM4_ENABLE_W {
        CM4_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&mut self) -> SYS_ENABLE_W {
        SYS_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&mut self) -> CM0_DISABLE_W {
        CM0_DISABLE_W { w: self }
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&mut self) -> CM4_DISABLE_W {
        CM4_DISABLE_W { w: self }
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&mut self) -> SYS_DISABLE_W {
        SYS_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access port control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ap_ctl](index.html) module"]
pub struct AP_CTL_SPEC;
impl crate::RegisterSpec for AP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ap_ctl::R](R) reader structure"]
impl crate::Readable for AP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ap_ctl::W](W) writer structure"]
impl crate::Writable for AP_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AP_CTL to value 0"]
impl crate::Resettable for AP_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
