//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/nsIBrowserHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBrowserHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AUTF8String startPage; */
                    Method {
                        name: "GetStartPage",
                        params: &[Param { name: "aStartPage", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetStartPage",
                        params: &[Param { name: "aStartPage", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String defaultArgs; */
                    Method {
                        name: "GetDefaultArgs",
                        params: &[Param { name: "aDefaultArgs", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultArgs",
                        params: &[Param { name: "aDefaultArgs", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean kiosk; */
                    Method {
                        name: "GetKiosk",
                        params: &[Param { name: "aKiosk", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetKiosk",
                        params: &[Param { name: "aKiosk", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getFeatures (in nsICommandLine aCmdLine); */
                    Method {
                        name: "GetFeatures",
                        params: &[Param { name: "aCmdLine", ty: "*const nsICommandLine" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

