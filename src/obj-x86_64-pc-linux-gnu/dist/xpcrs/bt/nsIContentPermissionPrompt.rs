//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentPermissionPrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPermissionType",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray options; */
                    Method {
                        name: "GetOptions",
                        params: &[Param { name: "aOptions", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIContentPermissionRequest",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIContentPermissionPrompt",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void prompt (in nsIContentPermissionRequest request); */
                    Method {
                        name: "Prompt",
                        params: &[Param { name: "request", ty: "*const nsIContentPermissionRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

