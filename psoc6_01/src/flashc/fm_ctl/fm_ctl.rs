#[doc = "Register `FM_CTL` reader"]
pub struct R(crate::R<FM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_CTL` writer"]
pub struct W(crate::W<FM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_CTL_SPEC>;
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
impl From<crate::W<FM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FM_MODE` reader - Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
pub struct FM_MODE_R(crate::FieldReader<u8, u8>);
impl FM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FM_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FM_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FM_MODE` writer - Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
pub struct FM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `FM_SEQ` reader - Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
pub struct FM_SEQ_R(crate::FieldReader<u8, u8>);
impl FM_SEQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        FM_SEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FM_SEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FM_SEQ` writer - Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
pub struct FM_SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_SEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DAA_MUX_SEL` reader - Direct memory cell access address."]
pub struct DAA_MUX_SEL_R(crate::FieldReader<u8, u8>);
impl DAA_MUX_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAA_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAA_MUX_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAA_MUX_SEL` writer - Direct memory cell access address."]
pub struct DAA_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAA_MUX_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `IF_SEL` reader - Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
pub struct IF_SEL_R(crate::FieldReader<bool, bool>);
impl IF_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_SEL` writer - Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
pub struct IF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_SEL_W<'a> {
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
#[doc = "Field `WR_EN` reader - '0': normal mode '1': Fm Write Enable"]
pub struct WR_EN_R(crate::FieldReader<bool, bool>);
impl WR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_EN` writer - '0': normal mode '1': Fm Write Enable"]
pub struct WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
    #[inline(always)]
    pub fn fm_mode(&self) -> FM_MODE_R {
        FM_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
    #[inline(always)]
    pub fn fm_seq(&self) -> FM_SEQ_R {
        FM_SEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&self) -> DAA_MUX_SEL_R {
        DAA_MUX_SEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
    #[inline(always)]
    pub fn if_sel(&self) -> IF_SEL_R {
        IF_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - '0': normal mode '1': Fm Write Enable"]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
    #[inline(always)]
    pub fn fm_mode(&mut self) -> FM_MODE_W {
        FM_MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
    #[inline(always)]
    pub fn fm_seq(&mut self) -> FM_SEQ_W {
        FM_SEQ_W { w: self }
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&mut self) -> DAA_MUX_SEL_W {
        DAA_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
    #[inline(always)]
    pub fn if_sel(&mut self) -> IF_SEL_W {
        IF_SEL_W { w: self }
    }
    #[doc = "Bit 25 - '0': normal mode '1': Fm Write Enable"]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WR_EN_W {
        WR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash macro control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_ctl](index.html) module"]
pub struct FM_CTL_SPEC;
impl crate::RegisterSpec for FM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_ctl::R](R) reader structure"]
impl crate::Readable for FM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_ctl::W](W) writer structure"]
impl crate::Writable for FM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM_CTL to value 0"]
impl crate::Resettable for FM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
