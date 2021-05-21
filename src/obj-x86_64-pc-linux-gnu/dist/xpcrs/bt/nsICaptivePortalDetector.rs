//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/captivedetect/nsICaptivePortalDetector.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICaptivePortalCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void prepare (); */
                    Method {
                        name: "Prepare",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void complete (in bool success); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "success", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICaptivePortalDetector",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void checkCaptivePortal (in AString ifname, in nsICaptivePortalCallback callback); */
                    Method {
                        name: "CheckCaptivePortal",
                        params: &[Param { name: "ifname", ty: "*const ::nsstring::nsAString" }, Param { name: "callback", ty: "*const nsICaptivePortalCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void abort (in AString ifname); */
                    Method {
                        name: "Abort",
                        params: &[Param { name: "ifname", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelLogin (in AString eventId); */
                    Method {
                        name: "CancelLogin",
                        params: &[Param { name: "eventId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void finishPreparation (in AString ifname); */
                    Method {
                        name: "FinishPreparation",
                        params: &[Param { name: "ifname", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

