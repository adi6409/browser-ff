//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIGfxInfoDebug.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGfxInfoDebug",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void spoofVendorID (in AString aVendorID); */
                    Method {
                        name: "SpoofVendorID",
                        params: &[Param { name: "aVendorID", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void spoofDeviceID (in AString aDeviceID); */
                    Method {
                        name: "SpoofDeviceID",
                        params: &[Param { name: "aDeviceID", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void spoofDriverVersion (in AString aDriverVersion); */
                    Method {
                        name: "SpoofDriverVersion",
                        params: &[Param { name: "aDriverVersion", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void spoofOSVersion (in unsigned long aVersion); */
                    Method {
                        name: "SpoofOSVersion",
                        params: &[Param { name: "aVersion", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void fireTestProcess (); */
                    Method {
                        name: "FireTestProcess",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

