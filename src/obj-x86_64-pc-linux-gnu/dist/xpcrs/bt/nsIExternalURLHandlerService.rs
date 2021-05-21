//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalURLHandlerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalURLHandlerService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIHandlerInfo getURLHandlerInfoFromOS (in nsIURI aURL, out boolean aFound); */
                    Method {
                        name: "GetURLHandlerInfoFromOS",
                        params: &[Param { name: "aURL", ty: "*const nsIURI" }, Param { name: "aFound", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut *const nsIHandlerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

