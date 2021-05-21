//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsPIPromptService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPIPromptService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void doDialog (in nsIDOMWindow aParent, in nsIDialogParamBlock aParamBlock, in string aChromeURL); */
                    Method {
                        name: "DoDialog",
                        params: &[Param { name: "aParent", ty: "*const nsIDOMWindow" }, Param { name: "aParamBlock", ty: "*const nsIDialogParamBlock" }, Param { name: "aChromeURL", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

