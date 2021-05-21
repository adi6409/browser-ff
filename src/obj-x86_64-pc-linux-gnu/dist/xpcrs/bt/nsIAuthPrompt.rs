//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPrompt",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean prompt (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, in wstring defaultText, out wstring result); */
                    Method {
                        name: "Prompt",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "passwordRealm", ty: "*const i16" }, Param { name: "savePassword", ty: "uint32_t" }, Param { name: "defaultText", ty: "*const i16" }, Param { name: "result", ty: "*mut *const i16" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring user, inout wstring pwd); */
                    Method {
                        name: "PromptUsernameAndPassword",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "passwordRealm", ty: "*const i16" }, Param { name: "savePassword", ty: "uint32_t" }, Param { name: "user", ty: "*mut *const i16" }, Param { name: "pwd", ty: "*mut *const i16" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean promptPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring pwd); */
                    Method {
                        name: "PromptPassword",
                        params: &[Param { name: "dialogTitle", ty: "*const i16" }, Param { name: "text", ty: "*const i16" }, Param { name: "passwordRealm", ty: "*const i16" }, Param { name: "savePassword", ty: "uint32_t" }, Param { name: "pwd", ty: "*mut *const i16" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

