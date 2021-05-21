//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrompt",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void alert (in wstring dialogTitle, in wstring text); */
                    Method {
                        name: "Alert",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void alertCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
                    Method {
                        name: "AlertCheck",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "checkMsg", ty: "*const i16" }, Param { name: "checkValue", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean confirm (in wstring dialogTitle, in wstring text); */
                    Method {
                        name: "Confirm",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean confirmCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
                    Method {
                        name: "ConfirmCheck",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "checkMsg", ty: "*const i16" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* int32_t confirmEx (in wstring dialogTitle, in wstring text, in unsigned long buttonFlags, in wstring button0Title, in wstring button1Title, in wstring button2Title, in wstring checkMsg, inout boolean checkValue); */
                    Method {
                        name: "ConfirmEx",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "buttonFlags", ty: "u32" }, Param { name: "button0Title", ty: "*const i16" }, Param { name: "button1Title", ty: "*const i16" }, Param { name: "button2Title", ty: "*const i16" }, Param { name: "checkMsg", ty: "*const i16" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean prompt (in wstring dialogTitle, in wstring text, inout wstring value, in wstring checkMsg, inout boolean checkValue); */
                    Method {
                        name: "Prompt",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "value", ty: "*mut *const i16" }, Param { name: "checkMsg", ty: "*const i16" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean promptPassword (in wstring dialogTitle, in wstring text, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
                    Method {
                        name: "PromptPassword",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "password", ty: "*mut *const i16" }, Param { name: "checkMsg", ty: "*const i16" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, inout wstring username, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
                    Method {
                        name: "PromptUsernameAndPassword",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "username", ty: "*mut *const i16" }, Param { name: "password", ty: "*mut *const i16" }, Param { name: "checkMsg", ty: "*const i16" }, Param { name: "checkValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean select (in wstring dialogTitle, in wstring text, in Array<AString> selectList, out long outSelection); */
                    Method {
                        name: "Select",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "selectList", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }, Param { name: "outSelection", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

