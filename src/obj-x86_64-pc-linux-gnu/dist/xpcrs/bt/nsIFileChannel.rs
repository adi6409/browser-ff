//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/file/nsIFileChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFileChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIFile file; */
                    Method {
                        name: "GetFile",
                        params: &[Param { name: "aFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

