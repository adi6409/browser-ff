//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICookieTransactionCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void callback (); */
                    Method {
                        name: "Callback",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICookieService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getCookieStringFromDocument (in Document aDocument); */
                    Method {
                        name: "GetCookieStringFromDocument",
                        params: &[Param { name: "aDocument", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getCookieStringFromHttp (in nsIURI aURI, in nsIChannel aChannel); */
                    Method {
                        name: "GetCookieStringFromHttp",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCookieStringFromDocument (in Document aDocument, in ACString aCookie); */
                    Method {
                        name: "SetCookieStringFromDocument",
                        params: &[Param { name: "aDocument", ty: "*const libc::c_void" }, Param { name: "aCookie", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCookieStringFromHttp (in nsIURI aURI, in ACString aCookie, in nsIChannel aChannel); */
                    Method {
                        name: "SetCookieStringFromHttp",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aCookie", ty: "*const ::nsstring::nsACString" }, Param { name: "aChannel", ty: "*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void runInTransaction (in nsICookieTransactionCallback aCallback); */
                    Method {
                        name: "RunInTransaction",
                        params: &[Param { name: "aCallback", ty: "*const nsICookieTransactionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

