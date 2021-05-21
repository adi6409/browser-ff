//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIBufferedStreams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIBufferedInputStream",
            base: Some("nsIInputStream"),
            methods: Ok(&[
                    /* void init (in nsIInputStream fillFromStream, in unsigned long bufferSize); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "fillFromStream", ty: "*const nsIInputStream" }, Param { name: "bufferSize", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIInputStream data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIBufferedOutputStream",
            base: Some("nsIOutputStream"),
            methods: Ok(&[
                    /* void init (in nsIOutputStream sinkToStream, in unsigned long bufferSize); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "sinkToStream", ty: "*const nsIOutputStream" }, Param { name: "bufferSize", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIOutputStream data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

