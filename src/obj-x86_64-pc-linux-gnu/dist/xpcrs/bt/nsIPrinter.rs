//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPrinter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrinterInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Array<nsIPaper> paperList; */
                    Method {
                        name: "GetPaperList",
                        params: &[Param { name: "aPaperList", ty: "*mut thin_vec::ThinVec<RefPtr<nsIPaper>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrintSettings defaultSettings; */
                    Method {
                        name: "GetDefaultSettings",
                        params: &[Param { name: "aDefaultSettings", ty: "*mut *const nsIPrintSettings" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPrinter",
            base: Some("nsISupports"),
            methods: Err("specialtype promise unsupported"),
        },

        ]; D}

