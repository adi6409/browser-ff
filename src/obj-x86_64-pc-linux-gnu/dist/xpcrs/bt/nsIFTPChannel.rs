//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/ftp/nsIFTPChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFTPChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute PRTime lastModifiedTime; */
                    Method {
                        name: "GetLastModifiedTime",
                        params: &[Param { name: "aLastModifiedTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLastModifiedTime",
                        params: &[Param { name: "aLastModifiedTime", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFTPEventSink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void OnFTPControlLog (in boolean server, in string msg); */
                    Method {
                        name: "OnFTPControlLog",
                        params: &[Param { name: "server", ty: "bool" }, Param { name: "msg", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

