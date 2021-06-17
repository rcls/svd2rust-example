#[doc = "Register `TRIM_RAM_CTL` reader"]
pub struct R(crate::R<TRIM_RAM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_RAM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_RAM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_RAM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_RAM_CTL` writer"]
pub struct W(crate::W<TRIM_RAM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_RAM_CTL_SPEC>;
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
impl From<crate::W<TRIM_RAM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_RAM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RM` reader - N/A"]
pub struct RM_R(crate::FieldReader<u8, u8>);
impl RM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RM` writer - N/A"]
pub struct RM_W<'a> {
    w: &'a mut W,
}
impl<'a> RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RME` reader - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\]
Read-Write margin setting."]
pub struct RME_R(crate::FieldReader<bool, bool>);
impl RME_R {
    pub(crate) fn new(bits: bool) -> Self {
        RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RME` writer - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\]
Read-Write margin setting."]
pub struct RME_W<'a> {
    w: &'a mut W,
}
impl<'a> RME_W<'a> {
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
#[doc = "Field `WPULSE` reader - Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
pub struct WPULSE_R(crate::FieldReader<u8, u8>);
impl WPULSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WPULSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPULSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPULSE` writer - Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
pub struct WPULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> WPULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `RA` reader - Read Assist control for WL under-drive."]
pub struct RA_R(crate::FieldReader<u8, u8>);
impl RA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Read Assist control for WL under-drive."]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `WA` reader - Write assist enable control (Active High). - WA\\[1:0\\]
Write Assist pins to control negative voltage on SRAM bitline."]
pub struct WA_R(crate::FieldReader<u8, u8>);
impl WA_R {
    pub(crate) fn new(bits: u8) -> Self {
        WA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WA` writer - Write assist enable control (Active High). - WA\\[1:0\\]
Write Assist pins to control negative voltage on SRAM bitline."]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\]
Read-Write margin setting."]
    #[inline(always)]
    pub fn rme(&self) -> RME_R {
        RME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wpulse(&self) -> WPULSE_R {
        WPULSE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Read Assist control for WL under-drive."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Write assist enable control (Active High). - WA\\[1:0\\]
Write Assist pins to control negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W {
        RM_W { w: self }
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\]
Read-Write margin setting."]
    #[inline(always)]
    pub fn rme(&mut self) -> RME_W {
        RME_W { w: self }
    }
    #[doc = "Bits 5:7 - Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wpulse(&mut self) -> WPULSE_W {
        WPULSE_W { w: self }
    }
    #[doc = "Bits 8:9 - Read Assist control for WL under-drive."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bits 12:14 - Write assist enable control (Active High). - WA\\[1:0\\]
Write Assist pins to control negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ram_ctl](index.html) module"]
pub struct TRIM_RAM_CTL_SPEC;
impl crate::RegisterSpec for TRIM_RAM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ram_ctl::R](R) reader structure"]
impl crate::Readable for TRIM_RAM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ram_ctl::W](W) writer structure"]
impl crate::Writable for TRIM_RAM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_RAM_CTL to value 0x6002"]
impl crate::Resettable for TRIM_RAM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6002
    }
}
