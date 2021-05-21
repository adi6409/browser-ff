//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIInterfaceRequestor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInterfaceRequestor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "GetInterface",
                        params: &[Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

