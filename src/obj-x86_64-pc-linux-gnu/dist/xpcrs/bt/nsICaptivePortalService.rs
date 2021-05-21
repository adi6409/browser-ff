//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICaptivePortalService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICaptivePortalServiceCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in bool success, in nsresult error); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "success", ty: "bool" }, Param { name: "error", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICaptivePortalService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void recheckCaptivePortal (); */
                    Method {
                        name: "RecheckCaptivePortal",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long lastChecked; */
                    Method {
                        name: "GetLastChecked",
                        params: &[Param { name: "aLastChecked", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

