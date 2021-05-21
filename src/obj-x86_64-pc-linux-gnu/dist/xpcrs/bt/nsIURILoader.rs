//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIURILoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURILoader",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerContentListener (in nsIURIContentListener aContentListener); */
                    Method {
                        name: "RegisterContentListener",
                        params: &[Param { name: "aContentListener", ty: "*const nsIURIContentListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unRegisterContentListener (in nsIURIContentListener aContentListener); */
                    Method {
                        name: "UnRegisterContentListener",
                        params: &[Param { name: "aContentListener", ty: "*const nsIURIContentListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void openURI (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "OpenURI",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFlags", ty: "u32" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIStreamListener openChannel (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
                    Method {
                        name: "OpenChannel",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFlags", ty: "u32" }, Param { name: "aWindowContext", ty: "*const nsIInterfaceRequestor" }, Param { name: "_retval", ty: "*mut*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stop (in nsISupports aLoadCookie); */
                    Method {
                        name: "Stop",
                        params: &[Param { name: "aLoadCookie", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

