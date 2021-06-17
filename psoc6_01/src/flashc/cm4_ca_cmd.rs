#[doc = "Register `CM4_CA_CMD` reader"]
pub struct R(crate::R<CM4_CA_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_CA_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_CA_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_CA_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM4_CA_CMD` writer"]
pub struct W(crate::W<CM4_CA_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM4_CA_CMD_SPEC>;
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
impl From<crate::W<CM4_CA_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM4_CA_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - See CM0_CA_CMD."]
pub struct INV_R(crate::FieldReader<bool, bool>);
impl INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - See CM0_CA_CMD."]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
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
impl R {
    #[doc = "Bit 0 - See CM0_CA_CMD."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CM0_CA_CMD."]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM4 cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_cmd](index.html) module"]
pub struct CM4_CA_CMD_SPEC;
impl crate::RegisterSpec for CM4_CA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_ca_cmd::R](R) reader structure"]
impl crate::Readable for CM4_CA_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm4_ca_cmd::W](W) writer structure"]
impl crate::Writable for CM4_CA_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM4_CA_CMD to value 0"]
impl crate::Resettable for CM4_CA_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
