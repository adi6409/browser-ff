//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/converters/nsICompressConvStats.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICompressConvStats",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint64_t decodedDataLength; */
                    Method {
                        name: "GetDecodedDataLength",
                        params: &[Param { name: "aDecodedDataLength", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

