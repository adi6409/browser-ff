//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFileStreams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileInputStream",
            base: Some("nsIInputStream"),
            methods: Ok(&[
                    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "ioFlags", ty: "i32" }, Param { name: "perm", ty: "i32" }, Param { name: "behaviorFlags", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFileOutputStream",
            base: Some("nsIOutputStream"),
            methods: Ok(&[
                    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "ioFlags", ty: "i32" }, Param { name: "perm", ty: "i32" }, Param { name: "behaviorFlags", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void preallocate (in long long length); */
                    Method {
                        name: "Preallocate",
                        params: &[Param { name: "length", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFileStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "file", ty: "*const nsIFile" }, Param { name: "ioFlags", ty: "i32" }, Param { name: "perm", ty: "i32" }, Param { name: "behaviorFlags", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFileMetadata",
            base: Some("nsISupports"),
            methods: Err("native type PRFileDesc unsupported"),
        },

        Interface {
            name: "nsIAsyncFileMetadata",
            base: Some("nsIFileMetadata"),
            methods: Ok(&[
                    /* void asyncFileMetadataWait (in nsIFileMetadataCallback aCallback, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "AsyncFileMetadataWait",
                        params: &[Param { name: "aCallback", ty: "*const nsIFileMetadataCallback" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFileMetadataCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onFileMetadataReady (in nsIAsyncFileMetadata aObject); */
                    Method {
                        name: "OnFileMetadataReady",
                        params: &[Param { name: "aObject", ty: "*const nsIAsyncFileMetadata" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

