//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/about/nsIAboutModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAboutModule",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
                    Method {
                        name: "NewChannel",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long getURIFlags (in nsIURI aURI); */
                    Method {
                        name: "GetURIFlags",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURI getChromeURI (in nsIURI aURI); */
                    Method {
                        name: "GetChromeURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

