#[doc = "Register `LUT_CTL[%s]` reader"]
pub struct R(crate::R<LUT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT_CTL[%s]` writer"]
pub struct W(crate::W<LUT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_CTL_SPEC>;
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
impl From<crate::W<LUT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT` reader - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
pub struct LUT_R(crate::FieldReader<u8, u8>);
impl LUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT` writer - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
pub struct LUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LUT_OPC` reader - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
pub struct LUT_OPC_R(crate::FieldReader<u8, u8>);
impl LUT_OPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LUT_OPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_OPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_OPC` writer - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
pub struct LUT_OPC_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_OPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&self) -> LUT_R {
        LUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&self) -> LUT_OPC_R {
        LUT_OPC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&mut self) -> LUT_W {
        LUT_W { w: self }
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&mut self) -> LUT_OPC_W {
        LUT_OPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT component control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_ctl](index.html) module"]
pub struct LUT_CTL_SPEC;
impl crate::RegisterSpec for LUT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut_ctl::R](R) reader structure"]
impl crate::Readable for LUT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut_ctl::W](W) writer structure"]
impl crate::Writable for LUT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT_CTL[%s]
to value 0"]
impl crate::Resettable for LUT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
