//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIncrementalDownload.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIncrementalDownload",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "destination", ty: "*const nsIFile" }, Param { name: "chunkSize", ty: "i32" }, Param { name: "intervalInSeconds", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "aURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI finalURI; */
                    Method {
                        name: "GetFinalURI",
                        params: &[Param { name: "aFinalURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile destination; */
                    Method {
                        name: "GetDestination",
                        params: &[Param { name: "aDestination", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long totalSize; */
                    Method {
                        name: "GetTotalSize",
                        params: &[Param { name: "aTotalSize", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long currentSize; */
                    Method {
                        name: "GetCurrentSize",
                        params: &[Param { name: "aCurrentSize", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void start (in nsIRequestObserver observer, in nsISupports ctxt); */
                    Method {
                        name: "Start",
                        params: &[Param { name: "observer", ty: "*const nsIRequestObserver" }, Param { name: "ctxt", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

