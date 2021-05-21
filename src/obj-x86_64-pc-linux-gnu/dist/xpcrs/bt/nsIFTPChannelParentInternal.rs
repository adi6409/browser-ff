//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/ftp/nsIFTPChannelParentInternal.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFTPChannelParentInternal",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setErrorMsg (in string msg, in boolean useUTF8); */
                    Method {
                        name: "SetErrorMsg",
                        params: &[Param { name: "msg", ty: "*const libc::c_char" }, Param { name: "useUTF8", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

