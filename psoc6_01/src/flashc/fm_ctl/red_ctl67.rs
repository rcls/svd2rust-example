#[doc = "Register `RED_CTL67` reader"]
pub struct R(crate::R<RED_CTL67_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL67_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL67_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL67_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL67` writer"]
pub struct W(crate::W<RED_CTL67_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL67_SPEC>;
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
impl From<crate::W<RED_CTL67_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL67_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLIM_TRIM_HV_1` reader - '2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
pub struct VLIM_TRIM_HV_1_R(crate::FieldReader<bool, bool>);
impl VLIM_TRIM_HV_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLIM_TRIM_HV_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLIM_TRIM_HV_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLIM_TRIM_HV_1` writer - '2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
pub struct VLIM_TRIM_HV_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VLIM_TRIM_HV_1_W<'a> {
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
#[doc = "Field `DNU_67_1` reader - Not Used"]
pub struct DNU_67_1_R(crate::FieldReader<bool, bool>);
impl DNU_67_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_67_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_67_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_67_1` writer - Not Used"]
pub struct DNU_67_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_1_W<'a> {
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
#[doc = "Field `VPROT_ACT_HV` reader - Forces VPROT in active mode all the time"]
pub struct VPROT_ACT_HV_R(crate::FieldReader<bool, bool>);
impl VPROT_ACT_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        VPROT_ACT_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPROT_ACT_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPROT_ACT_HV` writer - Forces VPROT in active mode all the time"]
pub struct VPROT_ACT_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VPROT_ACT_HV_W<'a> {
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
#[doc = "Field `DNU_67_3` reader - Not Used"]
pub struct DNU_67_3_R(crate::FieldReader<bool, bool>);
impl DNU_67_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_67_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_67_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_67_3` writer - Not Used"]
pub struct DNU_67_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `IPREF_TC_HV` reader - Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
pub struct IPREF_TC_HV_R(crate::FieldReader<bool, bool>);
impl IPREF_TC_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPREF_TC_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPREF_TC_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPREF_TC_HV` writer - Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
pub struct IPREF_TC_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TC_HV_W<'a> {
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
#[doc = "Field `DNU_67_5` reader - Not Used"]
pub struct DNU_67_5_R(crate::FieldReader<bool, bool>);
impl DNU_67_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_67_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_67_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_67_5` writer - Not Used"]
pub struct DNU_67_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `IPREF_TRIMA_HI_HV` reader - Adds 200-300nA boost on IPREF_HI"]
pub struct IPREF_TRIMA_HI_HV_R(crate::FieldReader<bool, bool>);
impl IPREF_TRIMA_HI_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPREF_TRIMA_HI_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPREF_TRIMA_HI_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPREF_TRIMA_HI_HV` writer - Adds 200-300nA boost on IPREF_HI"]
pub struct IPREF_TRIMA_HI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIMA_HI_HV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DNU_67_7` reader - Not Used"]
pub struct DNU_67_7_R(crate::FieldReader<bool, bool>);
impl DNU_67_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DNU_67_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_67_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_67_7` writer - Not Used"]
pub struct DNU_67_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `IPREF_TRIMA_LO_HV` reader - Adds 200-300nA boost on IPREF_LO"]
pub struct IPREF_TRIMA_LO_HV_R(crate::FieldReader<bool, bool>);
impl IPREF_TRIMA_LO_HV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPREF_TRIMA_LO_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPREF_TRIMA_LO_HV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPREF_TRIMA_LO_HV` writer - Adds 200-300nA boost on IPREF_LO"]
pub struct IPREF_TRIMA_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIMA_LO_HV_W<'a> {
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
#[doc = "Field `DNU_67_23_16` reader - Not Used"]
pub struct DNU_67_23_16_R(crate::FieldReader<u8, u8>);
impl DNU_67_23_16_R {
    pub(crate) fn new(bits: u8) -> Self {
        DNU_67_23_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNU_67_23_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNU_67_23_16` writer - Not Used"]
pub struct DNU_67_23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - '2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_1(&self) -> VLIM_TRIM_HV_1_R {
        VLIM_TRIM_HV_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_1(&self) -> DNU_67_1_R {
        DNU_67_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn vprot_act_hv(&self) -> VPROT_ACT_HV_R {
        VPROT_ACT_HV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_3(&self) -> DNU_67_3_R {
        DNU_67_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
    #[inline(always)]
    pub fn ipref_tc_hv(&self) -> IPREF_TC_HV_R {
        IPREF_TC_HV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_5(&self) -> DNU_67_5_R {
        DNU_67_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Adds 200-300nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn ipref_trima_hi_hv(&self) -> IPREF_TRIMA_HI_HV_R {
        IPREF_TRIMA_HI_HV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_7(&self) -> DNU_67_7_R {
        DNU_67_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Adds 200-300nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&self) -> IPREF_TRIMA_LO_HV_R {
        IPREF_TRIMA_LO_HV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_23_16(&self) -> DNU_67_23_16_R {
        DNU_67_23_16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - '2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_1(&mut self) -> VLIM_TRIM_HV_1_W {
        VLIM_TRIM_HV_1_W { w: self }
    }
    #[doc = "Bit 1 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_1(&mut self) -> DNU_67_1_W {
        DNU_67_1_W { w: self }
    }
    #[doc = "Bit 2 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn vprot_act_hv(&mut self) -> VPROT_ACT_HV_W {
        VPROT_ACT_HV_W { w: self }
    }
    #[doc = "Bit 3 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_3(&mut self) -> DNU_67_3_W {
        DNU_67_3_W { w: self }
    }
    #[doc = "Bit 4 - Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
    #[inline(always)]
    pub fn ipref_tc_hv(&mut self) -> IPREF_TC_HV_W {
        IPREF_TC_HV_W { w: self }
    }
    #[doc = "Bit 5 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_5(&mut self) -> DNU_67_5_W {
        DNU_67_5_W { w: self }
    }
    #[doc = "Bit 6 - Adds 200-300nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn ipref_trima_hi_hv(&mut self) -> IPREF_TRIMA_HI_HV_W {
        IPREF_TRIMA_HI_HV_W { w: self }
    }
    #[doc = "Bit 7 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_7(&mut self) -> DNU_67_7_W {
        DNU_67_7_W { w: self }
    }
    #[doc = "Bit 8 - Adds 200-300nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&mut self) -> IPREF_TRIMA_LO_HV_W {
        IPREF_TRIMA_LO_HV_W { w: self }
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_23_16(&mut self) -> DNU_67_23_16_W {
        DNU_67_23_16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Controll normal sectors 6,7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl67](index.html) module"]
pub struct RED_CTL67_SPEC;
impl crate::RegisterSpec for RED_CTL67_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl67::R](R) reader structure"]
impl crate::Readable for RED_CTL67_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl67::W](W) writer structure"]
impl crate::Writable for RED_CTL67_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RED_CTL67 to value 0"]
impl crate::Resettable for RED_CTL67_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
