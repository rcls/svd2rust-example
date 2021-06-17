#[doc = "Register `CM4_NMI_CTL` reader"]
pub struct R(crate::R<CM4_NMI_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_NMI_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_NMI_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_NMI_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM4_NMI_CTL` writer"]
pub struct W(crate::W<CM4_NMI_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM4_NMI_CTL_SPEC>;
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
impl From<crate::W<CM4_NMI_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM4_NMI_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX0_SEL` reader - System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub struct MUX0_SEL_R(crate::FieldReader<u8, u8>);
impl MUX0_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUX0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX0_SEL` writer - System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub struct MUX0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn mux0_sel(&self) -> MUX0_SEL_R {
        MUX0_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System interrupt select for CPU NMI. The reset value ensure that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn mux0_sel(&mut self) -> MUX0_SEL_W {
        MUX0_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM4 NMI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_nmi_ctl](index.html) module"]
pub struct CM4_NMI_CTL_SPEC;
impl crate::RegisterSpec for CM4_NMI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_nmi_ctl::R](R) reader structure"]
impl crate::Readable for CM4_NMI_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm4_nmi_ctl::W](W) writer structure"]
impl crate::Writable for CM4_NMI_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM4_NMI_CTL to value 0xf0"]
impl crate::Resettable for CM4_NMI_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
