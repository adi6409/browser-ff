//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIDirIndexListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirIndexListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex); */
                    Method {
                        name: "OnIndexAvailable",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCtxt", ty: "*const nsISupports" }, Param { name: "aIndex", ty: "*const nsIDirIndex" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo); */
                    Method {
                        name: "OnInformationAvailable",
                        params: &[Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCtxt", ty: "*const nsISupports" }, Param { name: "aInfo", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDirIndexParser",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* attribute nsIDirIndexListener listener; */
                    Method {
                        name: "GetListener",
                        params: &[Param { name: "aListener", ty: "*mut *const nsIDirIndexListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetListener",
                        params: &[Param { name: "aListener", ty: "*const nsIDirIndexListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string comment; */
                    Method {
                        name: "GetComment",
                        params: &[Param { name: "aComment", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute string encoding; */
                    Method {
                        name: "GetEncoding",
                        params: &[Param { name: "aEncoding", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEncoding",
                        params: &[Param { name: "aEncoding", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

