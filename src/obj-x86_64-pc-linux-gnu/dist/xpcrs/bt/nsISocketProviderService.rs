//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISocketProviderService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISocketProviderService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISocketProvider getSocketProvider (in string socketType); */
                    Method {
                        name: "GetSocketProvider",
                        params: &[Param { name: "socketType", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut*const nsISocketProvider" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

