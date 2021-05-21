//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIOpenWindowInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBrowsingContextReadyCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void browsingContextReady (in BrowsingContext bc); */
                    Method {
                        name: "BrowsingContextReady",
                        params: &[Param { name: "bc", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIOpenWindowInfo",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        ]; D}

