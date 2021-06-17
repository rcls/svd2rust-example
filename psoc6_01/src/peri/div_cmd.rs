#[doc = "Register `DIV_CMD` reader"]
pub struct R(crate::R<DIV_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_CMD` writer"]
pub struct W(crate::W<DIV_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_CMD_SPEC>;
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
impl From<crate::W<DIV_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_SEL` reader - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
pub struct DIV_SEL_R(crate::FieldReader<u8, u8>);
impl DIV_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_SEL` writer - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
pub struct DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `TYPE_SEL` reader - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub struct TYPE_SEL_R(crate::FieldReader<u8, u8>);
impl TYPE_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPE_SEL` writer - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub struct TYPE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PA_DIV_SEL` reader - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '63' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
pub struct PA_DIV_SEL_R(crate::FieldReader<u8, u8>);
impl PA_DIV_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_DIV_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_DIV_SEL` writer - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '63' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
pub struct PA_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `PA_TYPE_SEL` reader - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub struct PA_TYPE_SEL_R(crate::FieldReader<u8, u8>);
impl PA_TYPE_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_TYPE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_TYPE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_TYPE_SEL` writer - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub struct PA_TYPE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_TYPE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `DISABLE` reader - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
pub struct DISABLE_R(crate::FieldReader<bool, bool>);
impl DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE` writer - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub fn div_sel(&self) -> DIV_SEL_R {
        DIV_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn type_sel(&self) -> TYPE_SEL_R {
        TYPE_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '63' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub fn pa_div_sel(&self) -> PA_DIV_SEL_R {
        PA_DIV_SEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn pa_type_sel(&self) -> PA_TYPE_SEL_R {
        PA_TYPE_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - (TYPE_SEL, DIV_SEL) specifies the divider on which the command (DISABLE/ENABLE) is performed. If DIV_SEL is '63' and TYPE_SEL is '3' (default/reset value), no divider is specified and no clock signal(s) are generated."]
    #[inline(always)]
    pub fn div_sel(&mut self) -> DIV_SEL_W {
        DIV_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Specifies the divider type of the divider on which the command is performed: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn type_sel(&mut self) -> TYPE_SEL_W {
        TYPE_SEL_W { w: self }
    }
    #[doc = "Bits 8:13 - (PA_TYPE_SEL, PA_DIV_SEL) specifies the divider to which phase alignment is performed for the clock enable command. Any enabled divider can be used as reference. This allows all dividers to be aligned with each other, even when they are enabled at different times. If PA_DIV_SEL is '63' and PA_TYPE_SEL is '3', 'clk_peri' is used as reference."]
    #[inline(always)]
    pub fn pa_div_sel(&mut self) -> PA_DIV_SEL_W {
        PA_DIV_SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Specifies the divider type of the divider to which phase alignment is performed for the clock enable command: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn pa_type_sel(&mut self) -> PA_TYPE_SEL_W {
        PA_TYPE_SEL_W { w: self }
    }
    #[doc = "Bit 30 - Clock divider disable command (mutually exclusive with ENABLE). SW sets this field to '1' and HW sets this field to '0'. The DIV_SEL and TYPE_SEL fields specify which divider is to be disabled. The HW sets the DISABLE field to '0' immediately and the HW sets the DIV_XXX_CTL.EN field of the divider to '0' immediately."]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
    #[doc = "Bit 31 - Clock divider enable command (mutually exclusive with DISABLE). Typically, SW sets this field to '1' to enable a divider and HW sets this field to '0' to indicate that divider enabling has completed. When a divider is enabled, its integer and fractional (if present) counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the DIV_CMD.DISABLE field. 1: Configure the divider's DIV_XXX_CTL register. 2: Enable the divider using the DIV_CMD_ENABLE field. The DIV_SEL and TYPE_SEL fields specify which divider is to be enabled. The enabled divider may be phase aligned to either 'clk_peri' (typical usage) or to ANY enabled divider. The PA_DIV_SEL and PA_TYPE_SEL fields specify the reference divider. The HW sets the ENABLE field to '0' when the enabling is performed and the HW set the DIV_XXX_CTL.EN field of the divider to '1' when the enabling is performed. Note that enabling with phase alignment to a low frequency divider takes time. E.g. To align to a divider that generates a clock of 'clk_peri'/n (with n being the integer divider value INT_DIV+1), up to n cycles may be required to perform alignment. Phase alignment to 'clk_peri' takes affect immediately. SW can set this field to '0' during phase alignment to abort the enabling process."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divider command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_cmd](index.html) module"]
pub struct DIV_CMD_SPEC;
impl crate::RegisterSpec for DIV_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_cmd::R](R) reader structure"]
impl crate::Readable for DIV_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_cmd::W](W) writer structure"]
impl crate::Writable for DIV_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_CMD to value 0xffff"]
impl crate::Resettable for DIV_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
