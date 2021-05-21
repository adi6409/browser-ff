//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDevicePrompt.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationDeviceRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString origin; */
                    Method {
                        name: "GetOrigin",
                        params: &[Param { name: "aOrigin", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray requestURLs; */
                    Method {
                        name: "GetRequestURLs",
                        params: &[Param { name: "aRequestURLs", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute EventTarget chromeEventHandler; */
                    Method {
                        name: "GetChromeEventHandler",
                        params: &[Param { name: "aChromeEventHandler", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void select (in nsIPresentationDevice device); */
                    Method {
                        name: "Select",
                        params: &[Param { name: "device", ty: "*const nsIPresentationDevice" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancel (in nsresult reason); */
                    Method {
                        name: "Cancel",
                        params: &[Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationDevicePrompt",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void promptDeviceSelection (in nsIPresentationDeviceRequest request); */
                    Method {
                        name: "PromptDeviceSelection",
                        params: &[Param { name: "request", ty: "*const nsIPresentationDeviceRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

