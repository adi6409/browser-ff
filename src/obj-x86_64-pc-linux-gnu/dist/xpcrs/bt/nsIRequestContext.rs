//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestContext.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequestTailUnblockCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onTailUnblock (in nsresult aResult); */
                    Method {
                        name: "OnTailUnblock",
                        params: &[Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIRequestContext",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsIRequestContextService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIRequestContext getRequestContext (in unsigned long long id); */
                    Method {
                        name: "GetRequestContext",
                        params: &[Param { name: "id", ty: "u64" }, Param { name: "_retval", ty: "*mut *const nsIRequestContext" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIRequestContext getRequestContextFromLoadGroup (in nsILoadGroup lg); */
                    Method {
                        name: "GetRequestContextFromLoadGroup",
                        params: &[Param { name: "lg", ty: "*const nsILoadGroup" }, Param { name: "_retval", ty: "*mut *const nsIRequestContext" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIRequestContext newRequestContext (); */
                    Method {
                        name: "NewRequestContext",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIRequestContext" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeRequestContext (in unsigned long long id); */
                    Method {
                        name: "RemoveRequestContext",
                        params: &[Param { name: "id", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

