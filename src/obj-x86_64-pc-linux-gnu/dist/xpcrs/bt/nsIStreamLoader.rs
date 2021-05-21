//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamLoaderObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onStreamComplete (in nsIStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
                    Method {
                        name: "OnStreamComplete",
                        params: &[Param { name: "loader", ty: "*const nsIStreamLoader" }, Param { name: "ctxt", ty: "*const nsISupports" }, Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "resultLength", ty: "u32" }, Param { name: "result", ty: "*const u8" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIStreamLoader",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aStreamObserver", ty: "*const nsIStreamLoaderObserver" }, Param { name: "aRequestObserver", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long numBytesRead; */
                    Method {
                        name: "GetNumBytesRead",
                        params: &[Param { name: "aNumBytesRead", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIRequest request; */
                    Method {
                        name: "GetRequest",
                        params: &[Param { name: "aRequest", ty: "*mut*const nsIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

