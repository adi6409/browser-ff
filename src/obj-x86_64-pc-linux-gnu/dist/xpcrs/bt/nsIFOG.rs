//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/glean/xpcom/nsIFOG.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFOG",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void initializeFOG (); */
                    Method {
                        name: "InitializeFOG",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setLogPings (in boolean aEnableLogPings); */
                    Method {
                        name: "SetLogPings",
                        params: &[Param { name: "aEnableLogPings", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTagPings (in ACString aDebugTag); */
                    Method {
                        name: "SetTagPings",
                        params: &[Param { name: "aDebugTag", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sendPing (in ACString aPingName); */
                    Method {
                        name: "SendPing",
                        params: &[Param { name: "aPingName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

