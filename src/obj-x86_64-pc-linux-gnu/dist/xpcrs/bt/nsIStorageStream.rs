//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIStorageStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStorageStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in uint32_t segmentSize, in uint32_t maxSize); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "segmentSize", ty: "uint32_t" }, Param { name: "maxSize", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOutputStream getOutputStream (in int32_t startPosition); */
                    Method {
                        name: "GetOutputStream",
                        params: &[Param { name: "startPosition", ty: "int32_t" }, Param { name: "_retval", ty: "*mut*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream newInputStream (in int32_t startPosition); */
                    Method {
                        name: "NewInputStream",
                        params: &[Param { name: "startPosition", ty: "int32_t" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute uint32_t length; */
                    Method {
                        name: "GetLength",
                        params: &[Param { name: "aLength", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLength",
                        params: &[Param { name: "aLength", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean writeInProgress; */
                    Method {
                        name: "GetWriteInProgress",
                        params: &[Param { name: "aWriteInProgress", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

