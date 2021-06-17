#[doc = "Register `ARGUMENT` reader"]
pub struct R(crate::R<ARGUMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARGUMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARGUMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARGUMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARGUMENT` writer"]
pub struct W(crate::W<ARGUMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARGUMENT_SPEC>;
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
impl From<crate::W<ARGUMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARGUMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CmdArg` reader - Command argument"]
pub struct CMDARG_R(crate::FieldReader<u32, u32>);
impl CMDARG_R {
    pub(crate) fn new(bits: u32) -> Self {
        CMDARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CmdArg` writer - Command argument"]
pub struct CMDARG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CMDARG_R {
        CMDARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmd_arg(&mut self) -> CMDARG_W {
        CMDARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argument](index.html) module"]
pub struct ARGUMENT_SPEC;
impl crate::RegisterSpec for ARGUMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [argument::R](R) reader structure"]
impl crate::Readable for ARGUMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [argument::W](W) writer structure"]
impl crate::Writable for ARGUMENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARGUMENT to value 0"]
impl crate::Resettable for ARGUMENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
